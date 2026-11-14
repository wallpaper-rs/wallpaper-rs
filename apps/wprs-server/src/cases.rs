use crate::db::{now, one, rows};
use crate::error::Res;
use rusqlite::Connection;
use serde_json::{Value, json};

macro_rules! after {
    () => {
        "CASE WHEN v.state = 'pending' THEN (SELECT v2.version FROM versions v2 \
        WHERE v2.package_id = v.package_id AND v2.id > v.id ORDER BY v2.id DESC LIMIT 1) END AS newer, \
        CASE WHEN v.state = 'pending' THEN (SELECT c2.id FROM cases c2 JOIN versions v2 ON v2.id = c2.version_id \
        WHERE v2.package_id = v.package_id AND v2.id > v.id AND c2.state = 'open' \
        ORDER BY v2.id DESC LIMIT 1) END AS newer_case, \
        NOT EXISTS (SELECT 1 FROM versions v3 WHERE v3.package_id = v.package_id AND v3.id < v.id) AS first_upload"
    };
}
pub(crate) use after;

const CASE: &str = concat!(
    "SELECT c.id, c.reason, c.detail, c.priority, c.created_at, c.state, c.decision, c.statement, \
    v.package_id AS package, v.version, v.title, v.kind, v.state AS version_state, v.sha256, v.license, \
    v.signer, v.asks, v.photosensitive, v.content, v.created_at AS uploaded_at, v.uploaded_by AS uploader_id, \
    u.handle AS uploader, ",
    after!(),
    " FROM cases c JOIN versions v ON v.id = c.version_id \
    LEFT JOIN users u ON u.id = v.uploaded_by WHERE c.id = ?1"
);

const HIDDEN: &str = "SELECT p.id, p.hidden AS reason, p.hidden_at, u.handle AS owner, \
    (SELECT v.title FROM versions v WHERE v.package_id = p.id ORDER BY v.created_at DESC LIMIT 1) AS title, \
    (SELECT v.sha256 FROM versions v WHERE v.package_id = p.id AND v.state = 'published' \
     ORDER BY v.created_at DESC LIMIT 1) AS sha256 \
    FROM packages p LEFT JOIN users u ON u.id = p.owner_id WHERE p.hidden IS NOT NULL ORDER BY p.hidden_at DESC";

pub fn case(c: &Connection, id: i64) -> Res<Option<Value>> {
    Ok(rows(c, CASE, [id])?.into_iter().next())
}

pub fn hidden(c: &Connection) -> Res<Vec<Value>> {
    rows(c, HIDDEN, [])
}

pub fn history(c: &Connection, case: &Value) -> Res<Value> {
    let (user, current) = (case["uploader_id"].as_i64(), case["package"].as_str().unwrap_or_default());
    let Some(user) = user else { return Ok(json!({"known": false})) };
    let tally = |state: &str| {
        let sql = "SELECT COUNT(*) FROM versions WHERE uploaded_by = ?1 AND state = ?2 \
                   AND NOT (package_id = ?3 AND version = ?4)";
        let version = case["version"].as_str().unwrap_or_default();
        one::<i64>(c, sql, rusqlite::params![user, state, current, version]).map(Option::unwrap_or_default)
    };
    let total =
        "SELECT COUNT(*) FROM versions WHERE uploaded_by = ?1 AND NOT (package_id = ?2 AND version = ?3)";
    let version = case["version"].as_str().unwrap_or_default();
    let earlier = one::<i64>(c, total, rusqlite::params![user, current, version])?.unwrap_or_default();
    let strikes = "SELECT COUNT(*) FROM strikes WHERE user_id = ?1 AND expires_at > ?2";
    let strikes = one::<i64>(c, strikes, rusqlite::params![user, now()])?.unwrap_or_default();
    let since = one::<i64>(c, "SELECT created_at FROM users WHERE id = ?1", [user])?.unwrap_or_default();
    Ok(json!({
        "known": true, "earlier": earlier, "published": tally("published")?, "rejected": tally("rejected")?,
        "strikes": strikes, "since": since,
    }))
}

pub fn newer(c: &Connection, version_id: i64) -> Res<Option<String>> {
    let sql = "SELECT v2.version FROM versions v2 JOIN versions v ON v.id = ?1 \
               WHERE v2.package_id = v.package_id AND v2.id > v.id ORDER BY v2.id DESC LIMIT 1";
    one::<String>(c, sql, [version_id])
}
