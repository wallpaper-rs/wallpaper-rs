use crate::lang::{t, tf};
use crate::ui::Tone;
use serde_json::Value;

pub struct Check {
    pub label: &'static str,
    pub tone: Tone,
    pub text: String,
}

pub fn accepted(allowed: &[String], license: &str) -> bool {
    let mut ids =
        license.split([' ', '(', ')']).filter(|t| !t.is_empty() && !["AND", "OR", "WITH"].contains(t));
    ids.all(|l| allowed.contains(&l.to_string()))
}

fn asks(case: &Value) -> Vec<String> {
    let list: Vec<String> = serde_json::from_str(case["asks"].as_str().unwrap_or("[]")).unwrap_or_default();
    list
}

pub fn checks(case: &Value, allowed: &[String]) -> Vec<Check> {
    let license = case["license"].as_str().unwrap_or_default();
    let who = case["uploader"].as_str().unwrap_or_default();
    let license_check = match accepted(allowed, license) {
        true => (Tone::Good, tf("{}, accepted here", &[&license])),
        false => (Tone::Bad, tf("{}: not accepted here", &[&license])),
    };
    let signature = match case["signer"].is_null() {
        false => (Tone::Good, tf("Matches a key of {}", &[&who])),
        true => (Tone::Warn, t("Not signed").to_string()),
    };
    let asked = asks(case);
    let permissions = match asked.is_empty() {
        true => (Tone::Good, t("None: it uses no network and no microphone").to_string()),
        false => (Tone::Warn, tf("Asks for: {}", &[&asked.join(", ")])),
    };
    let flashing = match case["photosensitive"].as_bool() {
        Some(false) => (Tone::Good, t("It has none").to_string()),
        Some(true) => (Tone::Warn, t("It has flashing lights").to_string()),
        None => (Tone::Warn, t("The author did not say").to_string()),
    };
    let rating = match case["content"].as_str() {
        Some("general") => (Tone::Good, t("For everyone").to_string()),
        Some(_) => (Tone::Warn, t("Mature content").to_string()),
        None => (Tone::Warn, t("Not rated").to_string()),
    };
    [
        (t("License"), license_check),
        (t("Signature"), signature),
        (t("Permissions"), permissions),
        (t("Flashes"), flashing),
        (t("Rating"), rating),
    ]
    .into_iter()
    .map(|(label, (tone, text))| Check { label, tone, text })
    .collect()
}

pub fn verdict(checks: &[Check]) -> (Tone, String) {
    let worst = |tone| checks.iter().find(|c| c.tone == tone);
    match worst(Tone::Bad).or_else(|| worst(Tone::Warn)) {
        Some(c) => (c.tone, c.text.clone()),
        None => (Tone::Good, t("All in order").to_string()),
    }
}
