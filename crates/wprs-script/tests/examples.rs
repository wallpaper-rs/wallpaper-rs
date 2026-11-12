use serde_json::{Value, json};
use std::path::{Path, PathBuf};
use wprs_scene::{AudioFrame, PointerState, Scene, ScriptOutputs, Values};
use wprs_script::{Frame, Limits, Script, Setup};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/crear/ejemplos")
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn defaults(folder: &Path) -> Values {
    let manifest: Value = serde_json::from_str(&read(&folder.join("manifest.json"))).unwrap();
    let props = manifest["properties"].as_object().cloned().unwrap_or_default();
    props.iter().filter_map(|(id, p)| Some((id.clone(), p.get("default")?.clone()))).collect()
}

struct Inputs {
    pointer: PointerState,
    audio: AudioFrame,
    wall: f64,
}

fn quiet() -> Inputs {
    Inputs { pointer: PointerState::default(), audio: AudioFrame::default(), wall: 12.0 * 3600.0 }
}

struct Play {
    script: Script,
    props: Values,
    at: usize,
    console: Vec<String>,
}

impl Play {
    fn new(name: &str) -> Play {
        let folder = root().join(name);
        let scene = Scene::from_json(&read(&folder.join("scene.json"))).unwrap();
        let read_one = |s: &wprs_scene::Script| (s.path.clone(), read(&folder.join(&s.path)));
        let sources: Vec<_> = scene.scripts.iter().map(read_one).collect();
        let setup = Setup::new(&scene, &sources, 7, ("es", &json!({})));
        Play {
            script: Script::load(&setup, Limits::default()).unwrap(),
            props: defaults(&folder),
            at: 0,
            console: Vec::new(),
        }
    }

    fn go(&mut self, inputs: &Inputs, frames: usize) -> ScriptOutputs {
        let mut out = ScriptOutputs::default();
        for _ in 0..frames {
            let frame = Frame {
                dt: 1.0 / 60.0,
                wall: inputs.wall,
                props: &self.props,
                pointer: &inputs.pointer,
                audio: Some(&inputs.audio),
                ..Frame::at(self.at as f64 / 60.0)
            };
            self.at += 1;
            out = self.script.update(&frame).unwrap();
            self.console.extend(out.console.iter().cloned());
        }
        out
    }
}

fn names() -> Vec<String> {
    let mut found: Vec<String> = std::fs::read_dir(root())
        .unwrap()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().join("scene.json").exists())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    found.sort();
    found
}

#[test]
fn every_example_runs_without_errors() {
    let names = names();
    assert!(names.len() >= 6, "{names:?}");
    for name in names {
        let mut play = Play::new(&name);
        let pointer = PointerState { pos: Some([0.8, 0.3]), inside: true, ..PointerState::default() };
        let mut audio = AudioFrame { bass: 0.9, level: 0.7, beat: true, ..AudioFrame::default() };
        audio.average[3] = 0.8;
        for inputs in [quiet(), Inputs { pointer, audio, wall: 23.0 * 3600.0 }] {
            let out = play.go(&inputs, 300);
            assert!(out.stopped.is_empty(), "{name}: {:?}", out.stopped);
            let errors: Vec<_> = play.console.iter().filter(|l| l.starts_with("error")).collect();
            assert!(errors.is_empty(), "{name}: {errors:?}");
        }
        play.script.destroy();
    }
}

#[test]
fn the_clock_writes_the_time_and_the_date() {
    let out = Play::new("reloj").go(&quiet(), 2);
    assert_eq!(out.layers["hora"].text.as_deref(), Some("12:00"));
    let date = out.layers["fecha"].text.clone().unwrap();
    assert!(date.contains(" de "), "{date}");
}

#[test]
fn the_follower_eases_towards_the_pointer() {
    let mut play = Play::new("sigue-puntero");
    let pointer = PointerState { pos: Some([0.75, 0.25]), inside: true, ..PointerState::default() };
    let inputs = Inputs { pointer, ..quiet() };
    let near = play.go(&inputs, 6).layers["cabeza"].position.unwrap();
    assert!(near[0] > 960.0 && near[0] < 1440.0, "{near:?}");
    let late = play.go(&inputs, 300).layers["cabeza"].position.unwrap();
    assert!((late[0] - 1440.0).abs() < 1.0 && (late[1] - 270.0).abs() < 1.0, "{late:?}");
    let tail = play.go(&inputs, 1).layers["cola"].position.unwrap();
    assert!(tail[0] < late[0], "{tail:?} {late:?}");
}

#[test]
fn the_ring_grows_with_the_bass() {
    let mut play = Play::new("reaccion-al-sonido");
    let silent = play.go(&quiet(), 60);
    assert!(silent.layers["pulso"].scale.unwrap()[0] < 1.01);
    let audio = AudioFrame { bass: 1.0, level: 1.0, beat: true, ..AudioFrame::default() };
    let loud = play.go(&Inputs { audio, ..quiet() }, 60);
    assert!(loud.layers["pulso"].scale.unwrap()[0] > 1.7);
    assert!(loud.layers["destello"].opacity.unwrap() > 0.2);
}

#[test]
fn the_setting_and_the_hour_pick_the_group() {
    let mut play = Play::new("grupo-por-ajuste");
    let noon = play.go(&quiet(), 2);
    assert_eq!(noon.layers["dia"].visible, Some(true));
    assert_eq!(noon.layers["noche"].visible, Some(false));
    play.props.insert("momento".into(), json!("noche"));
    let night = play.go(&quiet(), 2);
    assert_eq!(night.layers["dia"].visible, Some(false));
    assert_eq!(night.layers["noche"].visible, Some(true));
}

#[test]
fn one_shape_is_chosen_at_the_start() {
    let mut play = Play::new("azar-al-inicio");
    let out = play.go(&quiet(), 2);
    let shown = ["circulo", "cuadrado", "anillo"].iter().filter(|id| out.layers[**id].visible == Some(true));
    assert_eq!(shown.count(), 1);
    assert!(play.console.iter().any(|l| l.contains("forma elegida")), "{:?}", play.console);
}

#[test]
fn the_traffic_light_walks_its_states() {
    let mut play = Play::new("maquina-estados");
    let lit =
        |out: &ScriptOutputs| ["roja", "ambar", "verde"].map(|id| out.layers[id].opacity.unwrap() > 0.5);
    let mut seen = Vec::new();
    for frames in [1, 240, 240, 90] {
        seen.push(lit(&play.go(&quiet(), frames)));
    }
    assert_eq!(
        seen,
        [[true, false, false], [false, false, true], [false, true, false], [true, false, false]]
    );
}
