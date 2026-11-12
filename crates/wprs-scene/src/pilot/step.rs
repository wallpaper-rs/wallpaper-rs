use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Step {
    #[default]
    Model,
    Steady,
    Climbed,
    Dropped,
    Top,
    Held,
    Still,
    Halved,
    Doubled,
}

impl Step {
    pub fn name(self) -> &'static str {
        ["model", "steady", "climbed", "dropped", "top", "held", "still", "halved", "doubled"][self as usize]
    }
}

pub fn lowers(over: bool, timing: Option<(f32, f32)>) -> bool {
    timing.map_or(over, |(frame, period)| frame > period * super::DROP)
}
