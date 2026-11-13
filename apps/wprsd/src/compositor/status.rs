#[derive(Debug, Clone, Default, PartialEq, serde::Serialize)]
pub struct OutputStatus {
    pub kind: &'static str,
    pub playing: bool,
    pub error: Option<String>,
    pub parallax: bool,
    pub frames: u64,
    pub page_frames: u64,
    pub fps: u32,
    pub sound: &'static str,
    pub flashes: &'static str,
    pub particles: &'static str,
    pub level: Level,
    pub decode: &'static str,
    pub left_out: Vec<String>,
    pub console: Vec<String>,
    pub problems: Vec<String>,
    pub software_still: bool,
    pub package: Option<std::path::PathBuf>,
    pub sandbox: &'static str,
    pub helper: Option<u32>,
    pub hdr: super::light::Hdr,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, serde::Serialize)]
pub struct Level {
    pub quality: &'static str,
    pub reason: &'static str,
    pub cost: f32,
    pub budget: f32,
    pub automatic: &'static str,
    pub frame_ms: Option<f32>,
    pub period_ms: Option<f32>,
    pub step: &'static str,
    pub pace: u32,
}

impl Level {
    pub fn timing(&self) -> wprs_core::model::QualityTiming {
        let tenths = |ms: Option<f32>| ms.map(|ms| f64::from((ms * 10.0).round() / 10.0));
        wprs_core::model::QualityTiming {
            quality_frame_ms: tenths(self.frame_ms),
            quality_period_ms: tenths(self.period_ms),
            quality_step: self.step.into(),
            quality_pace: self.pace,
        }
    }
}

impl OutputStatus {
    pub fn shown(&self) -> wprs_core::model::Shown {
        use wprs_core::model::Shown;
        match self.kind {
            "image" => Shown::Image,
            "video" => Shown::Video,
            "scene" => Shown::Scene,
            "web" => Shown::Web,
            _ => Shown::Color,
        }
    }
}

pub(super) fn problems_changed(
    now: &std::collections::BTreeMap<String, OutputStatus>,
    old: &std::collections::BTreeMap<String, OutputStatus>,
    names: &[String],
) -> bool {
    let problems = |all: &std::collections::BTreeMap<String, OutputStatus>, n: &String| {
        all.get(n).map(|s| s.problems.clone())
    };
    names.iter().any(|n| problems(now, n) != problems(old, n))
}
