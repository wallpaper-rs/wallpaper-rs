mod fixture;

use fixture::{DAWN, NOW, Repo};
use std::cell::Cell;
use std::fs;
use std::io;
use wprs_format::{Fetch, FileFetch, add_repo, install, installed, sha256_file};

struct Count(Cell<usize>);

impl Fetch for Count {
    fn get(&self, base: &str, path: &str, max: u64) -> io::Result<Option<Vec<u8>>> {
        self.0.set(self.0.get() + usize::from(path.starts_with("packages/")));
        FileFetch.get(base, path, max)
    }
}

#[test]
fn reinstalling_a_damaged_version_fetches_it_again_and_a_whole_one_does_not() {
    let repo = Repo::new();
    repo.add(DAWN, "1.0.0");
    repo.publish(NOW);
    let repo_id = add_repo(&repo.data, &FileFetch, &repo.url(), NOW).unwrap().repo_id;
    let spec = format!("{repo_id}/{DAWN}@1.0.0");
    let fetch = Count(Cell::new(0));
    let first = install(&repo.data, &fetch, &spec, NOW, false, false).unwrap();
    assert_eq!(fetch.0.get(), 1);
    assert_eq!(install(&repo.data, &fetch, &spec, NOW, false, false).unwrap(), first);
    assert_eq!(fetch.0.get(), 1, "a whole file is not fetched again");
    let whole = || sha256_file(&first.path).is_ok_and(|h| h == first.sha256);
    let cut = fs::read(&first.path).unwrap();
    fs::write(&first.path, &cut[..cut.len() / 2]).unwrap();
    assert_eq!(install(&repo.data, &fetch, &spec, NOW, false, false).unwrap(), first);
    assert!(fetch.0.get() == 2 && whole(), "a truncated file is repaired");
    fs::remove_file(&first.path).unwrap();
    assert_eq!(install(&repo.data, &fetch, DAWN, NOW, false, false).unwrap(), first);
    assert!(fetch.0.get() == 3 && whole(), "a missing file is repaired");
    assert_eq!(installed(&repo.data).unwrap(), [first]);
}
