use ab_glyph::FontArc;
use wprs_scene::Layout;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Measured {
    pub size: [f32; 2],
    pub lines: usize,
    pub widest: f32,
}

pub fn measure(chain: &[FontArc], text: &str, size: f32, layout: &Layout) -> Option<Measured> {
    let plan = crate::text::plan(chain, text, size, 1.0, layout)?;
    Some(Measured {
        size: [plan.width as f32, plan.height as f32],
        lines: plan.lines(),
        widest: plan.widest(),
    })
}
