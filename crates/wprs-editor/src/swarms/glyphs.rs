use ab_glyph::{Font, FontArc, FontVec, GlyphId, PxScale, ScaleFont, point};
use image::{Rgba, RgbaImage};
use serde_json::Value;
use std::path::Path;
use std::sync::OnceLock;

const SCALE: f32 = 40.0;
const MAX_CHARS: usize = 256;

fn database() -> &'static fontdb::Database {
    static DB: OnceLock<fontdb::Database> = OnceLock::new();
    DB.get_or_init(|| {
        let mut db = fontdb::Database::new();
        db.load_system_fonts();
        let has =
            |db: &fontdb::Database, name: &str| db.faces().any(|f| f.families.iter().any(|(n, _)| n == name));
        let pick =
            |db: &fontdb::Database, names: &[&str]| names.iter().find(|n| has(db, n)).map(|n| n.to_string());
        if let Some(name) = pick(&db, &["DejaVu Sans", "Noto Sans", "Roboto", "Liberation Sans", "Cantarell"])
        {
            db.set_sans_serif_family(name);
        }
        if let Some(name) = pick(&db, &["DejaVu Serif", "Noto Serif", "Liberation Serif", "Droid Serif"]) {
            db.set_serif_family(name);
        }
        if let Some(name) =
            pick(&db, &["DejaVu Sans Mono", "Noto Sans Mono", "Liberation Mono", "Droid Sans Mono"])
        {
            db.set_monospace_family(name);
        }
        db
    })
}

fn system(name: &str) -> Option<FontArc> {
    let db = database();
    let family = match name {
        "" | "sans-serif" => fontdb::Family::SansSerif,
        "serif" => fontdb::Family::Serif,
        "monospace" => fontdb::Family::Monospace,
        name => fontdb::Family::Name(name),
    };
    let query = fontdb::Query { families: &[family, fontdb::Family::SansSerif], ..Default::default() };
    let id = db.query(&query).or_else(|| db.faces().next().map(|f| f.id))?;
    db.with_face_data(id, |data, index| FontVec::try_from_vec_and_index(data.to_vec(), index).ok())?
        .map(FontArc::new)
}

fn packaged(dir: &Path, doc: &Value, name: &str) -> Option<FontArc> {
    let path = doc["scene"]["assets"][name]["path"].as_str()?;
    let bytes = std::fs::read(dir.join(path)).ok()?;
    FontVec::try_from_vec_and_index(bytes, 0).ok().map(FontArc::new)
}

pub(super) fn raster(dir: &Path, doc: &Value, font: &str, text: &str) -> Option<RgbaImage> {
    let font = packaged(dir, doc, font).or_else(|| system(font))?;
    let scale = PxScale::from(SCALE);
    let scaled = font.as_scaled(scale);
    let chars: Vec<char> = text.chars().filter(|c| !c.is_control()).take(MAX_CHARS).collect();
    if chars.is_empty() {
        return None;
    }
    let (mut pen, mut last, mut places) = (0.0_f32, None::<GlyphId>, Vec::new());
    for c in &chars {
        let id = font.glyph_id(*c);
        pen += last.map_or(0.0, |l| scaled.kern(l, id));
        places.push((id, pen));
        pen += scaled.h_advance(id);
        last = Some(id);
    }
    let width = pen.ceil().max(1.0) as u32;
    let height = (scaled.ascent() - scaled.descent()).ceil().max(1.0) as u32;
    let mut image = RgbaImage::from_pixel(width, height, Rgba([255, 255, 255, 0]));
    for (id, x) in places {
        let glyph = id.with_scale_and_position(scale, point(x, scaled.ascent()));
        let Some(outline) = font.outline_glyph(glyph) else { continue };
        let bounds = outline.px_bounds();
        outline.draw(|gx, gy, coverage| {
            let (px, py) = (bounds.min.x as i32 + gx as i32, bounds.min.y as i32 + gy as i32);
            if px >= 0
                && py >= 0
                && let Some(p) = image.get_pixel_mut_checked(px as u32, py as u32)
            {
                p.0[3] = p.0[3].max((coverage.clamp(0.0, 1.0) * 255.0).round() as u8);
            }
        });
    }
    Some(image)
}
