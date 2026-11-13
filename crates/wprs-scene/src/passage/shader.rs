use crate::effects::{Descriptor, ParamKind, Stage};
use naga::valid::{Capabilities, ValidationFlags, Validator};

pub const MAX_TRANSITION_UNITS: f32 = 8.0;

const CROSSED: &str = concat!(
    include_str!("../crossed.wgsl"),
    include_str!("../oklab.wgsl"),
    include_str!("../noise.wgsl"),
    include_str!("../dither.wgsl")
);

fn source(d: &Descriptor) -> String {
    format!("{}\n{CROSSED}", d.wgsl)
}

fn placed(e: impl std::fmt::Display, at: Option<naga::SourceLocation>, lines: usize) -> String {
    match at.filter(|a| a.line_number as usize <= lines) {
        Some(a) => {
            format!(
                "it does not build as a transition shader: wgsl:{}:{}: {e}",
                a.line_number, a.line_position
            )
        }
        None => format!("it does not build as a transition shader: {e}"),
    }
}

pub(crate) fn check(d: &Descriptor) -> Result<(), String> {
    if d.stage != Stage::Transition {
        return Ok(());
    }
    let pictured = |p: &&crate::ParamDef| {
        matches!(p.kind, ParamKind::Texture | ParamKind::Lut | ParamKind::Mask | ParamKind::Emitters)
    };
    if let Some(p) = d.params.iter().find(pictured) {
        return Err(format!(
            "a transition takes no picture, lut, mask or emitters parameter such as {}",
            p.id
        ));
    }
    let source = source(d);
    let lines = d.wgsl.lines().count();
    let module =
        naga::front::wgsl::parse_str(&source).map_err(|e| placed(e.message(), e.location(&source), lines))?;
    let mut validator = Validator::new(ValidationFlags::all(), Capabilities::default());
    validator.validate(&module).map_err(|e| placed(&e, e.location(&source), lines))?;
    Ok(())
}

pub fn transition_shader(d: &Descriptor) -> Result<naga::Module, String> {
    if d.stage != Stage::Transition {
        return Err("it is not a transition shader".into());
    }
    if !crate::verdicts::passed(d.key) {
        return Err("its shader was not checked".into());
    }
    let source = source(d);
    crate::shader::roomy(|| {
        let mut module = crate::shader::compile(&source)?.0;
        crate::fence::guard(&mut module);
        let mut validator = Validator::new(ValidationFlags::all(), Capabilities::default());
        validator
            .validate(&module)
            .map_err(|e| format!("its guarded shader: {}", e.emit_to_string(&source)))?;
        Ok(module)
    })
}
