use crate::check::pointer;
use crate::findings::{Problem, manifest_problem, problem};
use serde_json::Value;
use wprs_core::message::said;

pub(crate) fn about(doc: &Value, e: wprs_format::Error) -> Problem {
    let layer = match &e {
        wprs_format::Error::Invalid { field, .. } => {
            field.strip_prefix("scene.layers.").and_then(|f| f.strip_suffix(".text"))
        }
        _ => None,
    };
    match layer {
        Some(id) => {
            let at = pointer(doc, &format!("/layers/{id}/text"));
            let text =
                said("editor.text_default", "the text of %1 needs a text in the default language", &[&id]);
            problem(at, text)
        }
        None => manifest_problem(e),
    }
}
