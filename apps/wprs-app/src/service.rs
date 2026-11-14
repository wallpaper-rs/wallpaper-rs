use crate::args::{fail, invalid};
use crate::bus::checking;
use wprs_client::Error;
use zbus::Connection;

pub const SERVICE: &[(&str, &str, &[&str])] = &[("Service", "Restart", &[])];

const UNIT: &str = "wprsd.service";
const DAEMON: &str = "io.github.wallpaper_rs.Daemon";
const SYSTEMD: (&str, &str, &str) =
    ("org.freedesktop.systemd1", "/org/freedesktop/systemd1", "org.freedesktop.systemd1.Manager");
const DBUS: (&str, &str, &str) = ("org.freedesktop.DBus", "/org/freedesktop/DBus", "org.freedesktop.DBus");

pub async fn call(member: &str) -> Result<String, Error> {
    match member {
        "Restart" if checking() => Ok("[]".into()),
        "Restart" => restart().await.map(|()| "[]".into()),
        _ => Err(invalid(format!("unknown method Service.{member}"))),
    }
}

async fn restart() -> Result<(), Error> {
    let conn = wprs_client::bus::session().await.map_err(fail)?;
    let (to, path, iface) = SYSTEMD;
    conn.call_method(Some(to), path, Some(iface), "ResetFailedUnit", &(UNIT,)).await.ok();
    let restarted = conn.call_method(Some(to), path, Some(iface), "RestartUnit", &(UNIT, "replace")).await;
    match restarted {
        Ok(_) => Ok(()),
        Err(e) => activate(&conn).await.map_err(|_| fail(e)),
    }
}

async fn activate(conn: &Connection) -> zbus::Result<()> {
    let (to, path, iface) = DBUS;
    conn.call_method(Some(to), path, Some(iface), "StartServiceByName", &(DAEMON, 0u32)).await.map(|_| ())
}

#[cfg(test)]
mod tests {
    use super::{SERVICE, call};
    use crate::bus::block_on;

    #[test]
    fn the_service_table_knows_only_restart() {
        assert_eq!(SERVICE, &[("Service", "Restart", &[] as &[&str])]);
        assert!(block_on(call("Stop")).is_err());
    }
}
