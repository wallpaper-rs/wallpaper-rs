use super::super::Reel;
use image::codecs::gif::{GifEncoder, Repeat};
use image::{Delay, Frame};
use std::path::{Path, PathBuf};

impl Reel {
    pub fn write(&self, out: &Path) -> std::io::Result<Vec<PathBuf>> {
        let gif = out.extension().and_then(|e| e.to_str()).is_some_and(|e| e.eq_ignore_ascii_case("gif"));
        if gif {
            let file = std::io::BufWriter::new(std::fs::File::create(out)?);
            let mut encoder = GifEncoder::new_with_speed(file, 10);
            encoder.set_repeat(Repeat::Infinite).map_err(std::io::Error::other)?;
            let delay = Delay::from_numer_denom_ms(1000, self.fps.max(1));
            let frames = self.frames.iter().map(|f| Frame::from_parts(f.clone(), 0, 0, delay));
            encoder.encode_frames(frames).map_err(std::io::Error::other)?;
            return Ok(vec![out.to_owned()]);
        }
        if let [only] = self.frames.as_slice() {
            crate::save(only, out, 95)?;
            return Ok(vec![out.to_owned()]);
        }
        let stem = out.file_stem().and_then(|s| s.to_str()).unwrap_or("frame");
        let ext = out.extension().and_then(|e| e.to_str()).unwrap_or("png");
        let mut written = Vec::new();
        for (i, frame) in self.frames.iter().enumerate() {
            let file = out.with_file_name(format!("{stem}-{:04}.{ext}", i + 1));
            crate::save(frame, &file, 95)?;
            written.push(file);
        }
        Ok(written)
    }
}
