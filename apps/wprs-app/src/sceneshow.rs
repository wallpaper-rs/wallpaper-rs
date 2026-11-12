use crate::scene::qobject::SceneEditor;
use cxx_qt_lib::QString;
use std::pin::Pin;
use wprs_editor::outline;

impl SceneEditor {
    pub fn select(mut self: Pin<&mut Self>, pointer: &QString) {
        self.as_mut().set_selected(pointer.clone());
        self.show_fields();
    }

    pub fn show_fields(mut self: Pin<&mut Self>) {
        let fields = self.inspect(&self.selected.clone());
        self.as_mut().set_fields(fields);
    }

    pub fn refresh(mut self: Pin<&mut Self>) {
        self.as_mut().scoped();
        let Some(project) = self.project.as_ref() else { return };
        let view = outline::outline(project, self.time, &self.files).to_string();
        let dirty = project.dirty();
        self.as_mut().set_view(QString::from(&view));
        self.as_mut().set_dirty(dirty);
        self.show_fields();
    }
}
