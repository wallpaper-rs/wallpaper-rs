use crate::App;
use crate::creds::{digest, hex, random};
use crate::db::{DAY, audit, now};
use crate::error::Res;
use rusqlite::params;

pub async fn new_invite(app: &App, by: i64, role: &str, uses: i64, days: i64) -> Res<(String, i64, i64)> {
    let (code, expires) = (hex(&random::<16>()), now() + days * DAY);
    let (hash, kept) = (digest(&code), role.to_owned());
    let sql = "INSERT INTO invites (code_hash, role, uses_left, expires_at, created_by) VALUES (?1, ?2, ?3, ?4, ?5)";
    let id = app.db.run(move |c| {
        c.execute(sql, params![hash, kept, uses, expires, by])?;
        audit(c, Some(by), "invite", &kept, "")?;
        Ok(c.last_insert_rowid())
    });
    Ok((code, id.await?, expires))
}
