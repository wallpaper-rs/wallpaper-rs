use super::helper::{Page, available, with};
use std::time::{Duration, Instant};
use wprs_web::Isolation;

const ISOLATION: Isolation = Isolation { network: false, devices: false, limits: false };

#[test]
fn a_page_that_throws_at_start_reports_the_error_to_the_daemon() {
    if !available() {
        return eprintln!("skipped: no WPE WebKit 2.54");
    }
    let dir = tempfile::tempdir().unwrap();
    let script: &[u8] = b"throw new Error('boom at start');";
    let mut page = Page::open(with(dir.path(), 0, &[("probe.js", script)]), ISOLATION);
    let end = Instant::now() + Duration::from_secs(30);
    while Instant::now() < end && page.problems.is_empty() {
        page.pump(Instant::now() + Duration::from_millis(50));
    }
    assert!(page.problems.iter().any(|l| l.contains("boom at start")), "{:?}", page.problems);
}

#[test]
fn the_locale_a_page_gets_is_a_language_tag_the_engine_accepts() {
    if !available() {
        return eprintln!("skipped: no WPE WebKit 2.54");
    }
    let dir = tempfile::tempdir().unwrap();
    let script: &[u8] = b"const report = (n, v) => console.log(`probe ${n} ${JSON.stringify(v)}`);\n\
        wprs.onSettings((s) => { try { report('intl', new Intl.DateTimeFormat(s.locale).resolvedOptions().locale); } \
        catch (e) { report('intl', String(e)); } });";
    let mut start = with(dir.path(), 0, &[("probe.js", script)]);
    start.settings.locale = "es-AR".into();
    let mut page = Page::open(start, ISOLATION);
    assert_eq!(page.wait("intl", 30), "es-AR");
    assert!(page.problems.is_empty(), "{:?}", page.problems);
}
