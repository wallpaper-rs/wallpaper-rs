use crate::App;

#[allow(dead_code)]
pub enum Mail {
    NotSet,
    Works(String),
    Failed { when: i64, why: String, raw: String },
}

pub fn status(_app: &App) -> Mail {
    Mail::NotSet
}
