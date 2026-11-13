use std::fs;
use tempfile::TempDir;
use wprs_library::{Error, Library};

const NOW: i64 = 1_700_000_000;

#[test]
fn a_wallpaper_keeps_its_own_sound_through_rescans_and_changes_one_part_at_a_time() {
    let dir = TempDir::new().unwrap();
    let mut library = Library::open(dir.path().join("library.db")).unwrap();
    let root = dir.path().join("pictures");
    fs::create_dir_all(&root).unwrap();
    ["a.mp4", "b.mp4"].iter().for_each(|name| fs::write(root.join(name), name).unwrap());
    let source = library.add_folder(&root, true).unwrap();
    library.scan(&source, NOW).unwrap();
    let items = library.search("sort:title", NOW, 0, 10).unwrap();
    let (a, b) = (items[0].uid.clone(), items[1].uid.clone());
    assert_eq!((items[0].sound_muted, items[0].sound_volume), (None, None));
    library.set_sound_many(&[&a], Some(Some(true)), Some(Some(40))).unwrap();
    assert_eq!(
        (library.item(&a).unwrap().sound_muted, library.item(&a).unwrap().sound_volume),
        (Some(true), Some(40))
    );
    library.set_sound_many(&[&a], Some(Some(false)), None).unwrap();
    assert_eq!(library.item(&a).unwrap().sound_volume, Some(40), "keeping a part changed it");
    assert_eq!(library.item(&b).unwrap().sound_muted, None, "another wallpaper took the sound");
    fs::write(root.join("a.mp4"), "a changed").unwrap();
    library.scan(&source, NOW + 10).unwrap();
    assert_eq!(library.item(&a).unwrap().sound_muted, Some(false), "a rescan forgot the sound");
    assert!(matches!(library.set_sound_many(&[&a], None, Some(Some(101))), Err(Error::Invalid("volume", _))));
    library.set_sound_many(&[&a], Some(None), Some(None)).unwrap();
    assert_eq!((library.item(&a).unwrap().sound_muted, library.item(&a).unwrap().sound_volume), (None, None));
}
