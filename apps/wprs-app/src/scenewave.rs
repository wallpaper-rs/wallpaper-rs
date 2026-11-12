use crate::scene::qobject::SceneEditor;
use cxx_qt::Threading;
use cxx_qt_lib::QString;
use std::pin::Pin;

impl SceneEditor {
    pub fn wave(self: Pin<&mut Self>, asset: &QString, buckets: i32) {
        let Some(project) = self.project.as_ref() else { return };
        let (dir, doc) = (project.dir.clone(), project.doc.clone());
        let (name, qt) = (asset.to_string(), self.qt_thread());
        let buckets = buckets.clamp(1, 2048) as usize;
        std::thread::spawn(move || {
            let found = wprs_editor::sounds::wave(&dir, &doc, &name, buckets).to_string();
            qt.queue(move |editor| editor.waved(QString::from(&name), QString::from(&found))).ok();
        });
    }
}
