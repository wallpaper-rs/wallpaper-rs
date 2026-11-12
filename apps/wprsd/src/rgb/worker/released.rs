use super::super::covered::{cover, rest, want};
use super::tests::{SERIAL, follower, released, server};
use super::*;

fn green() -> Strip {
    Strip::new(1, 1, vec![0, 255, 0]).unwrap()
}

fn hidden(source: &str, on: bool) {
    cover(source, on);
    rest(source, on);
}

#[test]
fn a_shown_screen_that_keeps_painting_holds_the_lights_when_the_covered_one_painted_last() {
    let _serial = lock(&SERIAL);
    let (address, _heard) = server(|| {});
    let mut s = follower("TEST-BOTH-HIDDEN", address);
    hidden("TEST-BOTH-HIDDEN", true);
    want("TEST-BOTH-HIDDEN", true);
    s.retry();
    s.frame("TEST-BOTH-SHOWN".into(), green());
    s.frame("TEST-BOTH-HIDDEN".into(), green());
    want("TEST-BOTH-HIDDEN", false);
    s.retry();
    hidden("TEST-BOTH-HIDDEN", false);
    assert!(s.client.is_some() && s.wait() == LISTEN, "the lights were let go while a shown screen paints");
    assert!(lock(&s.status).connected, "the connection flapped while a shown screen paints");
}

#[test]
fn lights_opened_for_a_covered_screen_go_once_the_shown_one_rests_or_stops_sending() {
    let _serial = lock(&SERIAL);
    for forgotten in [false, true] {
        let (address, heard) = server(|| {});
        let mut s = follower("TEST-OPENED-HIDDEN", address);
        hidden("TEST-OPENED-HIDDEN", true);
        want("TEST-OPENED-HIDDEN", true);
        s.retry();
        s.frame("TEST-OPENED-HIDDEN".into(), green());
        s.frame("TEST-OPENED-SHOWN".into(), green());
        want("TEST-OPENED-HIDDEN", false);
        s.retry();
        assert!(s.client.is_some(), "the lights were let go while a shown screen paints");
        match forgotten {
            true => s.forget(Some("TEST-OPENED-SHOWN")),
            false => {
                rest("TEST-OPENED-SHOWN", true);
                s.retry();
            }
        }
        rest("TEST-OPENED-SHOWN", false);
        hidden("TEST-OPENED-HIDDEN", false);
        assert!(
            released(&s, &heard),
            "lights nothing shows any more kept the devices (forgotten: {forgotten})"
        );
    }
}

#[test]
fn lights_a_shown_screen_opened_go_when_it_is_covered_and_then_locked() {
    let _serial = lock(&SERIAL);
    let (address, heard) = server(|| {});
    let mut s = follower("TEST-LOCKED", address);
    s.frame("TEST-LOCKED".into(), green());
    assert!(s.client.is_some(), "a shown screen did not connect");
    hidden("TEST-LOCKED", true);
    want("TEST-LOCKED", true);
    s.retry();
    s.frame("TEST-LOCKED".into(), green());
    want("TEST-LOCKED", false);
    s.retry();
    hidden("TEST-LOCKED", false);
    assert!(released(&s, &heard), "a locked screen kept the devices it opened while shown");
}
