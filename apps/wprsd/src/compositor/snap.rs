use super::{Command, Compositor};
use std::sync::mpsc;
use std::time::Duration;

pub use super::pages::snap::Taken;

const WAIT: Duration = Duration::from_secs(2);

impl Compositor {
    pub fn snap_page(&self, output: &str) -> Option<Taken> {
        let (done, answer) = mpsc::channel();
        self.tx.send(Command::Snap(output.into(), done)).ok()?;
        answer.recv_timeout(WAIT).ok()
    }
}
