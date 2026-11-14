use crate::bus::{self, set_link, spawn};
use std::process::ExitCode;
use wprs_core::dbus::APP_ID;
use zbus::fdo::RequestNameFlags;

const LIBRARY: u8 = 3;
pub const PREVIEW: &str = "qrc:/qt/qml/io/github/wallpaper_rs/app/qml/Preview.qml";

async fn claim() -> bool {
    let conn = match wprs_client::bus::session().await {
        Ok(conn) => conn,
        Err(e) => {
            set_link(Err(format!("cannot connect to the session bus: {e}")));
            return true;
        }
    };
    let name = format!("{APP_ID}.Preview");
    let asked = conn.request_name_with_flags(name.as_str(), RequestNameFlags::DO_NOT_QUEUE).await;
    if let Err(zbus::Error::NameTaken) = asked {
        return false;
    }
    spawn(crate::app::watch(conn));
    true
}

pub fn start() -> ExitCode {
    if bus::checking() {
        set_link(Ok(bus::Daemon::Fixtures));
    } else if !bus::block_on(claim()) {
        return ExitCode::SUCCESS;
    }
    match super::engine(PREVIEW) {
        LIBRARY => library(),
        code => ExitCode::from(code),
    }
}

fn library() -> ExitCode {
    let opened = std::env::current_exe()
        .and_then(|exe| std::process::Command::new(exe).args(["--page", "library"]).spawn());
    if opened.is_ok() { ExitCode::SUCCESS } else { ExitCode::FAILURE }
}
