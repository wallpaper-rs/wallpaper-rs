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
        #[qproperty(bool, busy)]
        type Signing = super::SigningRust;

        #[qinvokable]
        fn folder(self: &Signing) -> QString;
        #[qinvokable]
        fn keys(self: &Signing, folder: &QString) -> QString;
        #[qinvokable]
        fn exists(self: &Signing, folder: &QString, name: &QString) -> bool;
        #[qinvokable]
        fn create(
            self: Pin<&mut Signing>,
            folder: &QString,
            name: &QString,
            password: &QString,
            replace: bool,
        );
        #[qinvokable]
        fn rotate(
            self: Pin<&mut Signing>,
            secret: &QString,
            password: &QString,
            new_key: &QString,
            author: &QString,
        );
        #[qinvokable]
        fn unlocks(self: &Signing, secret: &QString, password: &QString) -> bool;
        #[qinvokable]
        fn keep(self: &Signing, secret: &QString, folder: &QString) -> QString;
        #[qsignal]
        fn answered(self: Pin<&mut Signing>, operation: QString, answer: QString);
    }

    impl cxx_qt::Threading for Signing {}
}

use crate::bus;
use crate::scenefile::Shown;
use cxx_qt::Threading;
use cxx_qt_lib::QString;
use qobject::Signing;
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use std::pin::Pin;
use wprs_editor::author_keys;
use zeroize::Zeroizing;

#[derive(Default)]
pub struct SigningRust {
    busy: bool,
}

fn answer(result: anyhow::Result<Value>) -> QString {
    QString::from(&result.unwrap_or_else(|e| json!({ "error": e.shown() })).to_string())
}

fn secret(q: &QString) -> Option<Zeroizing<String>> {
    Some(Zeroizing::new(q.to_string())).filter(|t| !t.is_empty())
}

pub fn default_folder() -> PathBuf {
    match bus::checking() {
        true => crate::scenefile::projects_dir().join("keys"),
        false => author_keys::folder(),
    }
}

impl Signing {
    fn behind(
        mut self: Pin<&mut Self>,
        operation: &'static str,
        job: impl FnOnce() -> anyhow::Result<Value> + Send + 'static,
    ) {
        if bus::checking() {
            return self.answered(QString::from(operation), answer(job()));
        }
        self.as_mut().set_busy(true);
        let qt = self.qt_thread();
        bus::spawn(async move {
            let result = tokio::task::spawn_blocking(job).await.map_err(anyhow::Error::from);
            let reply = answer(result.and_then(|r| r));
            qt.queue(move |mut signing| {
                signing.as_mut().set_busy(false);
                signing.answered(QString::from(operation), reply);
            })
            .ok();
        });
    }

    pub fn folder(&self) -> QString {
        QString::from(default_folder().to_string_lossy().as_ref())
    }

    pub fn keys(&self, folder: &QString) -> QString {
        QString::from(&author_keys::list(Path::new(&folder.to_string())).to_string())
    }

    pub fn exists(&self, folder: &QString, name: &QString) -> bool {
        let dir = PathBuf::from(folder.to_string());
        let name = name.to_string();
        dir.join(format!("{name}.key")).exists() || dir.join(format!("{name}.pub")).exists()
    }

    pub fn create(self: Pin<&mut Self>, folder: &QString, name: &QString, password: &QString, replace: bool) {
        let (dir, name, password) = (PathBuf::from(folder.to_string()), name.to_string(), secret(password));
        self.behind("create", move || {
            author_keys::create(&dir, &name, password.map(|p| p.to_string()), replace)
        });
    }

    pub fn rotate(
        self: Pin<&mut Self>,
        secret_key: &QString,
        password: &QString,
        new_key: &QString,
        author: &QString,
    ) {
        let (old, password) = (PathBuf::from(secret_key.to_string()), secret(password));
        let (new, author) = (new_key.to_string(), author.to_string());
        self.behind("rotate", move || {
            author_keys::rotate(&old, password.map(|p| p.to_string()), &new, &author)
        });
    }

    pub fn unlocks(&self, secret_key: &QString, password: &QString) -> bool {
        let password = secret(password).map(|p| p.to_string());
        wprs_editor::package::signer(Path::new(&secret_key.to_string()), password).is_ok()
    }

    pub fn keep(&self, secret_key: &QString, folder: &QString) -> QString {
        let (key, dir) = (PathBuf::from(secret_key.to_string()), PathBuf::from(folder.to_string()));
        answer((|| {
            let mut kept = Vec::new();
            for file in [key.clone(), key.with_extension("pub")] {
                let name = file.file_name().ok_or_else(|| anyhow::anyhow!("no key file"))?;
                std::fs::copy(&file, dir.join(name))?;
                kept.push(dir.join(name).to_string_lossy().into_owned());
            }
            Ok(json!({ "kept": kept }))
        })())
    }
}
