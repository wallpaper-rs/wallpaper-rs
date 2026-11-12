use crate::{Error, said, settings};
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub(crate) enum Request {
    Wallpaper,
    RemoveWallpaper,
    RestoreWallpaper,
}

pub(crate) fn request(request: Request) -> Result<Value, Error> {
    match request {
        Request::Wallpaper => {}
        Request::RemoveWallpaper => settings::update(|config| {
            config.clear_content("");
            Ok(())
        })?,
        Request::RestoreWallpaper => {
            let mut back = false;
            settings::update(|config| {
                back = config.restore_content("");
                Ok(())
            })?;
            if !back {
                return Err(said(
                    "android.nothing_before",
                    "there is no earlier wallpaper to bring back",
                    &[],
                ));
            }
        }
    }
    let config = settings::config();
    Ok(json!({ "blank": config.is_blank(""), "previous": config.has_previous("") }))
}
