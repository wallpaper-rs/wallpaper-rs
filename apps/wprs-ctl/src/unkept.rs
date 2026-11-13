use crate::told::{fill, told};
use serde_json::Value;
use wprs_editor::actions::{Action, ParticleEdit, SowEdit, TemplateEdit};
use wprs_editor::{Project, references};

pub(crate) fn named(project: &Project, actions: &[Action]) -> Vec<(String, String)> {
    actions
        .iter()
        .flat_map(|action| match action {
            Action::RenameProperty { id, .. }
            | Action::RemoveProperty { id }
            | Action::Template(TemplateEdit::Rename { name: id, .. } | TemplateEdit::Remove { name: id })
            | Action::Particles(ParticleEdit::Sow(SowEdit::Rename { system: id, .. })) => {
                references::scripts(&project.dir, &project.doc, id)
                    .into_iter()
                    .map(|p| (p, id.clone()))
                    .collect()
            }
            Action::Rename(r) => references::scripts(&project.dir, &project.doc, &r.id)
                .into_iter()
                .map(|p| (p, r.id.clone()))
                .collect(),
            _ => Vec::new(),
        })
        .collect()
}

pub(crate) fn lost(doc: &Value, actions: &[Action]) -> Vec<String> {
    let mut all: Vec<String> = Vec::new();
    for action in actions {
        if let Action::RemoveLayer { id } = action {
            all.extend(wprs_editor::ids::unkept(doc, id));
        }
    }
    all.sort();
    all.dedup();
    all
}

pub(crate) fn warning(lost: &[String]) -> Option<String> {
    (!lost.is_empty()).then(|| {
        fill(
            told("warning: %1 followed the removed layer and a value of its place is bound to something other than keyframes, so it could not be kept where it was: check where it sits now."),
            &[&lost.join(", ")],
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn only_removed_layers_are_asked_about() {
        let doc = json!({ "scene": { "layers": [{ "id": "a", "type": "solid" }] } });
        assert!(lost(&doc, &[Action::RemoveLayer { id: "a".into() }]).is_empty());
        assert!(lost(&doc, &[Action::Undo {}]).is_empty());
        assert!(warning(&[]).is_none());
        assert!(warning(&["b".into(), "c".into()]).unwrap().contains("b, c followed"));
    }
}
