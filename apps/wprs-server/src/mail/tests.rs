#[path = "../../tests/common/smtp.rs"]
mod smtp;

use super::queue::WAITS;
use super::{KEYS, Why, deliver, enqueue, last, rewatch, watch};
use crate::db::{now, one};
use crate::settings::put;
use crate::{App, Config};
use smtp::{Answer, closed_port, listen};
use std::collections::HashMap;
use std::sync::Arc;

fn open() -> (tempfile::TempDir, Arc<App>) {
    opened(false)
}

fn opened(repo: bool) -> (tempfile::TempDir, Arc<App>) {
    let dir = tempfile::tempdir().unwrap();
    let vars: HashMap<String, String> = [
        ("WPRS_DATA", dir.path().join("data").display().to_string()),
        ("WPRS_PUBLIC_URL", "http://127.0.0.1:9".into()),
        ("WPRS_NAME", "Example".into()),
    ]
    .into_iter()
    .map(|(k, v)| (k.to_string(), v))
    .collect();
    let cfg = Config::from_vars(|key| vars.get(key).cloned()).unwrap();
    if repo {
        crate::init(&cfg, "Example wallpapers", false, false).unwrap();
    }
    (dir, App::open(cfg).unwrap())
}

async fn set(app: &App, port: u16, user: &str) {
    let (port, user) = (port.to_string(), user.to_string());
    app.db
        .run(move |c| {
            let values =
                ["127.0.0.1", &port, "none", &user, "Example <server@example.org>", "ana@example.org", "en"];
            put(c, &KEYS.iter().copied().zip(values).collect::<Vec<_>>())
        })
        .await
        .unwrap();
    super::keep_secret(&app.cfg.data, "a secret").unwrap();
}

async fn queued(app: &App) -> Vec<(String, i64, i64)> {
    let sql = "SELECT state, tries, next_at FROM mail_queue ORDER BY id";
    app.db
        .run(move |c| {
            let mut s = c.prepare(sql)?;
            let found = s.query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?;
            Ok(found.collect::<Result<Vec<_>, _>>()?)
        })
        .await
        .unwrap()
}

fn note() -> (String, String) {
    ("Example: test mail".into(), "This is the test mail.".into())
}

#[tokio::test(flavor = "multi_thread")]
async fn while_it_is_off_nothing_is_queued_or_sent() {
    let (_dir, app) = open();
    assert!(!enqueue(&app, "test", None, |_| note()).await.unwrap());
    assert!(queued(&app).await.is_empty());
    assert_eq!(deliver(&app, now()).await.unwrap(), None);
    rewatch(&app);
    assert_eq!(watch(&app).await.unwrap(), 0);
    assert!(queued(&app).await.is_empty());
}

#[tokio::test(flavor = "multi_thread")]
async fn a_test_mail_arrives_and_the_last_result_is_remembered() {
    let (_dir, app) = open();
    let inbox = listen().await;
    set(&app, inbox.port, "ana").await;
    assert!(enqueue(&app, "test", None, |_| note()).await.unwrap());
    assert_eq!(deliver(&app, now()).await.unwrap(), None);
    let mails = inbox.mails();
    assert_eq!(mails.len(), 1);
    assert!(mails[0].contains("Subject: Example: test mail") && mails[0].contains("To: ana@example.org"));
    assert_eq!(queued(&app).await[0].0, "sent");
    let seen = app.db.run(|c| last(c)).await.unwrap();
    assert!(seen.sent_at.is_some() && seen.failed.is_none());
}

#[tokio::test(flavor = "multi_thread")]
async fn a_failing_mail_retries_with_growing_waits_and_then_gives_up() {
    let (_dir, app) = open();
    set(&app, closed_port(), "").await;
    enqueue(&app, "test", None, |_| note()).await.unwrap();
    let mut at = now();
    for (i, wait) in WAITS.iter().enumerate() {
        deliver(&app, at).await.unwrap();
        let (state, tries, next) = queued(&app).await[0].clone();
        assert_eq!((state.as_str(), tries, next), ("queued", i as i64 + 1, at + wait));
        deliver(&app, at + wait - 1).await.unwrap();
        assert_eq!(queued(&app).await[0].1, i as i64 + 1);
        at += wait;
    }
    assert_eq!(deliver(&app, at).await.unwrap(), None);
    let (state, tries, _) = queued(&app).await[0].clone();
    assert_eq!((state.as_str(), tries), ("failed", WAITS.len() as i64 + 1));
    let (when, why, raw) = app.db.run(|c| last(c)).await.unwrap().failed.unwrap();
    assert_eq!((when, why), (at, Why::Reach));
    assert!(raw.starts_with("smtp 127.0.0.1:") && !raw.contains("a secret"));
}

