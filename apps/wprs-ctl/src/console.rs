use anyhow::Result;
use serde_json::Value;
use std::collections::BTreeMap;
use wprs_client::Client;

const TIMEOUT_MS: u32 = 20_000;

#[derive(Default)]
pub struct Seen(BTreeMap<String, (usize, Option<String>)>);

pub fn news(state: &Value, output: &str, seen: &mut Seen) -> Vec<(bool, String)> {
    let Some(previews) = state["previews"].as_object() else { return Vec::new() };
    let named = previews.iter().filter(|(name, _)| output.is_empty() || name.as_str() == output);
    let several = named.clone().count() > 1;
    let mut out = Vec::new();
    for (name, preview) in named {
        let (shown, error) = seen.0.entry(name.clone()).or_default();
        let lines = preview["console"].as_array().map(Vec::as_slice).unwrap_or_default();
        let start = if *shown > lines.len() { 0 } else { *shown };
        let tag = |line: &str| if several { format!("{name}: {line}") } else { line.to_owned() };
        out.extend(lines[start..].iter().filter_map(Value::as_str).map(|l| (false, tag(l))));
        *shown = lines.len();
        let now = preview["error"].as_str().map(str::to_owned);
        if now != *error {
            out.extend(now.iter().map(|m| (true, tag(m))));
            *error = now;
        }
    }
    out
}

pub async fn watch(client: &Client, output: &str) -> Result<()> {
    let (mut seen, mut serial) = (Seen::default(), client.daemon.serial().await?);
    loop {
        let state: Value = serde_json::from_str(&client.daemon.get_state().await?)?;
        for (failed, line) in news(&state, output, &mut seen) {
            match failed {
                true => eprintln!("error: {line}"),
                false => println!("{line}"),
            }
        }
        (serial, _) = client.daemon.wait_changed(serial, TIMEOUT_MS).await?;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn lines_and_errors_of_a_preview_on_any_screen_are_printed_once() {
        let mut seen = Seen::default();
        let state = json!({"previews": {"DP-1": {"console": ["log a.js: hi"], "error": null}}});
        assert_eq!(news(&state, "", &mut seen), [(false, "log a.js: hi".to_owned())]);
        assert!(news(&state, "", &mut seen).is_empty());
        let state = json!({"previews": {"DP-1": {"console": ["log a.js: hi", "log a.js: two"], "error": "a.js: boom (a.js line 2)"}}});
        let told = news(&state, "DP-1", &mut seen);
        assert_eq!(
            told,
            [(false, "log a.js: two".to_owned()), (true, "a.js: boom (a.js line 2)".to_owned())]
        );
        assert!(news(&state, "DP-2", &mut Seen::default()).is_empty());
    }

    #[test]
    fn a_restarted_preview_prints_its_new_lines_again() {
        let mut seen = Seen::default();
        let two = json!({"previews": {"DP-1": {"console": ["a", "b"]}}});
        assert_eq!(news(&two, "", &mut seen).len(), 2);
        let one = json!({"previews": {"DP-1": {"console": ["c"]}}});
        assert_eq!(news(&one, "", &mut seen), [(false, "c".to_owned())]);
    }

    #[test]
    fn several_screens_are_told_apart() {
        let state = json!({"previews": {"A": {"console": ["x"]}, "B": {"console": ["y"]}}});
        let told = news(&state, "", &mut Seen::default());
        assert_eq!(told, [(false, "A: x".to_owned()), (false, "B: y".to_owned())]);
    }
}
