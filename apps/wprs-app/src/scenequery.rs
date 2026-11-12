use crate::scene::qobject::SceneEditor;
use cxx_qt_lib::QString;
use wprs_editor::references;

impl SceneEditor {
    pub fn value(&self, pointer: &QString) -> QString {
        let found = self.project.as_ref().and_then(|p| p.doc.pointer(&pointer.to_string()).cloned());
        QString::from(&found.unwrap_or_default().to_string())
    }

    pub fn condition(&self, pointer: &QString, text: &QString) -> QString {
        let doc = self.project.as_ref().map(|p| &p.doc);
        let found =
            doc.and_then(|d| wprs_editor::settings::condition(d, &pointer.to_string(), &text.to_string()));
        QString::from(&found.unwrap_or_default())
    }

    pub fn scripts(&self, id: &QString) -> QString {
        let found = self.project.as_ref().map(|p| references::scripts(&p.dir, &p.doc, &id.to_string()));
        QString::from(&serde_json::to_string(&found.unwrap_or_default()).unwrap_or_default())
    }
}
