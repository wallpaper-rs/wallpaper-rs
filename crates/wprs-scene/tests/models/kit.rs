use super::*;

pub fn split(glb: &[u8]) -> (Value, Vec<u8>) {
    let word = |at: usize| u32::from_le_bytes(glb[at..at + 4].try_into().unwrap()) as usize;
    let text = word(12);
    let doc = serde_json::from_slice(&glb[20..20 + text]).unwrap();
    (doc, glb[20 + text + 8..].to_vec())
}

pub fn pack(doc: &Value, bin: &[u8]) -> Vec<u8> {
    let mut text = doc.to_string().into_bytes();
    while !text.len().is_multiple_of(4) {
        text.push(b' ');
    }
    let total = 12 + 8 + text.len() + 8 + bin.len();
    let mut glb = Vec::with_capacity(total);
    for word in [0x4654_6C67, 2, total as u32, text.len() as u32, 0x4E4F_534A] {
        glb.extend_from_slice(&u32::to_le_bytes(word));
    }
    glb.extend_from_slice(&text);
    glb.extend_from_slice(&(bin.len() as u32).to_le_bytes());
    glb.extend_from_slice(&[0x42, 0x49, 0x4E, 0x00]);
    glb.extend_from_slice(bin);
    glb
}

pub fn header(side: u32) -> Vec<u8> {
    let mut png = vec![0x89, b'P', b'N', b'G', 13, 10, 26, 10, 0, 0, 0, 13, b'I', b'H', b'D', b'R'];
    png.extend(side.to_be_bytes());
    png.extend(side.to_be_bytes());
    png.extend([8, 6, 0, 0, 0]);
    png
}

pub fn painted(side: u32) -> (Value, Vec<u8>) {
    let mut m = ForgeMaterial::new("paint", [1.0; 4]);
    m.maps[0] = Some(header(side));
    split(&Forge::default().material(m).part("p", ForgeShape::Plane, [0.0; 3], 1.0, 0).glb())
}
