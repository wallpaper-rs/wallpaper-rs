use crate::daemon::Daemon;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::Arc;
use zbus::object_server::SignalEmitter;

const APP: &str = "wprs-app";
pub const START: &str = "--browse";

fn app() -> Option<PathBuf> {
    if let Some(chosen) = std::env::var_os("WPRS_PREVIEW_APP") {
        return Some(PathBuf::from(chosen)).filter(|p| p.is_file());
    }
    let beside = std::env::current_exe().ok().and_then(|exe| Some(exe.parent()?.join(APP)));
    let path = std::env::var_os("PATH").unwrap_or_default();
    let searched = std::env::split_paths(&path).map(|dir| dir.join(APP));
    beside.into_iter().chain(searched).find(|p| p.is_file())
}

pub fn window_name() -> String {
    format!("{}.Preview", wprs_core::dbus::APP_ID)
}

pub async fn open(conn: &zbus::Connection, output: &str) {
    let proxy = zbus::fdo::DBusProxy::new(conn).await;
    let name = zbus::names::BusName::try_from(window_name());
    let owned = match (proxy, name) {
        (Ok(proxy), Ok(name)) => proxy.name_has_owner(name).await.unwrap_or(false),
        _ => false,
    };
    if !owned {
        spawn(output);
    }
}

fn spawn(output: &str) {
    let Some(app) = app() else { return tracing::info!("no {APP} to show the wallpaper preview") };
    let mut command = Command::new(app);
    command.arg(START).arg(output).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
    match command.process_group(0).spawn() {
        Ok(mut child) => drop(std::thread::spawn(move || child.wait())),
        Err(e) => tracing::info!("cannot open the wallpaper preview: {e}"),
    }
}

pub(super) async fn pressed(daemon: &Arc<Daemon>, emitter: &SignalEmitter<'static>, steps: i32) {
    let told = daemon.run(move |d| d.browse_move("", "", steps)).await;
    let told = match told {
        Ok(told) => told,
        Err(e) => {
            tracing::info!("cannot browse wallpapers: {e}");
            return open(emitter.connection(), "").await;
        }
    };
    let _ = super::super::told(&Arc::downgrade(daemon), emitter.clone(), told).await;
}
