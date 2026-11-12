use super::watch::{Screen, Watch};
use super::{changes, monitors};
use crate::Event;
use crate::desk::Desk;
use std::sync::Arc;
use x11rb::errors::ReplyOrIdError;

impl Watch {
    pub(super) fn monitors(&mut self, refresh: bool) -> Result<(), ReplyOrIdError> {
        let now = monitors(&self.x)?;
        let (gone, _) = changes(&self.known, &now);
        for name in gone {
            self.screens.retain(|s| s.monitor.name != name);
            (self.sink)(Event::Gone(name));
        }
        for monitor in &now {
            let want = (self.wanted)(&monitor.name);
            let at = self.screens.iter().position(|s| s.monitor.name == monitor.name);
            let fresh = refresh || !self.known.contains(monitor);
            match (want, at) {
                (true, Some(i)) => {
                    let screen = &mut self.screens[i];
                    if fresh {
                        screen.desk.place(monitor.area)?;
                        screen.monitor = monitor.clone();
                        let window = screen.desk.window(monitor, self.sink.clone());
                        (self.sink)(Event::Output(monitor.name.clone(), window));
                    }
                }
                (true, None) => {
                    if self.screens.is_empty() {
                        self.x.remember();
                    }
                    let desk = Arc::new(Desk::new(&self.x, monitor.area)?);
                    tracing::info!(output = monitor.name, "desktop window created");
                    let (covered, hidden) = (false, false);
                    let screen =
                        Screen { monitor: monitor.clone(), desk: desk.clone(), covered, hidden, buttons: 0 };
                    self.screens.push(screen);
                    let window = desk.window(monitor, self.sink.clone());
                    (self.sink)(Event::Output(monitor.name.clone(), window));
                }
                (false, Some(i)) => {
                    let screen = self.screens.remove(i);
                    tracing::info!(output = monitor.name, "desktop window removed");
                    drop(screen);
                    self.x.release_area(monitor.area);
                    (self.sink)(Event::Idle(monitor.name.clone(), monitor.placed()));
                }
                (false, None) => {
                    if fresh {
                        (self.sink)(Event::Idle(monitor.name.clone(), monitor.placed()));
                    }
                }
            }
        }
        if self.screens.is_empty() {
            self.x.restore_root();
        }
        self.known = now;
        Ok(())
    }
}
