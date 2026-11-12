use super::{known, with};
use crate::{Error, store};
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub(super) enum Request {
    Rate { uids: Vec<String>, rating: u8 },
    Retag { uids: Vec<String>, add: Vec<String>, remove: Vec<String> },
    Rename { uid: String, title: String },
    Tags,
}

pub(super) fn request(request: Request) -> Result<Value, Error> {
    let reply = with(|lib| match request {
        Request::Rate { uids, rating } => {
            Ok(lib.set_rating_many(&known(lib, &uids), rating).map(|()| Value::Null)?)
        }
        Request::Retag { uids, add, remove } => {
            let (add, remove): (Vec<&str>, Vec<&str>) =
                (add.iter().map(String::as_str).collect(), remove.iter().map(String::as_str).collect());
            Ok(lib.retag(&known(lib, &uids), &add, &remove).map(|()| Value::Null)?)
        }
        Request::Rename { uid, title } => Ok(lib.set_title(&uid, title.trim()).map(|()| Value::Null)?),
        Request::Tags => Ok(json!(
            lib.tags()?
                .into_iter()
                .map(|(name, count)| json!({"name": name, "count": count}))
                .collect::<Vec<_>>()
        )),
    })?;
    store::changed();
    Ok(reply)
}
