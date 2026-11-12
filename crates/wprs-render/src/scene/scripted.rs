use super::SceneAssets;
use serde_json::json;
use wprs_scene::{Course, Fired, LayerRead, MAX_FIRED, Runtime, Scene};
use wprs_script::{Frame, Limits, Mode, Script, Setup};

const WARMUP: usize = 3;

#[derive(Default)]
pub(crate) struct Scripted {
    script: Option<Script>,
    fired: Vec<Fired>,
    ended: Vec<String>,
    courses: Vec<Course>,
    filmed: Vec<Course>,
    reads: Vec<(String, LayerRead)>,
    sounds: Vec<String>,
    pub(crate) console: Vec<String>,
    pub(crate) stopped: Vec<String>,
}

impl Scripted {
    pub(crate) fn start(
        scene: &Scene,
        assets: &SceneAssets,
        runtime: &Runtime,
        locale: &str,
    ) -> Option<Scripted> {
        if scene.scripts.is_empty() || assets.scripts.is_empty() {
            return None;
        }
        let setup = Setup::new(scene, &assets.scripts, runtime.seed, (locale, &json!({})));
        let mut made = Scripted::default();
        match Script::load(&setup, Limits::default()) {
            Ok(script) => made.script = Some(script),
            Err(e) => made.stopped.push(e.to_string()),
        }
        Some(made)
    }

    pub(crate) fn warm(&mut self, runtime: &mut Runtime, shown: ([f32; 4], [u32; 2], u32, u8)) {
        (0..WARMUP).for_each(|_| self.step(runtime, shown));
    }

    pub(crate) fn step(
        &mut self,
        runtime: &mut Runtime,
        (view, screen, fps, quality): ([f32; 4], [u32; 2], u32, u8),
    ) {
        let Some(script) = self.script.as_mut() else { return };
        let room = MAX_FIRED.saturating_sub(self.fired.len());
        self.fired.extend(runtime.take_fired().into_iter().take(room));
        let room = MAX_FIRED.saturating_sub(self.ended.len());
        self.ended.extend(runtime.take_ended().into_iter().take(room));
        runtime.fill_playbacks(&mut self.courses);
        runtime.fill_reads(&mut self.reads);
        runtime.fill_sounds(&mut self.sounds);
        runtime.fill_videos(&mut self.filmed);
        let rt = &*runtime;
        let frame = Frame {
            time: rt.time(),
            dt: rt.dt(),
            wall: rt.wall(),
            props: rt.props(),
            pointer: rt.pointer(),
            view,
            screen,
            mode: Mode::Wallpaper,
            fps,
            quality,
            timelines: &self.courses,
            fired: &self.fired,
            reads: &self.reads,
            sounds: &self.sounds,
            videos: &self.filmed,
            ended: &self.ended,
            ..Frame::at(rt.time())
        };
        let answer = script.update(&frame);
        self.fired.clear();
        self.ended.clear();
        match answer {
            Ok(mut out) => {
                self.console.append(&mut out.console);
                for why in out.stopped.drain(..) {
                    if !self.stopped.contains(&why) {
                        self.stopped.push(why);
                    }
                }
                runtime.set_script(out);
            }
            Err(e) => {
                let why = e.to_string();
                if !self.stopped.contains(&why) {
                    self.stopped.push(why);
                }
            }
        }
    }
}

impl Drop for Scripted {
    fn drop(&mut self) {
        self.script.iter_mut().for_each(Script::destroy);
    }
}
