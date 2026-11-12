use crate::custom::CONTRACT;
use crate::effects::{Descriptor, FRAME_WGSL, ParamKind, Stage};
use naga::SourceLocation;
use naga::valid::{Capabilities, ValidationFlags, Validator};

pub const MAX_MATERIAL_IMAGES: usize = 4;
pub const MAX_MATERIAL_PARAMS: usize = 8;

const HEAD: &str = concat!(
    "@group(0) @binding(3) var<uniform> frame: Frame;\n",
    "fn param(i: u32) -> vec4<f32> {\n    return vec4<f32>(f32(i));\n}\n",
    "@fragment\nfn wprs_material_check(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {\n",
    "    let s = Surface(vec3<f32>(0.0), vec3<f32>(0.0, 0.0, 1.0), uv, vec4<f32>(1.0), vec3<f32>(0.0), 0.5, 0.0, 1.0);\n",
    "    return material(s).albedo;\n}\n",
);

pub fn material_images(d: &Descriptor) -> impl Iterator<Item = &str> {
    let pictures = d.params.iter().filter(|p| p.kind == ParamKind::Texture);
    pictures.map(|p| p.id.as_str()).take(MAX_MATERIAL_IMAGES)
}

fn placed(at: Option<SourceLocation>, why: &str, lines: usize) -> String {
    let before = CONTRACT.lines().count() + 1;
    match at.filter(|a| a.line_number as usize <= lines) {
        Some(a) => format!(
            "it does not build as a material shader: wgsl:{}:{}: {why}",
            a.line_number as usize + before,
            a.line_position
        ),
        None => format!("it does not build as a material shader: {why}"),
    }
}

pub(crate) fn check(d: &Descriptor) -> Result<(), String> {
    if d.stage != Stage::Material {
        return Ok(());
    }
    let unusable =
        |p: &&crate::ParamDef| matches!(p.kind, ParamKind::Lut | ParamKind::Mask | ParamKind::Emitters);
    if let Some(p) = d.params.iter().find(unusable) {
        return Err(format!("a material shader takes no lut, mask or emitters parameter such as {}", p.id));
    }
    if d.params.len() > MAX_MATERIAL_PARAMS {
        return Err(format!("a material shader has at most {MAX_MATERIAL_PARAMS} parameters"));
    }
    let pictures = d.params.iter().filter(|p| p.kind == ParamKind::Texture).count();
    if pictures > MAX_MATERIAL_IMAGES {
        return Err(format!("a material shader samples at most {MAX_MATERIAL_IMAGES} images"));
    }
    let source = format!("{}\n{FRAME_WGSL}{}{HEAD}", d.wgsl, crate::sprites::MATERIAL);
    let lines = d.wgsl.lines().count();
    let module =
        naga::front::wgsl::parse_str(&source).map_err(|e| placed(e.location(&source), e.message(), lines))?;
    let mut validator = Validator::new(ValidationFlags::all(), Capabilities::default());
    validator.validate(&module).map_err(|e| placed(e.location(&source), &e.to_string(), lines))?;
    Ok(())
}
