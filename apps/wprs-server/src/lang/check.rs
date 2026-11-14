#![cfg(test)]
use super::{Lang, catalogue, count, es, missed_texts, since, t, tf, tm, within, word};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

fn sources(dir: &Path, out: &mut Vec<String>) {
    for entry in fs::read_dir(dir).unwrap().flatten() {
        let path = entry.path();
        if path.is_dir() && path.file_name().is_some_and(|n| n != "es") {
            sources(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") && !path.ends_with("check.rs") {
            out.push(fs::read_to_string(&path).unwrap());
        }
    }
}

fn literal(text: &str) -> Option<(String, &str)> {
    let rest = text.trim_start().strip_prefix('"')?;
    let (mut out, mut chars) = (String::new(), rest.char_indices());
    while let Some((i, c)) = chars.next() {
        match c {
            '"' => return Some((out, &rest[i + 1..])),
            '\\' => match chars.next()?.1 {
                'n' => out.push('\n'),
                '\n' => {
                    while rest[chars.clone().next()?.0..].starts_with([' ', '\n']) {
                        chars.next();
                    }
                }
                other => out.push(other),
            },
            other => out.push(other),
        }
    }
    None
}

fn keys(source: &str) -> Vec<String> {
    let mut found = Vec::new();
    for name in ["t(", "tf(", "tm(", "count("] {
        for (at, _) in source.match_indices(name) {
            let before = source[..at].chars().next_back().unwrap_or(' ');
            if before.is_alphanumeric() || before == '_' || before == '.' || source[..at].ends_with("fn ") {
                continue;
            }
            let mut rest = &source[at + name.len()..];
            if name == "count(" {
                let mut depth = 0;
                let cut = rest.char_indices().find(|(_, c)| {
                    depth += (*c == '(') as i32 - (*c == ')') as i32;
                    *c == ',' && depth == 0
                });
                rest = &rest[cut.map_or(0, |(i, _)| i + 1)..];
            }
            while let Some((text, tail)) = literal(rest) {
                found.push(text);
                rest = tail.trim_start().strip_prefix(',').unwrap_or("");
                if name != "count(" {
                    break;
                }
            }
        }
    }
    found
}

fn used() -> HashSet<String> {
    let mut all = Vec::new();
    sources(&Path::new(env!("CARGO_MANIFEST_DIR")).join("src"), &mut all);
    all.iter().flat_map(|s| keys(s)).collect()
}

#[test]
fn every_text_in_the_code_has_its_spanish() {
    let missing: Vec<_> = used().into_iter().filter(|k| !catalogue().contains_key(k.as_str())).collect();
    assert!(missing.is_empty(), "no Spanish text for: {missing:#?}");
}

#[test]
fn the_catalogue_keeps_nothing_the_code_no_longer_asks_for() {
    let asked = used();
    let stale: Vec<_> = catalogue().keys().filter(|k| !asked.contains(**k)).collect();
    assert!(stale.is_empty(), "unused Spanish texts: {stale:#?}");
}

#[test]
fn each_text_is_listed_once_and_keeps_its_placeholders() {
    let all: Vec<_> = es::TEXTS.iter().flat_map(|p| p.iter()).collect();
    assert_eq!(all.len(), catalogue().len(), "a text is listed twice");
    for (en, es) in all {
        let (a, b) = (en.matches('{').count(), es.matches('{').count());
        assert_eq!(a, b, "placeholders differ: {en}");
        assert!(!es.is_empty(), "{en}");
    }
}

#[test]
fn a_missing_text_is_recorded_and_stays_english() {
    let shown = within(Lang::Es, || t("this text has no Spanish"));
    assert_eq!(shown, "this text has no Spanish");
    assert!(missed_texts().contains(&"this text has no Spanish"));
    assert_eq!(within(Lang::En, || t("Sign in")), "Sign in");
    assert_eq!(within(Lang::Es, || t("Sign in")), "Entrar");
}

#[test]
fn language_follows_the_preference_order() {
    let pick = |h: &str| Lang::from_header(Some(h));
    assert_eq!(pick("es-AR,es;q=0.9,en;q=0.5"), Lang::Es);
    assert_eq!(pick("en-US,en;q=0.9,es;q=0.8"), Lang::En);
    assert_eq!(pick("fr;q=0.9, es;q=0.4"), Lang::En);
    assert_eq!(pick("ES"), Lang::Es);
    assert_eq!(pick("es;q=0"), Lang::En);
    assert_eq!(pick("*"), Lang::En);
    assert_eq!(Lang::from_header(None), Lang::En);
}

#[test]
fn placeholders_fill_in_both_orders_and_escape_markup() {
    within(Lang::Es, || {
        assert_eq!(tf("Account ({})", &[&"ana"]), "Cuenta (ana)");
        let shown = tm(
            "By creating the account you accept the {}. Read the {} too.",
            &[maud::html! { b { "a<b" } }, maud::html! { "admin" }],
        );
        assert_eq!(
            shown.into_string(),
            "Al crear la cuenta aceptas los <b>a&lt;b</b>. Lee también el admin."
        );
    });
}

#[test]
fn counts_and_times_read_well_in_both_languages() {
    let es = |f: fn() -> String| within(Lang::Es, f);
    let en = |f: fn() -> String| within(Lang::En, f);
    assert_eq!(en(|| count(1, "{} day ago", "{} days ago")), "1 day ago");
    assert_eq!(en(|| count(3, "{} day ago", "{} days ago")), "3 days ago");
    assert_eq!(es(|| count(1, "{} day ago", "{} days ago")), "hace 1 día");
    assert_eq!(es(|| count(0, "{} day ago", "{} days ago")), "hace 0 días");
    assert_eq!(en(|| since(10)), "just now");
    assert_eq!(es(|| since(10)), "ahora mismo");
    assert_eq!(en(|| since(180)), "3 min ago");
    assert_eq!(es(|| since(180)), "hace 3 min");
    assert_eq!(es(|| since(7200)), "hace 2 h");
    assert_eq!(es(|| since(86400 * 3)), "hace 3 días");
    assert_eq!(en(|| since(-86400 * 89)), "in 89 days");
    assert_eq!(es(|| since(-86400)), "en 1 día");
    assert_eq!(es(|| since(-600)), "en 10 min");
}

#[test]
fn the_roles_and_states_have_words() {
    for w in crate::auth::ROLES {
        assert!(within(Lang::Es, || word(w)) != w, "{w}");
    }
}

#[test]
fn every_state_the_review_pages_show_has_a_spanish_word() {
    let shown = [
        ["pending", "published", "rejected", "removed", "revoked"],
        ["open", "closed", "actioned", "dismissed", "withdrawn"],
        ["upload", "report", "notice", "counter", "approve"],
        ["reject", "remove", "revoke", "restore", "dismiss"],
        ["image", "video", "scene", "web", "general"],
        ["mature", "unrated", "copyright", "illegal", "malware"],
        ["spam", "other", "license", "takedown", "broken"],
        ["received", "done", "failed", "public", "unlisted"],
    ];
    let known = super::vocabulary();
    let missing: Vec<&str> = shown.iter().flatten().copied().filter(|w| !known.contains_key(w)).collect();
    assert!(missing.is_empty(), "no Spanish word for: {missing:?}");
}

#[test]
fn every_permission_the_validator_knows_has_a_spanish_word() {
    let known = super::vocabulary();
    let names = wprs_format::Input::ALL.iter().map(|i| i.name()).chain(["rgb"]);
    let missing: Vec<&str> = names.filter(|n| !known.contains_key(format!("asks {n}").as_str())).collect();
    assert!(missing.is_empty(), "no Spanish word for permission: {missing:?}");
}

#[test]
fn every_month_a_certificate_date_shows_has_a_spanish_word() {
    let known = super::vocabulary();
    let missing: Vec<&str> =
        crate::certs::MONTHS.iter().copied().filter(|m| !known.contains_key(m)).collect();
    assert!(missing.is_empty(), "no Spanish word for: {missing:?}");
}
