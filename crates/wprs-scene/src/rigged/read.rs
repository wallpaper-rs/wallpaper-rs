use super::doc::{Accessor, Gltf};
use serde::Deserialize;

#[derive(Deserialize, Default)]
#[serde(default)]
pub(crate) struct Sparse {
    pub count: usize,
    pub indices: SparsePart,
    pub values: SparsePart,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub(crate) struct SparsePart {
    #[serde(rename = "bufferView")]
    pub view: usize,
    #[serde(rename = "byteOffset")]
    pub offset: usize,
    #[serde(rename = "componentType")]
    pub component: u32,
}

pub(crate) struct Bin<'a> {
    pub doc: &'a Gltf,
    pub bin: &'a [u8],
}

pub(crate) fn chunks(bytes: &[u8]) -> Result<(&[u8], &[u8]), String> {
    let word = |at: usize| bytes.get(at..at + 4).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]));
    let json = word(12).ok_or("has a cut header")? as usize;
    let text = bytes.get(20..20 + json).ok_or("has a cut JSON chunk")?;
    let at = 20 + json;
    let bin = match (word(at), word(at + 4)) {
        (Some(len), Some(0x004E_4942)) => {
            bytes.get(at + 8..at + 8 + len as usize).ok_or("has a cut BIN chunk")?
        }
        _ => &[],
    };
    Ok((text, bin))
}

fn width(kind: &str) -> Option<usize> {
    Some(match kind {
        "SCALAR" => 1,
        "VEC2" => 2,
        "VEC3" => 3,
        "VEC4" => 4,
        "MAT4" => 16,
        _ => return None,
    })
}

fn size(component: u32) -> Option<usize> {
    Some(match component {
        5120 | 5121 => 1,
        5122 | 5123 => 2,
        5125 | 5126 => 4,
        _ => return None,
    })
}

impl Bin<'_> {
    fn accessor(&self, i: usize) -> Result<&Accessor, String> {
        self.doc.accessors.get(i).ok_or_else(|| format!("names the missing accessor {i}"))
    }

    pub fn count(&self, i: usize) -> Result<usize, String> {
        Ok(self.accessor(i)?.count)
    }

    pub fn view(&self, i: usize) -> Result<&[u8], String> {
        let v = self.doc.views.get(i).ok_or_else(|| format!("names the missing buffer view {i}"))?;
        let end = v.offset.checked_add(v.length).filter(|_| v.buffer == 0);
        end.and_then(|e| self.bin.get(v.offset..e))
            .ok_or_else(|| format!("buffer view {i} is out of the file"))
    }

    pub fn floats(&self, i: usize, wanted: usize) -> Result<Vec<f32>, String> {
        let a = self.accessor(i)?;
        let (n, c) = (width(&a.kind), size(a.component));
        let (Some(n), Some(c)) = (n, c) else { return Err(format!("accessor {i} has an unknown type")) };
        if n != wanted {
            return Err(format!("accessor {i} holds {} values per element, expected {wanted}", n));
        }
        if a.count.saturating_mul(n) > 1 << 24 {
            return Err(format!("accessor {i} holds too many values"));
        }
        let Some(view) = a.view else {
            let mut out = vec![0.0; a.count * n];
            self.scatter(i, (n, c), &mut out)?;
            return Ok(out);
        };
        let data = self.view(view)?;
        let stride = self.doc.views[view].stride.unwrap_or(n * c).max(n * c);
        let last = a.count.checked_sub(1).map_or(Some(0), |k| k.checked_mul(stride)?.checked_add(n * c));
        let fits = last.and_then(|l| l.checked_add(a.offset)).is_some_and(|end| end <= data.len());
        if !fits {
            return Err(format!("accessor {i} reads past its buffer view"));
        }
        let mut out = Vec::with_capacity(a.count * n);
        for k in 0..a.count {
            for e in 0..n {
                let at = a.offset + k * stride + e * c;
                out.push(component(&data[at..at + c], a.component, a.normalized));
            }
        }
        self.scatter(i, (n, c), &mut out)?;
        if out.iter().any(|v| !v.is_finite()) {
            return Err(format!("accessor {i} holds a value that is not a number"));
        }
        Ok(out)
    }

    fn scatter(&self, i: usize, (n, c): (usize, usize), out: &mut [f32]) -> Result<(), String> {
        let a = &self.doc.accessors[i];
        let Some(s) = &a.sparse else { return Ok(()) };
        let wide = size(s.indices.component).filter(|_| [5121, 5123, 5125].contains(&s.indices.component));
        let wide = wide.ok_or_else(|| format!("accessor {i} has sparse indices of an unknown type"))?;
        let (keys, values) = (self.view(s.indices.view)?, self.view(s.values.view)?);
        let fits = |data: &[u8], at: usize, len: usize| {
            s.count.checked_mul(len).and_then(|l| l.checked_add(at)).is_some_and(|e| e <= data.len())
        };
        if !fits(keys, s.indices.offset, wide) || !fits(values, s.values.offset, n * c) {
            return Err(format!("accessor {i} has sparse values past their buffer views"));
        }
        for k in 0..s.count {
            let at = s.indices.offset + k * wide;
            let slot = component(&keys[at..at + wide], s.indices.component, false) as usize;
            let row = out
                .get_mut(slot * n..slot * n + n)
                .ok_or_else(|| format!("accessor {i} replaces an element it lacks"))?;
            for (e, v) in row.iter_mut().enumerate() {
                let at = s.values.offset + (k * n + e) * c;
                *v = component(&values[at..at + c], a.component, a.normalized);
            }
        }
        Ok(())
    }

    pub fn ints(&self, i: usize, wanted: usize) -> Result<Vec<u32>, String> {
        let a = self.accessor(i)?;
        if a.component == 5126 || a.normalized {
            return Err(format!("accessor {i} holds fractions where whole numbers were expected"));
        }
        Ok(self.floats(i, wanted)?.into_iter().map(|v| v as u32).collect())
    }
}

fn component(b: &[u8], kind: u32, normalized: bool) -> f32 {
    let (v, max) = match kind {
        5120 => (f32::from(b[0] as i8), 127.0),
        5121 => (f32::from(b[0]), 255.0),
        5122 => (f32::from(i16::from_le_bytes([b[0], b[1]])), 32767.0),
        5123 => (f32::from(u16::from_le_bytes([b[0], b[1]])), 65535.0),
        5125 => (u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as f32, 1.0),
        _ => (f32::from_le_bytes([b[0], b[1], b[2], b[3]]), 1.0),
    };
    if normalized && kind != 5126 { (v / max).max(-1.0) } else { v }
}
