use super::{ask, open};
use crate::choice::{self, Target};
use crate::{settings, store};
use serde_json::{Value, json};

fn on(lock: bool) -> String {
    choice::shown_on(false, lock).map(|shown| shown.uid).unwrap_or_default()
}

fn set(choice: Value, target: Target) {
    assert_eq!(ask(json!({ "op": "pick", "choice": choice })), Value::Null);
    choice::commit(true, target).unwrap();
}

pub(super) fn home_and_lock() {
    let green = store::store(&mut open("green-64x32.png"), "green.png").unwrap().0;
    let bands = store::store(&mut open("bands-640x180.png"), "bands.png").unwrap().0;
    set(json!({ "item": green }), Target::Both);
    assert_eq!((on(false), on(true)), (green.clone(), green.clone()));
    set(json!({ "item": bands }), Target::Lock);
    assert_eq!((on(false), on(true)), (green.clone(), bands.clone()));
    set(json!({ "item": bands }), Target::Home);
    set(json!({ "item": green }), Target::Home);
    assert_eq!((on(false), on(true)), (green.clone(), bands.clone()), "the lock keeps its own choice");
    set(json!({ "item": bands }), Target::Both);
    assert_eq!((on(false), on(true)), (bands.clone(), bands.clone()));
    set(json!({ "item": green }), Target::Home);
    assert_eq!((on(false), on(true)), (green.clone(), bands.clone()), "home alone leaves the lock as it was");
    let list = ask(json!({ "op": "create", "name": "Both" })).as_str().unwrap().to_owned();
    ask(json!({ "op": "add", "playlist": list, "items": [green, bands] }));
    set(json!({ "playlist": list }), Target::Lock);
    let listed = ask(json!({ "op": "playlists" }));
    let row = listed.as_array().unwrap().iter().find(|p| p["playlist"]["uid"] == json!(list)).unwrap();
    assert_eq!((&row["home"], &row["lock"], &row["current"]), (&json!(false), &json!(true), &json!(true)));
    assert_eq!((on(false), on(true)), (green.clone(), green.clone()));
    assert!(!choice::advance(true, true).unwrap() && choice::colors(false, true).unwrap().is_some());
    assert_eq!(ask(json!({ "op": "delete", "playlist": list })), Value::Null);
    assert_eq!(on(true), green, "a deleted list leaves the lock showing the home screen");
    set(json!({ "item": bands }), Target::Lock);
    assert_eq!(ask(json!({ "op": "remove", "uids": [bands] })), Value::Null);
    assert_eq!((on(false), on(true)), (green.clone(), green.clone()));
    assert!(settings::config().outputs.get(choice::LOCK).is_none_or(|lock| lock.image.is_none()));
    removing_the_wallpaper_keeps_the_previous_one();
}

fn removing_the_wallpaper_keeps_the_previous_one() {
    let state = ask(json!({ "op": "wallpaper" }));
    assert_eq!((&state["blank"], &state["previous"]), (&json!(false), &json!(false)));
    let green = on(false);
    let removed = ask(json!({ "op": "remove_wallpaper" }));
    assert_eq!((&removed["blank"], &removed["previous"]), (&json!(true), &json!(true)), "{removed}");
    assert_eq!((on(false), on(true)), (String::new(), String::new()));
    let restored = ask(json!({ "op": "restore_wallpaper" }));
    assert_eq!((&restored["blank"], &restored["previous"]), (&json!(false), &json!(false)), "{restored}");
    assert_eq!((on(false), on(true)), (green.clone(), green));
    assert!(ask(json!({ "op": "restore_wallpaper" }))["error"].is_string());
}

pub(super) fn ruled_lists() {
    let morning = ask(json!({ "op": "create", "name": "Morning" })).as_str().unwrap().to_owned();
    let info = json!({ "uid": morning, "name": "Mañana", "query": "kind:image",
        "rules": { "interval": "1h", "at": ["07:30", "18:00"], "days": ["Mon", "Sat"] } });
    assert_eq!(ask(json!({ "op": "save", "playlist": info })), Value::Null);
    let listed = ask(json!({ "op": "playlists" }));
    let row = listed.as_array().unwrap().iter().find(|p| p["playlist"]["uid"] == json!(morning)).unwrap();
    let rules = &row["playlist"]["rules"];
    assert_eq!(
        (&row["playlist"]["name"], &rules["at"], &rules["days"]),
        (&json!("Mañana"), &json!(["07:30", "18:00"]), &json!(["Mon", "Sat"]))
    );
    let images = ask(json!({ "op": "count", "query": "kind:image" }));
    assert_eq!(row["count"], images, "a smart list holds what its search finds");
    assert!(ask(json!({ "op": "count", "query": "\"open" }))["error"].is_string());
    let bad = json!({ "uid": morning, "name": "Mañana", "query": null, "rules": { "at": ["25:00"] } });
    assert!(ask(json!({ "op": "save", "playlist": bad }))["error"].is_string());
    assert_eq!(ask(json!({ "op": "delete", "playlist": morning })), Value::Null);
}

