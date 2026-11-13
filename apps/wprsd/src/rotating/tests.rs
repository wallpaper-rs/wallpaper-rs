use super::*;

#[test]
fn a_followed_folder_becomes_a_query_of_its_pictures_and_videos() {
    let query = follows(Path::new("/pictures/Sea trip")).unwrap();
    assert_eq!(query, "folder:\"/pictures/Sea trip\" -kind:package");
    assert!(follows(Path::new("/pictures/\"odd\"")).is_err());
}

#[test]
fn a_rotation_name_is_never_one_that_exists() {
    let names = ["Sea".to_owned(), "sea 2".to_owned()];
    assert_eq!(taken(&names, "Sky"), "Sky");
    assert_eq!(taken(&names, "sea"), "sea 3");
}

#[test]
fn a_rotation_needs_a_time_longer_than_zero() {
    assert!(set_rule(&Rules::default(), "interval", "0s").unwrap().interval.is_zero());
    assert_eq!(set_rule(&Rules::default(), "interval", "10m").unwrap().interval.as_secs(), 600);
}
