use crate::scene::qobject::SceneEditor;
use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;
use std::path::Path;
use std::pin::Pin;
use wprs_editor::{check, project};

impl SceneEditor {
    pub fn diverged(&self) -> bool {
        self.project.as_ref().is_some_and(|p| p.diverged())
    }

    pub fn settle_outside(mut self: Pin<&mut Self>, choice: &QString, folder: &QString) {
        let settled = {
            let mut this = self.as_mut().rust_mut();
            let Some(project) = this.project.as_mut() else { return };
            match choice.to_string().as_str() {
                "mine" => project.keep_mine(),
                "theirs" => project.take_theirs(),
                _ => project.save_elsewhere(Path::new(&folder.to_string())),
            }
            .map(|_| (check::check_files(&project.dir, &project.doc), project::stamp(&project.doc)))
        };
        match settled {
            Ok((files, stamp)) => {
                (self.as_mut().rust_mut().files, self.as_mut().rust_mut().persisted) = (files, stamp);
                self.as_mut().set_error(QString::default());
                if let Some(project) = self.project.as_ref() {
                    let folder = QString::from(project.dir.to_string_lossy().as_ref());
                    self.as_mut().set_folder(folder);
                }
                self.as_mut().refresh();
                self.send();
            }
            Err(e) => self.fail(e),
        }
    }

    pub fn follow_folder(mut self: Pin<&mut Self>) {
        let (stamp, followed, files) = {
            let mut this = self.as_mut().rust_mut();
            let Some(project) = this.project.as_mut() else { return };
            let stamp = project.disk_stamp();
            if stamp == this.seen {
                return;
            }
            let project = this.project.as_mut().expect("checked above");
            let followed = project.follow_disk().unwrap_or(false);
            (stamp, followed, check::check_files(&project.dir, &project.doc))
        };
        (self.as_mut().rust_mut().seen, self.as_mut().rust_mut().files) = (stamp, files);
        match followed {
            true => self.changed(),
            false => self.refresh(),
        }
    }
}