pub(super) fn synced_lists_fade_their_own_way() {
    let green = store::store(&mut open("green-64x32.png"), "green.png").unwrap().0;
    let bands = store::store(&mut open("bands-640x180.png"), "bands.png").unwrap().0;
    let list = ask(json!({ "op": "create", "name": "Twins" })).as_str().unwrap().to_owned();
    ask(json!({ "op": "add", "playlist": list, "items": [green, bands] }));
    let rules = json!({ "interval": "1h", "synced": true, "resume": false, "transition_ms": 250 });
    let info = json!({ "uid": list, "name": "Twins", "query": null, "rules": rules });
    assert_eq!(ask(json!({ "op": "save", "playlist": info })), Value::Null);
    set(json!({ "playlist": list }), Target::Both);
    assert_eq!(on(false), on(true));
    assert_eq!(settings::look(true).fade, std::time::Duration::from_millis(250));
    crate::library::with(|lib| {
        let mut row = lib.rotation("home")?.unwrap();
        row.next_at = Some(0);
        Ok(lib.save_rotation(&row)?)
    })
    .unwrap();
    let before = on(false);
    assert!(choice::advance(true, true).unwrap());
    assert!(on(true) != before && on(false) == on(true), "a synced list turns both screens together");
    assert_eq!(ask(json!({ "op": "delete", "playlist": list })), Value::Null);
    assert_eq!(settings::look(true).fade, settings::config().transition.duration());
    assert_eq!(ask(json!({ "op": "remove", "uids": [green, bands] })), Value::Null);
    rotating_in_one_step();
    sound_of_the_wallpaper();
}

fn rotating_in_one_step() {
    let green = store::store(&mut open("green-64x32.png"), "green.png").unwrap().0;
    let bands = store::store(&mut open("bands-640x180.png"), "bands.png").unwrap().0;
    let rotate = |every: &str| {
        ask(json!({ "op": "rotate", "name": "Rotation", "items": [green, bands], "every": every }))
    };
    let first = rotate("10m").as_str().unwrap().to_owned();
    let second = rotate("1h").as_str().unwrap().to_owned();
    assert!(rotate("0s")["error"].is_string(), "a rotation needs a time longer than zero");
    assert!(rotate("soon")["error"].is_string());
    let listed = ask(json!({ "op": "playlists" }));
    let row = |uid: &str| {
        listed.as_array().unwrap().iter().find(|p| p["playlist"]["uid"] == json!(uid)).unwrap().clone()
    };
    let (a, b) = (row(&first), row(&second));
    assert_eq!((&a["count"], &a["playlist"]["rules"]["interval"]), (&json!(2), &json!("10m")));
    assert_eq!((&a["playlist"]["name"], &b["playlist"]["name"]), (&json!("Rotation"), &json!("Rotation 2")));
    choice::commit(true, Target::Home).unwrap();
    assert_eq!(on(false), green, "the new rotation is the one that starts");
    for uid in [first, second] {
        assert_eq!(ask(json!({ "op": "delete", "playlist": uid })), Value::Null);
    }
    assert_eq!(ask(json!({ "op": "remove", "uids": [green, bands] })), Value::Null);
}

fn sound_of_the_wallpaper() {
    let own = |key: &str, value: Option<&str>| settings::setting(key, value).unwrap();
    assert_eq!(
        (own("sound.heard", None).as_str(), own("playback.muted.default", None).as_str()),
        ("1", "none")
    );
    own("playback.volume.default", Some("0.4"));
    assert_eq!(own("sound.heard", None), "0.4");
    own("playback.muted.default", Some("true"));
    assert_eq!(
        (own("sound.heard", None).as_str(), own("playback.volume.default", None).as_str()),
        ("0", "0.4")
    );
    assert!(settings::setting("playback.volume.default", Some("loud")).is_err());
    own("playback.muted.default", Some("none"));
    own("playback.volume.default", Some("none"));
    assert_eq!(own("sound.heard", None), "1", "a sound set back to follow left a trace");
}
