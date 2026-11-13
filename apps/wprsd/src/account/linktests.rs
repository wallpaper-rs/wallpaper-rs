use super::link::{EXPIRED, NEEDS_PLAIN, REFUSED, problem_of};
use super::saved::Saved;
use super::sync_error;
use crate::error::{Error, failed};
use wprs_core::dbus::error::{KEYRING_UNAVAILABLE, SIGNED_OUT};
use wprs_core::model::AccountState;
use wprs_sync::Error as Sync;

fn code(error: &Error) -> Option<&'static str> {
    problem_of(error).map(|(code, _)| code)
}

#[test]
fn a_failed_link_is_told_apart_by_its_cause() {
    let keyring = Error::Named(KEYRING_UNAVAILABLE, "no keyring answers".into());
    let causes = [keyring, failed(REFUSED), failed(EXPIRED), failed(NEEDS_PLAIN), failed("disk full")];
    let found: Vec<_> = causes.iter().map(code).collect();
    assert_eq!(found, [Some("keyring"), Some("refused"), Some("expired"), Some("encryption"), Some("other")]);
    assert_eq!(code(&Error::Cancelled), None);
}

#[test]
fn a_token_the_server_refuses_means_signed_out() {
    let refused = Sync::Server { status: 401, code: "unauthorized".into(), detail: "sign in".into() };
    assert_eq!(sync_error(refused).parts().0, SIGNED_OUT);
    let other = Sync::Server { status: 403, code: "forbidden".into(), detail: "no".into() };
    assert_ne!(sync_error(other).parts().0, SIGNED_OUT);
}

#[test]
fn a_signed_out_computer_shows_it_and_stops_syncing() {
    let mut saved = Saved {
        url: "https://wallpapers.example.org".into(),
        device: Some("d".into()),
        ready: true,
        ..Saved::default()
    };
    assert!(saved.active());
    assert_eq!(saved.model(false, None).map(|a| a.state), Some(AccountState::Ready));
    saved.signed_out = true;
    assert!(!saved.active());
    assert_eq!(saved.model(false, None).map(|a| a.state), Some(AccountState::SignedOut));
}

#[test]
fn a_failed_link_reaches_the_status() {
    let saved = Saved {
        url: "https://wallpapers.example.org".into(),
        link_problem: Some("keyring".into()),
        link_error: Some("no keyring answers".into()),
        ..Saved::default()
    };
    let account = saved.model(false, None).unwrap();
    assert_eq!(account.state, AccountState::Unlinked);
    assert_eq!(account.link_problem.as_deref(), Some("keyring"));
}
