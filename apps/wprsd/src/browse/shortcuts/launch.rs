use crate::daemon::Daemon;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::Arc;
use zbus::object_server::SignalEmitter;

const APP: &str = "wprs-app";
pub const START: &str = "--browse";

fn app() -> Option<PathBuf> {
    let beside = std::env::current_exe().ok().and_then(|exe| Some(exe.parent()?.join(APP)));
    let path = std::env::var_os("PATH").unwrap_or_default();
    let searched = std::env::split_paths(&path).map(|dir| dir.join(APP));
    beside.into_iter().chain(searched).find(|p| p.is_file())
}

fn open(output: &str) {
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
        Err(e) => return tracing::info!("cannot browse wallpapers: {e}"),
    };
    let (output, fresh) = (told.output.clone(), told.fresh.is_some());
    let _ = super::super::told(&Arc::downgrade(daemon), emitter.clone(), told).await;
    if fresh {
        open(&output);
    }
}
