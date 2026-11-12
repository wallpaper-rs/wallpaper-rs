use super::kinds::{Kind, Options};
use crate::{Error, said};
use wprs_providers::wallhaven::Search;
use wprs_providers::{Apod, Bing, Immich, Provider, Secret, Wallhaven, WebDav, WikimediaPotd, client};

pub(super) fn build(kind: &Kind, o: &Options, secret: Option<&str>) -> Result<Box<dyn Provider>, Error> {
    let text = |key: &str| o.get(key).cloned().unwrap_or_default();
    let number = |key: &str| o.get(key).and_then(|v| v.parse::<u32>().ok()).unwrap_or_default();
    let (shut, key) = (
        |e| said("android.web_client", "web connections cannot start: %1", &[&e]),
        secret.unwrap_or_default(),
    );
    let web = || client().map_err(shut);
    Ok(match kind.name {
        "bing" => Box::new(Bing { count: number("count") as u8, ..Bing::new(web()?, text("market"), true) }),
        "apod" => Box::new(Apod {
            days: number("days"),
            ..Apod::new(web()?, secret.filter(|k| !k.is_empty()).unwrap_or("DEMO_KEY"))
        }),
        "wikimedia" => {
            Box::new(WikimediaPotd { days: number("days"), width: 2160, ..WikimediaPotd::new(web()?) })
        }
        "wallhaven" => {
            let search = Search::from_options(|key| text(key));
            Box::new(Wallhaven {
                search,
                api_key: secret.filter(|k| !k.is_empty()).map(Secret::from),
                ..Wallhaven::new().map_err(shut)?
            })
        }
        "webdav" => Box::new(WebDav::new(text("url"), text("user"), key).map_err(shut)?),
        "nextcloud" => {
            Box::new(WebDav::nextcloud(&text("server"), &text("user"), key, &text("folder")).map_err(shut)?)
        }
        _ => Box::new(Immich::new(&text("server"), key, text("album")).map_err(shut)?),
    })
}
