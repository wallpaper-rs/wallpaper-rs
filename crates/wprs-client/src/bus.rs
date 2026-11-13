mod halves;

pub use halves::split;

use std::os::unix::net::UnixStream;
use zbus::address::transport::{Transport, UnixSocket};
use zbus::connection::Builder;

pub fn session_builder() -> zbus::Result<Builder<'static>> {
    builder(zbus::Address::session()?, Builder::session)
}

pub fn system_builder() -> zbus::Result<Builder<'static>> {
    builder(zbus::Address::system()?, Builder::system)
}

pub async fn session() -> zbus::Result<zbus::Connection> {
    session_builder()?.build().await
}

pub async fn system() -> zbus::Result<zbus::Connection> {
    system_builder()?.build().await
}

fn builder(address: zbus::Address, otherwise: fn() -> Builder<'static>) -> zbus::Result<Builder<'static>> {
    let connected = unix_stream(&address);
    match connected
        .map_err(|e| std::io::Error::new(e.kind(), format!("failed to connect to `{address}`: {e}")))?
    {
        Some(stream) => Ok(Builder::socket(split(stream)?)),
        None => Ok(otherwise()),
    }
}

fn unix_stream(address: &zbus::Address) -> std::io::Result<Option<UnixStream>> {
    let Transport::Unix(unix) = address.transport() else { return Ok(None) };
    match unix.path() {
        UnixSocket::File(path) => UnixStream::connect(path).map(Some),
        UnixSocket::Abstract(name) => {
            use std::os::linux::net::SocketAddrExt;
            use std::os::unix::ffi::OsStrExt;
            let address = std::os::unix::net::SocketAddr::from_abstract_name(name.as_bytes())?;
            UnixStream::connect_addr(&address).map(Some)
        }
        _ => Ok(None),
    }
}
