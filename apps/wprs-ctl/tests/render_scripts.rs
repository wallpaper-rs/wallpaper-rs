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

fn lamp_scene(home: &Path, source: &str) -> std::path::PathBuf {
    let folder = home.join("lamp");
    assert!(ctl(home).args(["scene", "new"]).arg(&folder).output().unwrap().status.success());
    let mut doc: serde_json::Value =
        serde_json::from_slice(&std::fs::read(folder.join("scene.json")).unwrap()).unwrap();
    doc["canvas"]["size"] = serde_json::json!([32, 18]);
    doc["layers"] =
        serde_json::json!([{ "id": "lamp", "type": "solid", "color": "#ffffff", "visible": false }]);
    let script =
        serde_json::json!({ "path": "scripts/lamp.js", "events": ["update"], "uses": ["layers", "media"] });
    doc["scripts"] = serde_json::json!([script]);
    std::fs::write(folder.join("scene.json"), doc.to_string()).unwrap();
    std::fs::create_dir_all(folder.join("scripts")).unwrap();
    std::fs::write(folder.join("scripts/lamp.js"), source).unwrap();
    folder
}

fn lit(home: &Path, folder: &Path, extra: &[&str]) -> bool {
    let out = home.join("lit.png");
    let done = ctl(home)
        .args(["scene", "render"])
        .arg(folder)
        .arg("-o")
        .arg(&out)
        .args(["--size", "32", "--time-of-day", "15:30"])
        .args(extra)
        .output()
        .unwrap();
    assert!(done.status.success(), "{}", String::from_utf8_lossy(&done.stderr));
    image::open(&out).unwrap().to_rgba8().get_pixel(16, 9).0[0] > 200
}

fn has_gpu() -> bool {
    wprs_render::Renderer::shared().is_ok() || std::env::var_os("WPRS_REQUIRE_GPU").is_some()
}

#[test]
fn a_render_gives_the_scripts_the_clock_and_the_made_up_media_player() {
    if !has_gpu() {
        return;
    }
    let home = tempfile::tempdir().unwrap();
    let source = "import { scene, time, media } from 'wprs';
export function update() {
  scene.layer('lamp').visible = time.format('HH') === '15' && media.title === 'Night Drive' && media.playing;
}";
    let folder = lamp_scene(home.path(), source);
    let track = home.path().join("track.json");
    std::fs::write(&track, r#"{ "title": "Night Drive", "playing": true, "duration": 200 }"#).unwrap();
    let track = track.to_str().unwrap();
    assert!(!lit(home.path(), &folder, &[]), "the lamp lit without a media player");
    assert!(lit(home.path(), &folder, &["--media", track]), "the made-up player did not reach the script");
}

#[test]
fn the_captured_thumbnail_of_a_scene_is_drawn_by_its_scripts() {
    if !has_gpu() {
        return;
    }
    let home = tempfile::tempdir().unwrap();
    let source = "import { scene } from 'wprs';
export function update() { scene.layer('lamp').visible = true; }";
    let folder = lamp_scene(home.path(), source);
    let done = ctl(home.path()).args(["scene", "thumbnail"]).arg(&folder).arg("--capture").output().unwrap();
    assert!(done.status.success(), "{}", String::from_utf8_lossy(&done.stderr));
    let kept = std::fs::read_dir(folder.join("assets")).unwrap().flatten().map(|e| e.path());
    let kept = kept.into_iter().find(|p| p.file_stem().is_some_and(|s| s == "preview")).unwrap();
    let picture = image::open(kept).unwrap().to_rgba8();
    let (w, h) = picture.dimensions();
    assert!(picture.get_pixel(w / 2, h / 2).0[0] > 200, "the thumbnail was drawn without the script");
}
