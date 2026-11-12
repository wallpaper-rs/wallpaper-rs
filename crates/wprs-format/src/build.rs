use crate::validate::extension;
use crate::{
    Error, FileEntry, Kind, Limits, MANIFEST, Manifest, Result, SIGNATURE, manifest_comment, signs_comment,
    validate,
};
use sha2::{Digest, Sha256};
use std::fmt::Write as _;
use std::fs::{self, File};
use std::io::{self, BufWriter, Read, Write};
use std::path::Path;
use wprs_core::message::said;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, DateTime, ZipWriter};

const STORED: &[&str] = &[
    "png", "jpg", "jpeg", "webp", "avif", "gif", "ktx2", "webm", "mp4", "mkv", "ogg", "opus", "flac", "mp3",
    "glb",
];

pub fn build(source: impl AsRef<Path>, out: impl AsRef<Path>, limits: &Limits) -> Result<String> {
    let source = source.as_ref();
    crate::fill(source)?;
    current(source)?;
    pack(source, None, out.as_ref(), limits)
}

fn current(source: &Path) -> Result<()> {
    let signed = match fs::read_to_string(source.join(SIGNATURE)) {
        Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
        read => read?,
    };
    let json = fs::read(source.join(MANIFEST))?;
    let manifest = Manifest::from_json(&json)?;
    let comment = manifest_comment(&manifest.id, &manifest.version, &json);
    crate::ensure(signs_comment(&signed, &comment), || {
        crate::refuse(
            SIGNATURE,
            said(
                "format.stale_signature",
                "it signs another version of manifest.json; run pack sign again, or delete it to build unsigned",
                &[],
            ),
        )
    })
}

pub fn build_staged(
    source: impl AsRef<Path>,
    manifest: &[u8],
    out: impl AsRef<Path>,
    limits: &Limits,
) -> Result<String> {
    pack(source.as_ref(), Some(manifest), out.as_ref(), limits)
}

fn pack(source: &Path, staged: Option<&[u8]>, out: &Path, limits: &Limits) -> Result<String> {
    let bytes = match staged {
        Some(bytes) => bytes.to_vec(),
        None => fs::read(source.join(MANIFEST))?,
    };
    let manifest = crate::source::with_strings(source, Manifest::from_json(&bytes)?)?;
    let mut files = Vec::new();
    collect(source, "", &mut files)?;
    if staged.is_some() {
        files.retain(|f| f.path != crate::SIGNATURE);
        files.iter_mut().filter(|f| f.path == MANIFEST).for_each(|f| f.size = bytes.len() as u64);
    }
    files.sort_by(|a, b| (a.path != MANIFEST, &a.path).cmp(&(b.path != MANIFEST, &b.path)));
    validate(&manifest, &files, limits)?;
    let mut zip = ZipWriter::new(BufWriter::new(File::create(out)?));
    for file in &files {
        let options = SimpleFileOptions::default()
            .compression_method(method(&file.path))
            .last_modified_time(DateTime::default())
            .unix_permissions(0o644)
            .large_file(file.size >= u64::from(u32::MAX));
        zip.start_file(file.path.as_str(), options)?;
        match (staged, file.path == MANIFEST) {
            (Some(_), true) => zip.write_all(&bytes)?,
            _ => drop(io::copy(&mut File::open(source.join(&file.path))?.take(file.size), &mut zip)?),
        }
    }
    zip.finish()?.flush()?;
    if (manifest.kind == Kind::Scene || manifest.v2())
        && let Err(e) = crate::open(out, limits)
    {
        fs::remove_file(out)?;
        return Err(e);
    }
    sha256_file(out)
}

pub fn sha256(mut reader: impl Read) -> Result<String> {
    let mut hasher = Hasher(Sha256::new());
    io::copy(&mut reader, &mut hasher)?;
    Ok(hasher.0.finalize().iter().fold(String::with_capacity(64), |mut hex, byte| {
        let _ = write!(hex, "{byte:02x}");
        hex
    }))
}

pub fn sha256_file(path: impl AsRef<Path>) -> Result<String> {
    sha256(File::open(path)?)
}

struct Hasher(Sha256);

impl Write for Hasher {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.update(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub(crate) fn collect(root: &Path, prefix: &str, files: &mut Vec<FileEntry>) -> Result<()> {
    for item in fs::read_dir(root.join(prefix))? {
        let item = item?;
        let name =
            item.file_name().into_string().map_err(|n| Error::UnsafePath(n.to_string_lossy().into()))?;
        let path = if prefix.is_empty() { name } else { format!("{prefix}/{name}") };
        let kind = item.file_type()?;
        if kind.is_dir() {
            collect(root, &path, files)?;
            continue;
        }
        let size = if kind.is_file() { item.metadata()?.len() } else { 0 };
        files.push(FileEntry { path, size, compressed_size: size, symlink: !kind.is_file() });
    }
    Ok(())
}

fn method(path: &str) -> CompressionMethod {
    let stored = extension(path).is_some_and(|e| STORED.contains(&e.as_str()));
    if stored { CompressionMethod::Stored } else { CompressionMethod::Deflated }
}
