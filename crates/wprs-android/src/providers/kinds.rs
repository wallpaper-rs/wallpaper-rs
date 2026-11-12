use crate::{Error, said};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use wprs_providers::wallhaven::SORTINGS;

pub(crate) type Options = BTreeMap<String, String>;

const REFRESH: [&str; 5] = ["6h", "12h", "1d", "3d", "7d"];

#[derive(Clone, Copy)]
pub(super) enum Shape {
    Text,
    Url,
    Int(u32, u32),
    Pick(&'static [&'static str]),
}

pub(super) struct Field(pub(super) &'static str, pub(super) Shape, pub(super) &'static str, pub(super) bool);

pub(super) struct Kind {
    pub(super) name: &'static str,
    pub(super) hosts: &'static [&'static str],
    pub(super) secret: Option<(&'static str, bool)>,
    pub(super) refresh: &'static str,
    pub(super) fields: &'static [Field],
}

pub(super) const KEPT: Field = Field("keep", Shape::Int(5, 200), "30", false);
pub(super) const WHEN: Field = Field("refresh", Shape::Pick(&REFRESH), "1d", false);

const KINDS: [Kind; 7] = [
    Kind {
        name: "bing",
        hosts: &["www.bing.com"],
        secret: None,
        refresh: "12h",
        fields: &[Field("market", Shape::Text, "en-US", false), Field("count", Shape::Int(1, 8), "8", false)],
    },
    Kind {
        name: "apod",
        hosts: &["api.nasa.gov", "apod.nasa.gov"],
        secret: Some(("api_key", false)),
        refresh: "12h",
        fields: &[Field("days", Shape::Int(1, 30), "1", false)],
    },
    Kind {
        name: "wikimedia",
        hosts: &["commons.wikimedia.org", "upload.wikimedia.org"],
        secret: None,
        refresh: "12h",
        fields: &[Field("days", Shape::Int(1, 30), "1", false)],
    },
    Kind {
        name: "wallhaven",
        hosts: &["wallhaven.cc", "w.wallhaven.cc"],
        secret: Some(("api_key", false)),
        refresh: "1d",
        fields: &[
            Field("query", Shape::Text, "", false),
            Field("sorting", Shape::Pick(&SORTINGS), "toplist", false),
            Field("max_items", Shape::Int(1, 24), "12", false),
        ],
    },
    Kind {
        name: "webdav",
        hosts: &[],
        secret: Some(("password", true)),
        refresh: "1d",
        fields: &[Field("url", Shape::Url, "", true), Field("user", Shape::Text, "", true)],
    },
    Kind {
        name: "nextcloud",
        hosts: &[],
        secret: Some(("password", true)),
        refresh: "1d",
        fields: &[
            Field("server", Shape::Url, "", true),
            Field("user", Shape::Text, "", true),
            Field("folder", Shape::Text, "", false),
        ],
    },
    Kind {
        name: "immich",
        hosts: &[],
        secret: Some(("api_key", true)),
        refresh: "1d",
        fields: &[Field("server", Shape::Url, "", true), Field("album", Shape::Text, "", true)],
    },
];

pub(super) fn kind(name: &str) -> Result<&'static Kind, Error> {
    let found = KINDS.iter().find(|k| k.name == name);
    found.ok_or_else(|| said("android.provider_kind", "%1 is not a kind of picture source", &[&name]))
}

fn describe(field: &Field, default: &str) -> Value {
    let Field(key, shape, _, required) = field;
    let (kind, values, min, max) = match shape {
        Shape::Text => ("text", json!([]), 0, 0),
        Shape::Url => ("url", json!([]), 0, 0),
        Shape::Int(min, max) => ("int", json!([]), *min, *max),
        Shape::Pick(values) => ("pick", json!(values), 0, 0),
    };
    json!({"key": key, "type": kind, "default": default, "required": required, "values": values, "min": min, "max": max})
}

pub(super) fn schema() -> Value {
    let kinds = KINDS.iter().map(|k| {
        let own = k.fields.iter().map(|f| describe(f, f.2));
        let common = [describe(&WHEN, k.refresh), describe(&KEPT, KEPT.2)];
        let secret = k.secret.map(|(name, required)| json!({"name": name, "required": required}));
        json!({"kind": k.name, "hosts": k.hosts, "options": own.chain(common).collect::<Vec<_>>(), "secret": secret})
    });
    json!(kinds.collect::<Vec<_>>())
}

pub(super) fn hosts(kind: &Kind, options: &Options) -> Vec<String> {
    let own = ["url", "server"].iter().find_map(|key| reqwest::Url::parse(options.get(*key)?).ok());
    let own = own.and_then(|url| url.host_str().map(String::from));
    if kind.hosts.is_empty() {
        own.into_iter().collect()
    } else {
        kind.hosts.iter().map(|h| (*h).to_owned()).collect()
    }
}

pub(super) fn secret_of(kind: &Kind) -> Option<(&'static str, bool)> {
    kind.secret
}
