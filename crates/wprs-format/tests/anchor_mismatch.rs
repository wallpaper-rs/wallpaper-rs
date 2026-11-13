mod fixture;

use fixture::{NOW, Repo};
use wprs_format::{Error, FileFetch, add_repo};

#[test]
fn a_wrong_fingerprint_says_it_does_not_match_the_repository() {
    let repo = Repo::with(|_, _| {});
    repo.publish(NOW);
    let right = repo.url();
    let (base, print) = right.split_once("?fingerprint=").unwrap();
    let last = print.chars().last().unwrap();
    let flipped = if last == '0' { '1' } else { '0' };
    let wrong = format!("{base}?fingerprint={}{flipped}", &print[..print.len() - 1]);
    let refused = add_repo(&repo.data, &FileFetch, &wrong, NOW).unwrap_err();
    assert!(matches!(refused, Error::Unanchored(_)), "{refused}");
    assert!(refused.to_string().contains("does not match the repository at"), "{refused}");
    assert!(add_repo(&repo.data, &FileFetch, &right, NOW).is_ok());
}
