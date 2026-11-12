mod backdrop;
mod color;
mod control;
mod cosmic;
mod cover;
mod described;
mod desk;
mod device;
mod handlers;
mod hyprland;
mod modes;
mod monitors;
mod pacer;
mod pointer;
mod presented;
mod pump;
mod restore;
mod scale;
mod stacking;
mod state;
mod toplevel;
mod window;
mod x11;

pub use color::Light;
pub use control::{Control, Placed, Wanted};
pub use described::Mastered;
pub use device::compositor_device;
pub use modes::watch_modes;
pub use pacer::Pacer;
pub use window::{Display, Window};

use pump::{dispatch, ended};
use smithay_client_toolkit::reexports::client::globals::{GlobalList, registry_queue_init};
use smithay_client_toolkit::reexports::client::{Connection, EventQueue};
use state::State;
use std::sync::Arc;
use x11::X11;

pub enum Event {
    Output(String, Window),
    Gone(String),
    Idle(String, Placed),
    Frame(String),
    Covered(String, Cover, bool),
    Pointer(String, Option<[f32; 2]>, u8),
    Input(String),
    Exposed(String),
    Backdrop(String),
    Lost(String),
    Failed(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cover {
    Starved,
    Toplevel,
    Hyprland,
    Overlap,
}

pub(crate) type Sink = Arc<dyn Fn(Event) + Send + Sync>;

pub struct Session(Kind);

enum Kind {
    Wayland(Connection, GlobalList, EventQueue<State>, String),
    X11(Arc<X11>),
}

pub fn connect(namespace: &str) -> anyhow::Result<Session> {
    let conn = Connection::connect_to_env()?;
    let (globals, queue) = registry_queue_init::<State>(&conn)?;
    let offered = |list: &[_]| list.iter().any(|g: &_| state::is_layer_shell(g));
    anyhow::ensure!(
        globals.contents().with_list(offered),
        "the compositor does not offer layer-shell surfaces"
    );
    Ok(Session(Kind::Wayland(conn, globals, queue, namespace.to_owned())))
}

pub fn connect_x11() -> anyhow::Result<Session> {
    Ok(Session(Kind::X11(X11::connect()?)))
}

impl Session {
    pub fn kind(&self) -> &'static str {
        match self.0 {
            Kind::Wayland(..) => "layer-shell",
            Kind::X11(_) => "x11",
        }
    }

    pub fn display(&self) -> Display {
        match &self.0 {
            Kind::Wayland(conn, ..) => Display::Wayland(conn.clone()),
            Kind::X11(x) => Display::X11(x.clone()),
        }
    }

    pub fn start(
        self,
        wanted: impl Fn(&str) -> bool + Send + Sync + 'static,
        sink: impl Fn(Event) + Send + Sync + 'static,
    ) -> anyhow::Result<Control> {
        let (sink, wanted): (Sink, Wanted) = (Arc::new(sink), Arc::new(wanted));
        let (conn, globals, mut queue, namespace) = match self.0 {
            Kind::Wayland(conn, globals, queue, namespace) => (conn, globals, queue, namespace),
            Kind::X11(x) => {
                monitors::start(x.clone(), sink, wanted)?;
                return Ok(Control::x11(x));
            }
        };
        let mut state = State::new(conn, &globals, &queue.handle(), sink.clone(), wanted, namespace)?;
        hyprland::watch(sink);
        let (wake, listener) = control::Wake::pair()?;
        let control = Control::wayland(wake.clone());
        std::thread::Builder::new().name("wprs-wayland".into()).spawn(move || {
            let error = loop {
                let retry = state.retry_at();
                if let Err(e) = dispatch(&mut queue, &mut state, retry, Some(&listener)) {
                    break e;
                }
                state.reopen();
                if wake.flag.swap(false, std::sync::atomic::Ordering::Acquire) {
                    state.reconcile();
                }
            };
            (state.sink)(ended(error))
        })?;
        Ok(control)
    }
}
