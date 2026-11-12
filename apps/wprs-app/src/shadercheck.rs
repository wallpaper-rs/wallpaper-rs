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
        #[qproperty(QString, report)]
        #[qproperty(bool, checking)]
        type ShaderCheck = super::ShaderCheckRust;

        #[qinvokable]
        fn check(self: Pin<&mut ShaderCheck>, effect: &QString, descriptor: &QString, wgsl: &QString);
    }

    impl cxx_qt::Threading for ShaderCheck {}
}

use crate::bus;
use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::QString;
use qobject::ShaderCheck;
use std::pin::Pin;

#[derive(Default)]
pub struct ShaderCheckRust {
    pub report: QString,
    pub checking: bool,
    pub serial: u64,
}

impl ShaderCheck {
    pub fn check(mut self: Pin<&mut Self>, effect: &QString, descriptor: &QString, wgsl: &QString) {
        let serial = self.serial + 1;
        self.as_mut().rust_mut().serial = serial;
        self.as_mut().set_checking(true);
        let (id, json, code) = (effect.to_string(), descriptor.to_string(), wgsl.to_string());
        let qt = self.qt_thread();
        bus::spawn(async move {
            let job = move || wprs_editor::wgsl::check(&id, &json, &code);
            let found = tokio::task::spawn_blocking(job).await.unwrap_or_default();
            qt.queue(move |mut check| {
                if check.serial != serial {
                    return;
                }
                check.as_mut().set_checking(false);
                check.as_mut().set_report(QString::from(&found.to_string()));
            })
            .ok();
        });
    }
}
