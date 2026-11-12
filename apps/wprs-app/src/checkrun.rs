use crate::bus;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};
use std::time::Duration;

const CHECK_TIME: Duration = Duration::from_secs(360);
const FONT_DIRS: [&str; 1] = ["/usr/share/fonts"];

fn fonts(state: &Path) -> Option<PathBuf> {
    let dirs: String = FONT_DIRS
        .iter()
        .filter(|dir| Path::new(dir).is_dir())
        .map(|dir| format!("<dir>{dir}</dir>\n"))
        .collect();
    (!dirs.is_empty()).then_some(())?;
    std::fs::create_dir_all(state).ok()?;
    let cache = state.join("fonts-cache");
    let file = state.join("fonts.conf");
    let text = format!(
        "<?xml version=\"1.0\"?>\n<fontconfig>\n{dirs}<cachedir>{}</cachedir>\n</fontconfig>\n",
        cache.display()
    );
    std::fs::write(&file, text).ok()?;
    Some(file)
}

pub fn run() -> ExitCode {
    let platform = std::env::var("QT_QPA_PLATFORM").unwrap_or_else(|_| "offscreen".into());
    let state = std::env::temp_dir().join(format!("wprs-app-check-state-{}", std::process::id()));
    let font_file = std::env::var_os("FONTCONFIG_FILE").is_none().then(|| fonts(&state)).flatten();
    let child = std::env::current_exe().and_then(|exe| {
        let mut command = Command::new(exe);
        command.env(bus::CHECK, "1").env("QT_QPA_PLATFORM", platform).env("QT_FORCE_STDERR_LOGGING", "1");
        command.env("XDG_STATE_HOME", &state);
        if let Some(file) = &font_file {
            command.env("FONTCONFIG_FILE", file);
        }
        command.args(["--page", "settings"]);
        command.stderr(Stdio::piped()).spawn()
    });
    let Ok(mut child) = child else { return ExitCode::FAILURE };
    let (stderr, id) = (child.stderr.take().unwrap(), child.id());
    let (done, waited) = std::sync::mpsc::channel();
    let watch = std::thread::spawn(move || {
        let late = waited.recv_timeout(CHECK_TIME).is_err();
        if late {
            eprintln!("wprs-app: the QML check did not finish within {} s", CHECK_TIME.as_secs());
            child.kill().ok();
        }
        (child.wait(), late)
    });
    let mut problems = 0;
    for line in BufReader::new(stderr).lines().map_while(Result::ok) {
        eprintln!("{line}");
        problems +=
            usize::from([".qml", "QQmlApplicationEngine", "call failed"].iter().any(|p| line.contains(p)));
    }
    done.send(()).ok();
    let (status, late) = watch.join().unwrap_or((Err(std::io::ErrorKind::Other.into()), true));
    problems += usize::from(late);
    std::fs::remove_dir_all(std::env::temp_dir().join(format!("wprs-app-check-{id}"))).ok();
    std::fs::remove_dir_all(&state).ok();
    match status {
        Ok(status) if status.success() && problems == 0 => ExitCode::SUCCESS,
        _ => {
            eprintln!("wprs-app: the QML check found {problems} problems");
            ExitCode::FAILURE
        }
    }
}
