use crate::x11::X11;
use std::io::{ErrorKind, Read, Write};
use std::os::unix::net::UnixStream;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use x11rb::connection::Connection as _;
use x11rb::protocol::xproto::{AtomEnum, PropMode};
use x11rb::wrapper::ConnectionExt as _;

pub type Wanted = Arc<dyn Fn(&str) -> bool + Send + Sync>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Placed {
    pub logical: (u32, u32),
    pub pixels: (u32, u32),
    pub scale: u32,
    pub refresh: u32,
    pub position: (i32, i32),
    pub mm: (u32, u32),
    pub identity: String,
}

#[derive(Clone)]
pub struct Control(Kind);

#[derive(Clone)]
enum Kind {
    Wayland(Arc<Wake>),
    X11(Arc<X11>),
}

pub(crate) struct Wake {
    pub(crate) flag: AtomicBool,
    pub(crate) tx: UnixStream,
}

impl Placed {
    pub fn size(&self) -> (u32, u32) {
        self.pixels
    }
}

impl Wake {
    pub(crate) fn pair() -> std::io::Result<(Arc<Self>, UnixStream)> {
        let (tx, rx) = UnixStream::pair()?;
        tx.set_nonblocking(true)?;
        rx.set_nonblocking(true)?;
        Ok((Arc::new(Self { flag: AtomicBool::new(false), tx }), rx))
    }

    pub(crate) fn drain(rx: &UnixStream) {
        let mut buffer = [0u8; 64];
        let mut rx = rx;
        while rx.read(&mut buffer).is_ok_and(|n| n > 0) {}
    }
}

impl Control {
    pub(crate) fn wayland(wake: Arc<Wake>) -> Self {
        Self(Kind::Wayland(wake))
    }

    pub(crate) fn x11(x: Arc<X11>) -> Self {
        Self(Kind::X11(x))
    }

    pub fn refresh(&self) {
        match &self.0 {
            Kind::Wayland(wake) => {
                wake.flag.store(true, Ordering::Release);
                let mut tx = &wake.tx;
                match tx.write(&[1]) {
                    Err(e) if e.kind() != ErrorKind::WouldBlock => {
                        tracing::debug!("cannot wake the session: {e}")
                    }
                    _ => {}
                }
            }
            Kind::X11(x) => {
                let stamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map_or(0, |d| d.subsec_nanos());
                let sent = x.conn.change_property32(
                    PropMode::REPLACE,
                    x.root,
                    x.atoms._WPRS_REFRESH,
                    AtomEnum::CARDINAL,
                    &[stamp],
                );
                if sent.is_err() || x.conn.flush().is_err() {
                    tracing::debug!("cannot wake the X11 session");
                }
            }
        }
    }
}
