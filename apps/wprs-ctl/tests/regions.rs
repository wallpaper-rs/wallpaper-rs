use image::{GrayImage, Luma, Rgba, RgbaImage};
use serde_json::{Value, json};
use std::path::Path;
use std::process::{Command, Output, Stdio};

fn run(dir: &Path, args: &[&str]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_wprs-ctl"));
    command.args(args).current_dir(dir).stdin(Stdio::null()).env("LC_ALL", "C").env_remove("LANGUAGE");
    for (key, sub) in [
        ("XDG_DATA_HOME", "data"),
        ("XDG_CACHE_HOME", "cache"),
        ("XDG_CONFIG_HOME", "config"),
        ("XDG_STATE_HOME", "state"),
    ] {
        command.env(key, dir.join(sub));
    }
    command.env("DBUS_SESSION_BUS_ADDRESS", format!("unix:path={}/no-bus", dir.display()));
    command.output().unwrap()
}

fn ok(dir: &Path, args: &[&str]) -> String {
    let out = run(dir, args);
    assert!(out.status.success(), "{args:?}: {}", String::from_utf8_lossy(&out.stderr));
    String::from_utf8(out.stdout).unwrap()
}

fn fails(dir: &Path, args: &[&str], code: i32) -> String {
    let out = run(dir, args);
    assert_eq!(out.status.code(), Some(code), "{args:?}: {}", String::from_utf8_lossy(&out.stdout));
    String::from_utf8(out.stderr).unwrap()
}

fn zone(x: u32, y: u32) -> u8 {
    match (x, y) {
        (10..30, 10..40) => 230,
        (45..70, 15..35) => 140,
        _ => 20,
    }
}

fn shown(dir: &Path) -> Value {
    serde_json::from_str(&ok(dir, &["edit", "show", "park.png"])).unwrap()
}

fn listed(dir: &Path) -> Vec<Value> {
    serde_json::from_str(&ok(dir, &["edit", "depth", "regions", "park.png", "--json"])).unwrap()
}

#[test]
fn regions_are_listed_joined_split_and_moved_from_the_command_line() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path();
    RgbaImage::from_fn(80, 50, |x, y| Rgba([zone(x, y), 90, 120, 255])).save(dir.join("park.png")).unwrap();
    GrayImage::from_fn(80, 50, |x, y| Luma([zone(x, y)])).save(dir.join("park.png.depth.png")).unwrap();
    let text = ok(dir, &["edit", "depth", "regions", "park.png", "--png", "regions.png"]);
    assert_eq!(text.lines().count(), 3, "{text}");
    assert!(text.starts_with("1  depth 0.9"), "{text}");
    assert_eq!(image::open(dir.join("regions.png")).unwrap().width(), 80);
    let zones = listed(dir);
    let at =
        |k: usize| zones[k]["at"].as_array().unwrap().iter().map(|v| v.as_f64().unwrap()).collect::<Vec<_>>();
    let (near, mid) = (format!("{},{}", at(0)[0], at(0)[1]), format!("{},{}", at(1)[0], at(1)[1]));
    ok(dir, &["edit", "depth", "join", "park.png", &format!("{near};{mid}")]);
    assert_eq!(shown(dir)["depth"]["joins"].as_array().map(Vec::len), Some(1));
    assert_eq!(listed(dir).len(), 2);
    ok(dir, &["edit", "undo", "park.png"]);
    assert_eq!(listed(dir).len(), 3);
    ok(dir, &["edit", "depth", "split", "park.png", "0.0,0.5;1.0,0.5", "--radius", "0.02"]);
    assert!(listed(dir).len() > 3);
    ok(dir, &["edit", "depth", "split", "park.png", "--clear"]);
    ok(dir, &["edit", "depth", "move", "park.png", &near, "--by", "-0.7"]);
    assert_eq!(shown(dir)["depth"]["moves"][0]["by"], json!(-0.7));
    assert!(listed(dir)[0]["depth"].as_f64().unwrap() < 0.6);
    ok(dir, &["edit", "depth", "move", "park.png", "--clear"]);
    assert!((listed(dir)[0]["depth"].as_f64().unwrap() - 0.9).abs() < 0.05);
    ok(dir, &["edit", "depth", "move", "park.png", &near, "--by", "-0.2"]);
    ok(dir, &["edit", "depth", "move", "park.png", &near, "--by", "0.2"]);
    let out = run(dir, &["edit", "depth", "regions", "park.png"]);
    let said = String::from_utf8_lossy(&out.stderr);
    assert!(out.status.success() && said.contains("move 2 is not applied"), "{said}");
    assert!(said.contains("clear the moves"), "{said}");
    ok(dir, &["edit", "depth", "move", "park.png", "--clear"]);
    assert!(run(dir, &["edit", "depth", "regions", "park.png"]).stderr.is_empty());
    let said = fails(dir, &["edit", "depth", "join", "park.png", "0.1,0.1"], 2);
    assert!(said.contains("2 to 8"), "{said}");
    assert!(fails(dir, &["edit", "depth", "move", "park.png", &near], 2).contains("by"));
    assert!(fails(dir, &["edit", "depth", "split", "park.png"], 2).contains("give a stroke"));
}
