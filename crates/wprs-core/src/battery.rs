use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BatteryPause {
    Never,
    #[default]
    Always,
    Below(u8),
}

impl BatteryPause {
    pub const KEY: &str = "pause.on_battery";
    pub const PREFIX: &str = "below:";

    pub fn parse(text: &str) -> Result<Self, String> {
        let text = text.trim();
        match text {
            "never" => Ok(Self::Never),
            "always" => Ok(Self::Always),
            _ => match text.strip_prefix(Self::PREFIX).map(str::parse::<u8>) {
                Some(Ok(n @ 1..=100)) => Ok(Self::Below(n)),
                _ => Err(format!("'{text}' is not never, always or below:N with N from 1 to 100")),
            },
        }
    }

    pub fn word(self) -> String {
        match self {
            Self::Never => "never".into(),
            Self::Always => "always".into(),
            Self::Below(n) => format!("{}{n}", Self::PREFIX),
        }
    }

    pub fn pauses(self, on_battery: bool, percent: Option<u8>) -> bool {
        on_battery
            && match self {
                Self::Never => false,
                Self::Always => true,
                Self::Below(n) => percent.is_some_and(|p| p < n),
            }
    }

    pub fn limit(self) -> u8 {
        match self {
            Self::Never => 0,
            Self::Always => 101,
            Self::Below(n) => n,
        }
    }
}

impl Serialize for BatteryPause {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&self.word())
    }
}

impl<'de> Deserialize<'de> for BatteryPause {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        Self::parse(&String::deserialize(d)?).map_err(serde::de::Error::custom)
    }
}