#[tokio::test(flavor = "multi_thread")]
async fn a_refused_password_is_said_in_words_and_clears_once_a_mail_goes_out() {
    let (_dir, app) = open();
    let inbox = listen().await;
    inbox.answer(Answer::RefusePassword);
    set(&app, inbox.port, "ana").await;
    enqueue(&app, "test", None, |_| note()).await.unwrap();
    let at = now();
    deliver(&app, at).await.unwrap();
    let (_, why, raw) = app.db.run(|c| last(c)).await.unwrap().failed.unwrap();
    assert_eq!(why, Why::Password);
    assert!(raw.contains("535") && !raw.contains("a secret"));
    inbox.answer(Answer::Accept);
    deliver(&app, at + WAITS[0]).await.unwrap();
    assert_eq!(inbox.mails().len(), 1);
    assert!(app.db.run(|c| last(c)).await.unwrap().failed.is_none());
}

async fn health_mails(app: &App) -> i64 {
    let sql = "SELECT COUNT(*) FROM mail_queue WHERE topic = 'health'";
    app.db.run(move |c| one::<i64>(c, sql, [])).await.unwrap().unwrap_or_default()
}

#[tokio::test(flavor = "multi_thread")]
async fn a_health_row_mails_once_per_change_of_state_not_per_check() {
    let (_dir, app) = open();
    let inbox = listen().await;
    set(&app, inbox.port, "").await;
    rewatch(&app);
    assert_eq!(watch(&app).await.unwrap(), 1);
    assert_eq!(watch(&app).await.unwrap(), 0);
    rewatch(&app);
    assert_eq!(watch(&app).await.unwrap(), 0);
    std::fs::write(app.cfg.data.join("backups/today.tar"), b"copy").unwrap();
    rewatch(&app);
    assert_eq!(watch(&app).await.unwrap(), 0);
    std::fs::remove_file(app.cfg.data.join("backups/today.tar")).unwrap();
    rewatch(&app);
    assert_eq!(watch(&app).await.unwrap(), 1);
    assert_eq!(health_mails(&app).await, 2);
    deliver(&app, now()).await.unwrap();
    let mails = inbox.mails();
    assert!(mails.iter().all(|m| m.contains("Subject: Example: =E2") || m.contains("Last backup")));
    assert!(mails[0].contains("the row “Last backup” has little time left") && mails[0].contains("/admin"));
}

#[tokio::test(flavor = "multi_thread")]
async fn a_failed_automatic_signing_turns_the_row_red_and_mails_the_administrator() {
    let (_dir, app) = opened(true);
    crate::jobs::republish(&app, true).await.unwrap();
    let inbox = listen().await;
    set(&app, inbox.port, "").await;
    std::fs::write(app.cfg.data.join("backups/today.tar"), b"copy").unwrap();
    rewatch(&app);
    watch(&app).await.unwrap();
    assert_eq!(health_mails(&app).await, 0);
    std::fs::write(app.cfg.data.join("keys/online.key"), "not a key").unwrap();
    assert!(crate::jobs::republish(&app, true).await.is_err());
    let facts = crate::health::collect(&app).await.unwrap();
    assert!(facts.sign_failed.as_ref().is_some_and(|(_, raw)| !raw.is_empty()));
    let red = crate::health::watched(&facts).into_iter().find(|w| w.0 == "signature").unwrap();
    assert_eq!(red.2, Some(crate::ui::Tone::Bad));
    assert_eq!(watch(&app).await.unwrap(), 1);
    deliver(&app, now()).await.unwrap();
    assert!(inbox.mails().iter().any(|m| m.contains("the row “Index signature” turned red")));
}
