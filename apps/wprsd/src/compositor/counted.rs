use super::{Command, Compositor};
use std::sync::mpsc;
use std::time::Duration;

const WAIT: Duration = Duration::from_secs(2);

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Counted {
    pub path: &'static str,
    pub reason: &'static str,
    pub alive: u32,
    pub slots: u32,
    pub systems: Vec<(String, u32)>,
}

impl Compositor {
    pub fn count_particles(&self, output: &str) -> Option<Counted> {
        let (done, answer) = mpsc::channel();
        self.tx.send(Command::Count(output.into(), done)).ok()?;
        answer.recv_timeout(WAIT).ok().flatten()
    }
}
