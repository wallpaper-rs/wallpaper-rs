use crate::bus;
use crate::shards::{self, Census};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitCode, Stdio};
use std::sync::{Arc, Mutex, mpsc};
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

fn spawn(state: &Path, shard: usize, shards: usize, font_file: Option<&Path>) -> std::io::Result<Child> {
    let platform = std::env::var("QT_QPA_PLATFORM").unwrap_or_else(|_| "offscreen".into());
    let own = state.join(format!("shard-{shard}"));
    let mut command = Command::new(std::env::current_exe()?);
    command.env(bus::CHECK, "1").env("QT_QPA_PLATFORM", platform).env("QT_FORCE_STDERR_LOGGING", "1");
    command.env("XDG_STATE_HOME", own.join("state"));
    if shards > 1 {
        command.env("XDG_DATA_HOME", own.join("data")).env("XDG_CACHE_HOME", own.join("cache"));
        command.env(shards::CLAIMS, state.join("claims"));
    }
    if let Some(file) = font_file {
        command.env("FONTCONFIG_FILE", file);
    }
    command.args(["--page", "settings"]);
    command.stderr(Stdio::piped()).spawn()
}

fn read(stderr: std::process::ChildStderr, census: Arc<Mutex<Census>>, problems: Arc<Mutex<usize>>) {
    for line in BufReader::new(stderr).lines().map_while(Result::ok) {
        eprintln!("{line}");
        census.lock().unwrap().note(&line);
        let bad = [".qml", "QQmlApplicationEngine", "call failed"].iter().any(|p| line.contains(p));
        *problems.lock().unwrap() += usize::from(bad);
    }
}

pub fn run() -> ExitCode {
    let state = std::env::temp_dir().join(format!("wprs-app-check-state-{}", std::process::id()));
    let font_file = std::env::var_os("FONTCONFIG_FILE").is_none().then(|| fonts(&state)).flatten();
    let shards = shards::count();
    std::fs::create_dir_all(state.join("claims")).ok();
    let spawned = (0..shards).map(|n| spawn(&state, n, shards, font_file.as_deref()));
    let Ok(mut children) = spawned.collect::<Result<Vec<_>, _>>() else { return ExitCode::FAILURE };
    let ids: Vec<u32> = children.iter().map(Child::id).collect();
    let census = Arc::new(Mutex::new(Census::default()));
    let problems = Arc::new(Mutex::new(0usize));
    let readers: Vec<_> = children
        .iter_mut()
        .map(|child| {
            let stderr = child.stderr.take().unwrap();
            let (census, problems) = (census.clone(), problems.clone());
            std::thread::spawn(move || read(stderr, census, problems))
        })
        .collect();
    let children: Vec<_> = children.into_iter().map(|child| Arc::new(Mutex::new(child))).collect();
    let (done, waited) = mpsc::channel();
    let watched = children.clone();
    let watch = std::thread::spawn(move || {
        let late = waited.recv_timeout(CHECK_TIME).is_err();
        if late {
            eprintln!("wprs-app: the QML check did not finish within {} s", CHECK_TIME.as_secs());
            watched.iter().for_each(|child| drop(child.lock().unwrap().kill()));
        }
        late
    });
    readers.into_iter().for_each(|reader| drop(reader.join()));
    done.send(()).ok();
    let late = watch.join().unwrap_or(true);
    let failed = children.iter().any(|child| !child.lock().unwrap().wait().is_ok_and(|s| s.success()));
    for id in ids {
        std::fs::remove_dir_all(std::env::temp_dir().join(format!("wprs-app-check-{id}"))).ok();
    }
    std::fs::remove_dir_all(&state).ok();
    let census = census.lock().unwrap();
    let missing = if shards > 1 { census.problems() } else { Vec::new() };
    missing.iter().for_each(|m| eprintln!("wprs-app: {m}"));
    let found = *problems.lock().unwrap() + missing.len() + usize::from(late);
    if shards > 1 {
        eprintln!("wprs-app: {} check units in {shards} processes", census.units());
    }
    if failed || found > 0 {
        eprintln!("wprs-app: the QML check found {found} problems");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
