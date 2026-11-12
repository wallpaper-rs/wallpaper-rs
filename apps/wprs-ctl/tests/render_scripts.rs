use std::path::Path;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

fn ctl(home: &Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_wprs-ctl"));
    command
        .env("XDG_STATE_HOME", home.join("state"))
        .env("XDG_DATA_HOME", home.join("data"))
        .env("XDG_CONFIG_HOME", home.join("config"))
        .env("WPRS_PRIVATE_BUS_ONLY", "1")
        .env("LANG", "C.UTF-8");
    command
}

fn children_of(pid: u32) -> Vec<String> {
    let mut found = Vec::new();
    for entry in std::fs::read_dir("/proc").unwrap().flatten() {
        let Ok(stat) = std::fs::read_to_string(entry.path().join("stat")) else { continue };
        let after = stat.rsplit_once(')').map(|(_, rest)| rest.trim().to_owned()).unwrap_or_default();
        let parent: u32 = after.split(' ').nth(1).and_then(|p| p.parse().ok()).unwrap_or(0);
        if parent == pid {
            let cmdline = std::fs::read(entry.path().join("cmdline")).unwrap_or_default();
            found.push(String::from_utf8_lossy(&cmdline).replace('\0', " "));
        }
    }
    found
}

#[test]
fn a_render_runs_the_scripts_in_the_sandboxed_host_and_never_in_its_own_process() {
    if let Err(e) = wprs_render::Renderer::shared() {
        if std::env::var_os("WPRS_REQUIRE_GPU").is_some() {
            panic!("GPU required but unavailable: {e}");
        }
        return;
    }
    let home = tempfile::tempdir().unwrap();
    let folder = home.path().join("scene");
    let run = |args: &[&str]| ctl(home.path()).args(args).arg(&folder).output().unwrap();
    assert!(run(&["scene", "new"]).status.success());
    let script = home.path().join("hello.js");
    std::fs::write(&script, "export function update() { console.log('ran in the host'); }\n").unwrap();
    let imported =
        ctl(home.path()).args(["scene", "import-script"]).arg(&folder).arg(&script).output().unwrap();
    assert!(imported.status.success(), "{}", String::from_utf8_lossy(&imported.stderr));
    let out = home.path().join("out.gif");
    let mut render = ctl(home.path())
        .args(["scene", "render"])
        .arg(&folder)
        .arg("-o")
        .arg(&out)
        .args(["--frames", "240", "--fps", "60", "--size", "64"])
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let (pid, ends, mut host) = (render.id(), Instant::now() + Duration::from_secs(60), false);
    while render.try_wait().unwrap().is_none() && Instant::now() < ends {
        host |= children_of(pid).iter().any(|c| c.contains("--script-host"));
        std::thread::sleep(Duration::from_millis(2));
    }
    let done = render.wait_with_output().unwrap();
    let said = String::from_utf8_lossy(&done.stderr);
    assert!(done.status.success(), "{said}");
    assert!(said.contains("script: log scripts/hello.js: ran in the host"), "{said}");
    assert!(host, "the scripts ran but no script host process was a child of the render");
}
