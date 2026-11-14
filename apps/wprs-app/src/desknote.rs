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
        #[qproperty(QString, shown)]
        type DeskNote = super::DeskNoteRust;

        #[qinvokable]
        fn show(self: Pin<&mut DeskNote>, title: &QString, body: &QString, actions: &QString);
        #[qinvokable]
        fn close(self: Pin<&mut DeskNote>);
        #[qsignal]
        fn acted(self: Pin<&mut DeskNote>, key: QString);
        #[qsignal]
        fn failed(self: Pin<&mut DeskNote>);
    }

    impl cxx_qt::Threading for DeskNote {}
}

use crate::bus;
use cxx_qt::{CxxQtType, Threading};
use cxx_qt_lib::QString;
use futures_util::StreamExt;
use qobject::DeskNote;
use std::collections::HashMap;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use zbus::Variant;

const APP: &str = "io.github.wallpaper_rs";

static SESSION: tokio::sync::OnceCell<zbus::Connection> = tokio::sync::OnceCell::const_new();

async fn session() -> zbus::Result<&'static zbus::Connection> {
    SESSION.get_or_try_init(zbus::Connection::session).await
}

#[zbus::proxy(
    interface = "org.freedesktop.Notifications",
    default_service = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
trait Notifications {
    #[allow(clippy::too_many_arguments)]
    fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: &[&str],
        hints: HashMap<&str, Variant<'_>>,
        expire_timeout: i32,
    ) -> zbus::Result<u32>;

    fn close_notification(&self, id: u32) -> zbus::Result<()>;

    #[zbus(signal)]
    fn action_invoked(&self, id: u32, action_key: String) -> zbus::Result<()>;
}

#[derive(Default)]
pub struct DeskNoteRust {
    pub shown: QString,
    id: Arc<AtomicU32>,
    listening: bool,
}

pub fn buttons(actions: &str) -> Vec<String> {
    serde_json::from_str::<Vec<[String; 2]>>(actions).unwrap_or_default().into_iter().flatten().collect()
}

impl DeskNote {
    pub fn show(mut self: Pin<&mut Self>, title: &QString, body: &QString, actions: &QString) {
        self.as_mut().set_shown(title.clone());
        if bus::checking() {
            return;
        }
        let (title, body, buttons) = (title.to_string(), body.to_string(), buttons(&actions.to_string()));
        let (qt, id, listen) = (self.qt_thread(), self.id.clone(), !self.listening);
        self.as_mut().rust_mut().listening = true;
        bus::spawn(async move {
            let shown = async {
                let notes = NotificationsProxy::new(session().await?).await?;
                let keys: Vec<&str> = buttons.iter().map(String::as_str).collect();
                let hints = HashMap::from([
                    ("desktop-entry", Variant::from(APP)),
                    ("resident", Variant::from(true)),
                    ("urgency", Variant::from(1u8)),
                ]);
                let old = id.load(Ordering::Relaxed);
                let new = notes.notify("wallpaper-rs", old, APP, &title, &body, &keys, hints, 0).await?;
                id.store(new, Ordering::Relaxed);
                zbus::Result::Ok(notes)
            };
            match shown.await {
                Ok(notes) if listen => listen_to(notes, id, qt).await,
                Ok(_) => {}
                Err(_) => {
                    qt.queue(|mut note| {
                        note.as_mut().rust_mut().listening = false;
                        note.failed();
                    })
                    .ok();
                }
            }
        });
    }

    pub fn close(mut self: Pin<&mut Self>) {
        self.as_mut().set_shown(QString::default());
        let id = self.id.swap(0, Ordering::Relaxed);
        if id == 0 || bus::checking() {
            return;
        }
        bus::spawn(async move {
            if let Ok(bus) = session().await
                && let Ok(notes) = NotificationsProxy::new(bus).await
            {
                notes.close_notification(id).await.ok();
            }
        });
    }
}

async fn listen_to(
    notes: NotificationsProxy<'static>,
    id: Arc<AtomicU32>,
    qt: cxx_qt::CxxQtThread<DeskNote>,
) {
    let Ok(mut actions) = notes.receive_action_invoked().await else { return };
    while let Some(signal) = actions.next().await {
        let Ok(args) = signal.args() else { continue };
        if args.id != id.load(Ordering::Relaxed) {
            continue;
        }
        let key = QString::from(args.action_key.as_str());
        if qt.queue(move |note| note.acted(key)).is_err() {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::buttons;

    #[test]
    fn pairs_become_flat_keys_and_words() {
        let flat = buttons(r#"[["back","Back to the editor"],["keep","Keep it"]]"#);
        assert_eq!(flat, ["back", "Back to the editor", "keep", "Keep it"]);
        assert!(buttons("nonsense").is_empty());
    }
}
