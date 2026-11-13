use super::*;
use std::os::unix::fs::PermissionsExt;

fn rings(bus: &zbus::Connection, home: &Arc<Home>) -> [Keyring; 3] {
    let providers = Keyring::new(bus.clone()).with_home(home.clone());
    let server = Keyring::server(bus.clone(), home.clone());
    let properties = providers.properties();
    [providers, server, properties]
}

fn all(rings: &[Keyring; 3]) -> [&Keyring; 3] {
    [&rings[0], &rings[1], &rings[2]]
}

async fn same(keyring: &Keyring, id: &str, value: &str) {
    assert_eq!(secret(keyring, id, false).await.as_deref(), Some(value), "{id}");
}

#[tokio::test]
async fn every_secret_moves_to_the_private_file_and_back() {
    let bus = Bus::start();
    let vault = Shared::default();
    let _service = serve(&bus, &vault).await;
    let dir = tempfile::tempdir().unwrap();
    let home = Arc::new(Home::at(dir.path().join(NAME)));
    let connection = bus.connect().await;
    let rings = rings(&connection, &home);
    rings[0].store("photos", "one").await.unwrap();
    rings[1].store("server", "two").await.unwrap();
    rings[2].store("pkg/key", "three").await.unwrap();
    assert!(!home.in_file());

    relocate(&all(&rings), true).await.unwrap();
    assert!(home.in_file() && vault.lock().unwrap().items.is_empty());
    let mode = std::fs::metadata(dir.path().join(NAME)).unwrap().permissions().mode();
    assert_eq!(mode & 0o777, 0o600);
    same(&rings[0], "photos", "one").await;
    same(&rings[1], "server", "two").await;
    same(&rings[2], "pkg/key", "three").await;
    assert!(rings[0].has("photos").await.unwrap() && rings[0].reachable("x").await);
    rings[0].store("album", "four").await.unwrap();
    assert!(vault.lock().unwrap().items.is_empty());

    relocate(&all(&rings), false).await.unwrap();
    assert!(!home.in_file() && !dir.path().join(NAME).exists());
    assert_eq!(vault.lock().unwrap().items.len(), 4);
    same(&rings[0], "photos", "one").await;
    same(&rings[0], "album", "four").await;
    same(&rings[1], "server", "two").await;
    same(&rings[2], "pkg/key", "three").await;
    assert!(relocate(&all(&rings), false).await.is_ok());
}

#[tokio::test]
async fn without_a_keyring_the_file_can_be_chosen_but_not_left() {
    let bus = Bus::start();
    let dir = tempfile::tempdir().unwrap();
    let home = Arc::new(Home::at(dir.path().join(NAME)));
    let connection = bus.connect().await;
    let rings = rings(&connection, &home);
    relocate(&all(&rings), true).await.unwrap();
    assert!(home.in_file());
    rings[0].store("photos", "one").await.unwrap();
    let refused = unavailable_with(relocate(&all(&rings), false).await.map(drop));
    assert_eq!(refused, "no keyring answers to store the secrets");
    assert!(home.in_file());
    same(&rings[0], "photos", "one").await;
}

#[tokio::test]
async fn a_locked_keyring_that_stays_locked_changes_nothing() {
    let bus = Bus::start();
    let vault = Shared::default();
    let _service = serve(&bus, &vault).await;
    let dir = tempfile::tempdir().unwrap();
    let home = Arc::new(Home::at(dir.path().join(NAME)));
    let connection = bus.connect().await;
    let rings = rings(&connection, &home);
    rings[0].store("photos", "one").await.unwrap();
    vault.lock().unwrap().locked = true;
    assert!(relocate(&all(&rings), true).await.is_err());
    assert!(!home.in_file() && vault.lock().unwrap().items.len() == 1);
}
