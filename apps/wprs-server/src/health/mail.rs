use crate::App;
use crate::error::Res;
use crate::mail::{Setup, Why, last, setup};

pub enum Mail {
    NotSet,
    Ready(String),
    Works { to: String, sent: i64 },
    Failed { when: i64, why: Why, raw: String },
}

pub struct Found {
    pub mail: Mail,
    pub setup: Option<Setup>,
    pub secret: bool,
}

pub async fn status(app: &App) -> Res<Found> {
    let (setup, last) = app.db.run(|c| Ok((setup(c)?, last(c)?))).await?;
    let secret = crate::mail::secret_path(&app.cfg.data).is_file();
    let mail = match (&setup, last.failed, last.sent_at) {
        (None, ..) => Mail::NotSet,
        (Some(_), Some((when, why, raw)), _) => Mail::Failed { when, why, raw },
        (Some(s), None, Some(sent)) => Mail::Works { to: s.to.clone(), sent },
        (Some(s), None, None) => Mail::Ready(s.to.clone()),
    };
    Ok(Found { mail, setup, secret })
}
