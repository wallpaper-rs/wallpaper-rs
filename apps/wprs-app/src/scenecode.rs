use crate::scene::qobject::SceneEditor;
use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;
use serde_json::{Value, json};
use std::pin::Pin;
use wprs_editor::{Project, inspect, references, scripts};

fn joined(value: &Value) -> QString {
    QString::from(&value.to_string())
}

impl SceneEditor {
    pub fn inspect(&self, pointer: &QString) -> QString {
        let fields = self.project.as_ref().map(|p| {
            inspect::fields_from(Some(&p.dir), &p.doc, &pointer.to_string(), self.time, &p.effects())
        });
        QString::from(&serde_json::to_string(&fields.unwrap_or_default()).unwrap_or_default())
    }

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

    pub fn code(&self, query: &QString, args: &QString) -> QString {
        let args: Value = serde_json::from_str(&args.to_string()).unwrap_or_default();
        let path = args["path"].as_str().unwrap_or_default();
        let Some(project) = self.project.as_ref() else { return QString::default() };
        let found = match query.to_string().as_str() {
            "list" => scripts::listing(&project.dir, &project.doc),
            "audit" => scripts::audit(&project.dir, &project.doc, path).unwrap_or_default(),
            "source" => json!(source(project, path)),
            "reference" => scripts::reference(),
            "changes" => json!(self.report),
            "templates" => json!(wprs_editor::templates::tree(&project.dir, &project.doc)),
            _ => Value::Null,
        };
        joined(&found)
    }

    pub fn watch(mut self: Pin<&mut Self>) {
        let Some(project) = self.project.as_ref() else { return };
        let now = scripts::stamp(&project.dir, &project.doc);
        let first = self.watched == 0;
        if now == self.watched {
            return;
        }
        self.as_mut().rust_mut().watched = now;
        if !first {
            self.changed();
        }
    }

    pub(crate) fn scripts_now(&self) -> Value {
        let scripts = self.project.as_ref().and_then(|p| p.doc.pointer("/scene/scripts"));
        scripts.cloned().unwrap_or_default()
    }

    pub(crate) fn said(mut self: Pin<&mut Self>, before: &Value) {
        let report = scripts::changes(before, &self.scripts_now());
        self.as_mut().rust_mut().report = report.iter().map(|m| m.local()).collect();
    }
}

fn source(project: &Project, path: &str) -> String {
    let listed = project.doc["scene"]["scripts"].as_array().into_iter().flatten();
    let known = listed.clone().any(|s| s["path"] == path);
    let known = known || path.starts_with("scripts/") && path.ends_with(".js");
    let read = wprs_format::check_path(path).ok().filter(|_| known);
    read.and_then(|_| std::fs::read_to_string(project.dir.join(path)).ok()).unwrap_or_default()
}
