#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[auto_cxx_name]
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, folder)]
        #[qproperty(QString, view)]
        #[qproperty(QString, fields)]
        #[qproperty(QString, selected)]
        #[qproperty(QString, error)]
        #[qproperty(QString, output)]
        #[qproperty(QString, placed)]
        #[qproperty(QString, look)]
        #[qproperty(QString, levels)]
        #[qproperty(u32, node)]
        #[qproperty(f64, serial)]
        #[qproperty(f64, time)]
        #[qproperty(bool, playing)]
        #[qproperty(bool, ready)]
        #[qproperty(bool, dirty)]
        #[qproperty(bool, live)]
        #[qproperty(bool, busy)]
        #[qproperty(bool, sweeping)]
        type SceneEditor = super::SceneEditorRust;

        #[qinvokable]
        fn create(self: Pin<&mut SceneEditor>, folder: &QString, starter: &QString, title: &QString);
        #[qinvokable]
        fn open(self: Pin<&mut SceneEditor>, folder: &QString, package: &QString);
        #[qinvokable]
        fn copy_facts(self: &SceneEditor, package: &QString) -> QString;
        #[qinvokable]
        fn copy_into(
            self: &SceneEditor,
            package: &QString,
            folder: &QString,
            id: &QString,
            mine: bool,
        ) -> QString;
        #[qinvokable]
        fn run(self: Pin<&mut SceneEditor>, action: &QString, args: &QString, gesture: bool);
        #[qinvokable]
        fn hold(self: Pin<&mut SceneEditor>, on: bool);
        #[qinvokable]
        fn select(self: Pin<&mut SceneEditor>, pointer: &QString);
        #[qinvokable]
        fn inspect(self: &SceneEditor, pointer: &QString) -> QString;
        #[qinvokable]
        fn count(self: &SceneEditor, pointer: &QString) -> i32;
        #[qinvokable]
        fn value(self: &SceneEditor, pointer: &QString) -> QString;
        #[qinvokable]
        fn condition(self: &SceneEditor, pointer: &QString, text: &QString) -> QString;
        #[qinvokable]
        fn scripts(self: &SceneEditor, id: &QString) -> QString;
        #[qinvokable]
        fn code(self: &SceneEditor, query: &QString, args: &QString) -> QString;
        #[qinvokable]
        fn watch(self: Pin<&mut SceneEditor>);
        #[qinvokable]
        fn rig(self: &SceneEditor, id: &QString, query: &QString) -> QString;
        #[qinvokable]
        fn save(self: Pin<&mut SceneEditor>);
        #[qinvokable]
        fn recover(self: Pin<&mut SceneEditor>, keep: bool);
        #[qinvokable]
        fn prune(self: Pin<&mut SceneEditor>);
        #[qinvokable]
        fn autosave(self: Pin<&mut SceneEditor>);
        #[qinvokable]
        fn build(self: Pin<&mut SceneEditor>, out: &QString, key: &QString, password: &QString);
        #[qinvokable]
        fn apply(self: Pin<&mut SceneEditor>, output: &QString);
        #[qinvokable]
        fn seek(self: Pin<&mut SceneEditor>, time: f64, playing: bool);
        #[qinvokable]
        fn attach(self: Pin<&mut SceneEditor>);
        #[qinvokable]
        fn cover(self: Pin<&mut SceneEditor>, covered: bool);
        #[qinvokable]
        fn simulate_media(self: Pin<&mut SceneEditor>, file: &QString);
        #[qinvokable]
        fn fire(self: Pin<&mut SceneEditor>, trigger: &QString);
        #[qinvokable]
        fn touch(self: Pin<&mut SceneEditor>, x: f64, y: f64, buttons: u32);
        #[qinvokable]
        fn hear(self: Pin<&mut SceneEditor>, on: bool);
        #[qinvokable]
        fn sweep(self: Pin<&mut SceneEditor>, on: bool);
        #[qinvokable]
        fn wave(self: Pin<&mut SceneEditor>, asset: &QString, buckets: i32);
        #[qinvokable]
        fn preview(self: Pin<&mut SceneEditor>, live: bool);
        #[qinvokable]
        fn render(self: Pin<&mut SceneEditor>, out: &QString, spec: &QString);
        #[qinvokable]
        fn folder_for(self: &SceneEditor, name: &QString) -> QString;
        #[qinvokable]
        fn projects(self: &SceneEditor) -> QString;
        #[qinvokable]
        fn kind_of(self: &SceneEditor, folder: &QString, package: &QString) -> QString;
        #[qsignal]
        fn finished(self: Pin<&mut SceneEditor>, message: QString);
        #[qsignal]
        fn rendered(self: Pin<&mut SceneEditor>, files: QString);
        fn waved(self: Pin<&mut SceneEditor>, asset: QString, wave: QString);
    }

    impl cxx_qt::Threading for SceneEditor {}
}

use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;
use qobject::SceneEditor;
use serde_json::{Value, json};
use std::pin::Pin;
use wprs_editor::{Action, Problem, Project};

#[derive(Default)]
pub struct SceneEditorRust {
    pub folder: QString,
    pub view: QString,
    pub fields: QString,
    pub selected: QString,
    pub error: QString,
    pub output: QString,
    pub placed: QString,
    pub look: QString,
    pub levels: QString,
    pub sweeping: bool,
    pub node: u32,
    pub serial: f64,
    pub time: f64,
    pub playing: bool,
    pub ready: bool,
    pub dirty: bool,
    pub live: bool,
    pub busy: bool,
    pub project: Option<Project>,
    pub imports: Vec<Action>,
    pub report: Vec<String>,
    pub watched: u64,
    pub files: Vec<Problem>,
    pub persisted: String,
    pub sending: bool,
    pub queued: bool,
    pub sent: u64,
    pub shown: bool,
    pub attached: (u32, u32),
}

impl SceneEditor {
    pub fn run(mut self: Pin<&mut Self>, action: &QString, args: &QString, gesture: bool) {
        let args = serde_json::from_str::<Value>(&args.to_string()).unwrap_or(json!({}));
        let action = match serde_json::from_value::<Action>(json!({ action.to_string(): args })) {
            Ok(action) if crate::scenework::queued(&action) => return self.import(action),
            Ok(action) => action,
            Err(e) => return self.fail(e),
        };
        self.as_mut().scoped();
        let before = matches!(action, Action::ImportScript(_)).then(|| self.scripts_now());
        let done = {
            let mut this = self.as_mut().rust_mut();
            let Some(project) = this.project.as_mut() else { return };
            if gesture { project.drag(&action) } else { project.act(&action) }
        };
        match done {
            Ok(_) => self.as_mut().changed(),
            Err(e) => return self.fail(e),
        }
        if let Some(before) = before {
            self.as_mut().said(&before);
        }
        if gesture && let Some(project) = self.rust_mut().project.as_mut() {
            project.held();
        }
    }

    pub fn changed(mut self: Pin<&mut Self>) {
        self.as_mut().set_error(QString::default());
        self.as_mut().refresh();
        self.as_mut().attach();
        self.send();
    }
}
