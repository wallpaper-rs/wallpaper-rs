use super::{t, tf};

pub fn count(n: i64, one: &'static str, other: &'static str) -> String {
    tf(if n == 1 { one } else { other }, &[&n])
}

pub fn since(seconds: i64) -> String {
    let (past, span) = (seconds >= 0, seconds.abs());
    if span < 45 {
        return t("just now").to_string();
    }
    let (minutes, hours, days) = ((span + 30) / 60, (span + 1800) / 3600, (span + 43200) / 86400);
    match (past, span) {
        (true, ..3600) => tf("{} min ago", &[&minutes]),
        (true, ..86400) => tf("{} h ago", &[&hours]),
        (true, _) => count(days, "{} day ago", "{} days ago"),
        (false, ..3600) => tf("in {} min", &[&minutes]),
        (false, ..86400) => tf("in {} h", &[&hours]),
        (false, _) => count(days, "in {} day", "in {} days"),
    }
}
