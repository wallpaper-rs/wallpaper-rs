use serde_json::{Value, json};
use std::path::Path;
use wprs_editor::actions::visible;

pub(crate) fn document(doc: &Value, dir: &Path, look: &Value) -> Value {
    if wprs_editor::pages::paged(doc) {
        wprs_editor::pages::preview_document(doc, look)
    } else if wprs_editor::reels::media(doc) {
        wprs_editor::reels::preview_document(dir, doc, look)
    } else {
        let mut scene = visible(doc);
        wprs_editor::swarms::pin_seed(&mut scene);
        wprs_editor::rigs::preview(&mut scene, look);
        wprs_editor::scripts::stamped(&mut scene, dir, doc);
        let mut document = json!({"manifest": doc["manifest"], "scene": scene});
        if let Some(staged) = wprs_editor::rigs::staged(look) {
            document["preview"] = staged;
        }
        wprs_editor::tried::stage(doc, look, &mut document);
        document
    }
}
