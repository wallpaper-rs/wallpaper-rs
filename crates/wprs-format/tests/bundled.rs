mod fixture;

use fixture::{DAWN, DAY, DUSK, NOW, Repo};
use wprs_format::{FileFetch, add_repo, browse, bundled, repos};

#[test]
fn a_bundled_repository_is_added_updated_and_renewed_without_a_key_in_the_source() {
    let work = Repo::new();
    let (repo, keys, data) =
        (work.tmp.path().join("bundle"), work.tmp.path().join("keys"), work.data.clone());
    let dawn = work.package(DAWN, "1.0.0", None);
    let first = bundled(&repo, &keys, "Examples", std::slice::from_ref(&dawn), NOW).unwrap();
    assert!(first.changed && first.url.starts_with("file://") && first.url.contains(&first.repo_id));
    let info = add_repo(&data, &FileFetch, &first.url, NOW).unwrap();
    assert_eq!(
        (info.name.as_str(), info.packages, info.repo_id.as_str()),
        ("Examples", 1, first.repo_id.as_str())
    );

    let same = bundled(&repo, &keys, "Examples", std::slice::from_ref(&dawn), NOW + DAY).unwrap();
    assert!(!same.changed && same.url == first.url);

    let dusk = work.package(DUSK, "1.0.0", None);
    let more = bundled(&repo, &keys, "Examples", &[dawn.clone(), dusk], NOW + 2 * DAY).unwrap();
    assert!(more.changed && more.repo_id == first.repo_id);
    wprs_format::update_repo(&data, &FileFetch, &first.repo_id, NOW + 2 * DAY).unwrap();
    assert_eq!(browse(&data, "").unwrap().len(), 2);

    let later = NOW + 60 * DAY;
    let renewed = bundled(&repo, &keys, "Examples", std::slice::from_ref(&dawn), later).unwrap();
    assert!(renewed.changed);
    wprs_format::update_repo(&data, &FileFetch, &first.repo_id, later).unwrap();
    assert!(repos(&data, later).unwrap().iter().all(|r| !r.stale && r.broken.is_none()));

    let years = NOW + 1000 * DAY;
    let rotated = bundled(&repo, &keys, "Examples", &[dawn], years).unwrap();
    assert!(rotated.changed && rotated.repo_id == first.repo_id);
    wprs_format::update_repo(&data, &FileFetch, &first.repo_id, years).unwrap();
    assert!(repos(&data, years).unwrap().iter().all(|r| !r.stale && r.broken.is_none()));
}

#[test]
fn the_example_scenes_make_a_repository_that_installs_them() {
    let tmp = tempfile::tempdir().unwrap();
    let scenes = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scenes");
    let mut wpaks = Vec::new();
    for entry in
        std::fs::read_dir(scenes).unwrap().flatten().filter(|e| e.path().join("manifest.json").is_file())
    {
        let out = tmp.path().join(format!("{}.wpak", entry.file_name().to_string_lossy()));
        wprs_format::build(entry.path(), &out, &wprs_format::Limits::default()).unwrap();
        wpaks.push(out);
    }
    assert!(wpaks.len() >= 6);
    let made = bundled(&tmp.path().join("repo"), &tmp.path().join("keys"), "Examples", &wpaks, NOW).unwrap();
    let data = tmp.path().join("data");
    let info = add_repo(&data, &FileFetch, &made.url, NOW).unwrap();
    assert_eq!(info.packages, wpaks.len());
    assert_eq!(browse(&data, "").unwrap().len(), wpaks.len());
}
