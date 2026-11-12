use crate::shards::{Census, claim_in};

fn census(lines: &[&str]) -> Census {
    let mut census = Census::default();
    lines.iter().for_each(|line| census.note(line));
    census
}

#[test]
fn a_unit_run_once_and_seen_by_the_others_is_complete() {
    let census = census(&[
        "wprs-check ran scene.empty",
        "wprs-check seen scene.empty",
        "wprs-check seen scene.clock",
        "wprs-check ran scene.clock",
        "a line that is not a mark",
    ]);
    assert!(census.problems().is_empty());
    assert_eq!(census.units(), 2);
}

#[test]
fn a_unit_nobody_ran_is_reported() {
    let census = census(&["wprs-check seen repo", "wprs-check seen repo"]);
    assert_eq!(census.problems(), ["no process ran the check unit repo"]);
}

#[test]
fn a_unit_run_twice_is_reported() {
    let census = census(&["wprs-check ran image", "wprs-check ran image"]);
    assert_eq!(census.problems(), ["two processes ran the check unit image"]);
}

#[test]
fn only_the_first_claim_of_a_unit_wins() {
    let dir = std::env::temp_dir().join(format!("wprs-app-claims-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    assert_eq!(claim_in(Some(&dir), "scene.empty"), Some(true));
    assert_eq!(claim_in(Some(&dir), "scene.empty"), Some(false));
    assert_eq!(claim_in(Some(&dir), "scene.clock"), Some(true));
    assert_eq!(claim_in(None, "scene.empty"), None);
    std::fs::remove_dir_all(&dir).unwrap();
}
