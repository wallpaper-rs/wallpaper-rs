use crate::{Error, Frame, Host, Setup};
use std::path::PathBuf;
use std::time::{Duration, Instant};
use wprs_scene::ScriptOutputs;

pub const HOST_ARG: &str = "--script-host";
const ANSWER: Duration = Duration::from_secs(5);
const POLL: Duration = Duration::from_millis(1);

pub fn serve_if_asked() {
    if std::env::args().nth(1).as_deref() != Some(HOST_ARG) {
        return;
    }
    let code = match crate::serve() {
        Ok(()) => 0,
        Err(_) => 1,
    };
    std::process::exit(code);
}

pub struct Remote {
    host: Host,
}

impl Remote {
    pub fn start(setup: &Setup, program: Option<(PathBuf, Vec<String>)>) -> Remote {
        let (program, args) = program.unwrap_or_else(|| ("/proc/self/exe".into(), vec![HOST_ARG.into()]));
        let args: Vec<&str> = args.iter().map(String::as_str).collect();
        Remote { host: Host::new(program, &args, setup) }
    }

    pub fn step(&mut self, frame: &Frame) -> Result<ScriptOutputs, Error> {
        let ends = Instant::now() + ANSWER;
        loop {
            match self.host.update(frame) {
                Ok(Some(out)) => return Ok(out),
                Ok(None) => {}
                Err(Error::Host(_)) if Instant::now() < ends => {}
                Err(e) => return Err(e),
            }
            if Instant::now() >= ends {
                return Err(Error::Host("the script host did not answer".into()));
            }
            std::thread::sleep(POLL);
        }
    }
}
