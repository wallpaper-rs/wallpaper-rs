use std::path::PathBuf;
use wprs_editor::actions::{Action, Edit3d, ManifestEdit, ShotEdit};

fn absolute(path: &mut PathBuf) {
    if let Ok(whole) = std::fs::canonicalize(&*path) {
        *path = whole;
    }
}

fn each(path: &mut Option<PathBuf>) {
    path.iter_mut().for_each(absolute);
}

pub(crate) fn resolve(action: &mut Action) {
    match action {
        Action::AddLayer { file, .. } => each(file),
        Action::Import { file, .. } | Action::Thumbnail { file, .. } => absolute(file),
        Action::ImportEffect { descriptor, wgsl } => {
            absolute(descriptor);
            each(wgsl);
        }
        Action::ImportScript(script) if script.template.is_none() => absolute(&mut script.file),
        Action::ImportAssets(assets) => absolute(&mut assets.package),
        Action::Shot(ShotEdit::Still { base, .. } | ShotEdit::Recording { base, .. }) => each(base),
        Action::Manifest(ManifestEdit::AddFile { file, .. } | ManifestEdit::Media { file }) => absolute(file),
        Action::Scene3d(Edit3d::Material(material)) => material.files.values_mut().for_each(absolute),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn files_that_exist_are_made_absolute_and_the_rest_is_left() {
        let tmp = tempfile::tempdir().unwrap();
        let file = tmp.path().join("figure.glb");
        std::fs::write(&file, b"x").unwrap();
        let mut action = Action::Import { file: file.clone(), id: None, linear: false, atlas: None };
        resolve(&mut action);
        assert_eq!(
            action,
            Action::Import {
                file: std::fs::canonicalize(&file).unwrap(),
                id: None,
                linear: false,
                atlas: None
            }
        );
        let mut missing = Action::Thumbnail { file: "none.png".into(), animated: false };
        resolve(&mut missing);
        assert_eq!(missing, Action::Thumbnail { file: "none.png".into(), animated: false });
    }
}
