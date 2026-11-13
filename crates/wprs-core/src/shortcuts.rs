use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Shortcuts {
    pub enabled: bool,
    pub next: String,
    pub previous: String,
}

impl Default for Shortcuts {
    fn default() -> Self {
        Self { enabled: false, next: "LOGO+ALT+N".into(), previous: "LOGO+ALT+P".into() }
    }
}

impl Shortcuts {
    pub const MAX_LEN: usize = 64;

    pub fn trigger(text: &str) -> Option<&str> {
        let text = text.trim();
        let part = |p: &str| !p.is_empty() && p.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
        (text.len() <= Self::MAX_LEN && text.split('+').all(part)).then_some(text)
    }
}

#[cfg(test)]
mod tests {
    use super::Shortcuts;

    #[test]
    fn only_plain_key_combinations_are_proposed() {
        assert_eq!(Shortcuts::trigger(" CTRL+ALT+Page_Down "), Some("CTRL+ALT+Page_Down"));
        assert_eq!(Shortcuts::trigger(&Shortcuts::default().next), Some("LOGO+ALT+N"));
        for bad in ["", "CTRL+", "CTRL++N", "CTRL+ALT+N;rm", &"A+".repeat(40)] {
            assert_eq!(Shortcuts::trigger(bad), None, "{bad}");
        }
    }
}
