use super::kinds::{Field, KEPT, Kind, Options, Shape, WHEN};
use crate::{Error, said};

fn address(key: &str, text: &str) -> Result<(), Error> {
    let url = reqwest::Url::parse(text).ok().filter(|u| u.host_str().is_some());
    let url = url.ok_or_else(|| said("android.provider_address", "%1 is not a web address", &[&key]))?;
    let clean = url.username().is_empty()
        && url.password().is_none()
        && url.query().is_none()
        && url.fragment().is_none();
    if !matches!(url.scheme(), "https" | "http") || !clean {
        return Err(said(
            "android.provider_https",
            "the address of %1 starts with https:// or http:// and holds no password or search part",
            &[&key],
        ));
    }
    Ok(())
}

fn checked(field: &Field, value: &str) -> Result<String, Error> {
    let value = value.trim();
    let bad = || said("android.provider_option", "the value of %1 is not valid", &[&field.0]);
    let plain = value.len() <= 256 && !value.chars().any(char::is_control);
    match field.1 {
        Shape::Url => address(field.0, value).map(|()| value.to_owned()),
        Shape::Int(min, max) => value
            .parse::<u32>()
            .ok()
            .filter(|n| (min..=max).contains(n))
            .map(|n| n.to_string())
            .ok_or_else(bad),
        Shape::Pick(values) => values.contains(&value).then(|| value.to_owned()).ok_or_else(bad),
        Shape::Text if field.0 == "folder" => {
            (plain && !value.split('/').any(|s| s == "..")).then(|| value.to_owned()).ok_or_else(bad)
        }
        Shape::Text if field.0 == "market" => {
            let word = !value.is_empty() && value.chars().all(|c| c.is_ascii_alphanumeric() || c == '-');
            word.then(|| value.to_owned()).ok_or_else(bad)
        }
        Shape::Text => (plain && (!field.3 || !value.is_empty())).then(|| value.to_owned()).ok_or_else(bad),
    }
}

pub(super) fn effective(kind: &Kind, given: &Options) -> Result<Options, Error> {
    let known = |key: &str| key == WHEN.0 || key == KEPT.0 || kind.fields.iter().any(|f| f.0 == key);
    if let Some(key) = given.keys().find(|key| !known(key)) {
        return Err(said("android.provider_option_unknown", "%1 has no option %2", &[&kind.name, key]));
    }
    let mut options = Options::new();
    let all = kind.fields.iter().chain([&WHEN, &KEPT]);
    for field in all {
        let fallback = if field.0 == WHEN.0 { kind.refresh } else { field.2 };
        let value = match given.get(field.0) {
            Some(value) => checked(field, value)?,
            None if field.3 => {
                return Err(said(
                    "android.provider_needs",
                    "%1 needs the option %2",
                    &[&kind.name, &field.0],
                ));
            }
            None => fallback.to_owned(),
        };
        options.insert(field.0.to_owned(), value);
    }
    Ok(options)
}
