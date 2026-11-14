use crate::daemon::Daemon;
use futures_util::StreamExt;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Notify;
use wprs_core::dbus;
use wprs_core::model::ShortcutsState;
use wprs_core::shortcuts::Shortcuts;
use zbus::object_server::SignalEmitter;
use zbus::proxy::CacheProperties;
use zbus::{ObjectPath, OwnedObjectPath, OwnedVariant as OwnedValue, Variant as Value};

pub(crate) mod launch;

static REBIND: Notify = Notify::const_new();
pub const NEXT: &str = "browse-next";
pub const PREVIOUS: &str = "browse-previous";

type Options<'a> = HashMap<&'a str, Value<'a>>;

#[zbus::proxy(
    interface = "org.freedesktop.portal.GlobalShortcuts",
    default_service = "org.freedesktop.portal.Desktop",
    default_path = "/org/freedesktop/portal/desktop"
)]
trait GlobalShortcuts {
    fn create_session(&self, options: Options<'_>) -> zbus::Result<OwnedObjectPath>;
    fn bind_shortcuts(
        &self,
        session_handle: &ObjectPath<'_>,
        shortcuts: &[(&str, Options<'_>)],
        parent_window: &str,
        options: Options<'_>,
    ) -> zbus::Result<OwnedObjectPath>;
    #[zbus(signal)]
    fn activated(
        &self,
        session_handle: ObjectPath<'_>,
        shortcut_id: &str,
        timestamp: u64,
        options: HashMap<&str, OwnedValue>,
    ) -> zbus::Result<()>;
}

#[zbus::proxy(
    interface = "org.freedesktop.portal.Request",
    default_service = "org.freedesktop.portal.Desktop"
)]
trait Request {
    #[zbus(signal)]
    fn response(&self, response: u32, results: HashMap<&str, OwnedValue>) -> zbus::Result<()>;
}

#[zbus::proxy(
    interface = "org.freedesktop.portal.Session",
    default_service = "org.freedesktop.portal.Desktop"
)]
trait Session {
    fn close(&self) -> zbus::Result<()>;
}

pub fn rebind() {
    REBIND.notify_one();
}

pub fn proposed(wanted: &Shortcuts) -> Vec<(&'static str, &'static str, Option<&str>)> {
    vec![
        (NEXT, "Preview the next wallpaper", Shortcuts::trigger(&wanted.next)),
        (PREVIOUS, "Preview the previous wallpaper", Shortcuts::trigger(&wanted.previous)),
    ]
}

pub fn steps(id: &str) -> Option<i32> {
    match id {
        NEXT => Some(1),
        PREVIOUS => Some(-1),
        _ => None,
    }
}

async fn asked(
    conn: &zbus::Connection,
    token: &str,
    call: impl AsyncFnOnce() -> zbus::Result<OwnedObjectPath>,
) -> anyhow::Result<HashMap<String, OwnedValue>> {
    let sender = conn.unique_name().map(|n| n.trim_start_matches(':').replace('.', "_")).unwrap_or_default();
    let path = format!("/org/freedesktop/portal/desktop/request/{sender}/{token}");
    let request =
        RequestProxy::builder(conn).path(path).cache_properties(CacheProperties::No).build().await?;
    let mut responses = request.receive_response().await?;
    call().await?;
    let response = responses.next().await.ok_or_else(|| anyhow::anyhow!("the portal did not answer"))?;
    let args = response.args()?;
    anyhow::ensure!(args.response == 0, "refused");
    Ok(args.results.into_iter().map(|(k, v)| (k.to_owned(), v)).collect())
}

async fn bind(
    conn: &zbus::Connection,
    wanted: &Shortcuts,
) -> anyhow::Result<(GlobalShortcutsProxy<'static>, OwnedObjectPath)> {
    if crate::flatpak::app_id().is_none() {
        let options: Options<'_> = HashMap::new();
        let registry = (Some("org.freedesktop.portal.Desktop"), "/org/freedesktop/portal/desktop");
        let iface = Some("org.freedesktop.host.portal.Registry");
        let _ = conn.call_method(registry.0, registry.1, iface, "Register", &(dbus::BUS_NAME, options)).await;
    }
    let portal = GlobalShortcutsProxy::new(conn).await?;
    let token = format!("wprs{}", std::process::id());
    let options = || {
        HashMap::from([
            ("handle_token", Value::from(token.as_str())),
            ("session_handle_token", Value::from(token.as_str())),
        ])
    };
    let created = asked(conn, &token, async || portal.create_session(options()).await).await?;
    let handle =
        created.get("session_handle").ok_or_else(|| anyhow::anyhow!("the portal gave no session"))?;
    let session = OwnedObjectPath::try_from(String::try_from(handle.try_clone()?)?)?;
    let shortcuts = proposed(wanted);
    let described: Vec<(&str, Options<'_>)> = shortcuts
        .iter()
        .map(|(id, text, trigger)| {
            let mut options = HashMap::from([("description", Value::from(*text))]);
            options.extend(trigger.map(|t| ("preferred_trigger", Value::from(t))));
            (*id, options)
        })
        .collect();
    let bound = format!("{token}b");
    let options = HashMap::from([("handle_token", Value::from(bound.as_str()))]);
    asked(conn, &bound, async || portal.bind_shortcuts(&session, &described, "", options).await).await?;
    Ok((portal, session))
}

async fn close(conn: &zbus::Connection, session: &OwnedObjectPath) {
    let proxy = SessionProxy::builder(conn).path(session.clone()).cache_properties(CacheProperties::No);
    if let Ok(proxy) = proxy.build().await {
        let _ = proxy.close().await;
    }
}

pub async fn serve(conn: zbus::Connection, daemon: Arc<Daemon>) {
    let emitter = SignalEmitter::new(&conn, dbus::OBJECT_PATH).map(SignalEmitter::into_owned);
    loop {
        let wanted = daemon.state().config.shortcuts.clone();
        let browse = &daemon.rotations.browse;
        if !wanted.enabled {
            browse.set_shortcuts(ShortcutsState::Off);
            REBIND.notified().await;
            continue;
        }
        browse.set_shortcuts(ShortcutsState::Asking);
        let bound = bind(&conn, &wanted).await;
        let (portal, session) = match bound {
            Ok(bound) => bound,
            Err(e) => {
                let refused = e.to_string() == "refused";
                browse.set_shortcuts(if refused {
                    ShortcutsState::Refused
                } else {
                    ShortcutsState::Unavailable
                });
                tracing::info!("the desktop offers no global shortcuts for browsing wallpapers: {e:#}");
                REBIND.notified().await;
                continue;
            }
        };
        browse.set_shortcuts(ShortcutsState::Bound);
        let Ok(mut activated) = portal.receive_activated().await else { continue };
        loop {
            tokio::select! {
                () = REBIND.notified() => break,
                signal = activated.next() => {
                    let Some(signal) = signal else { break };
                    let Ok(args) = signal.args() else { continue };
                    let (Some(steps), Ok(emitter)) = (steps(args.shortcut_id), &emitter) else { continue };
                    if args.session_handle.as_str() == session.as_str() {
                        launch::pressed(&daemon, emitter, steps).await;
                    }
                }
            }
        }
        close(&conn, &session).await;
    }
}
