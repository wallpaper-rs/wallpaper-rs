use super::Text;
use std::collections::BTreeMap;
use wprs_core::locale::{names, variants};

impl Text {
    pub fn pick(&self, languages: &[String]) -> String {
        match self {
            Text::Plain(text) => text.clone(),
            Text::Localized(map) => choose(map, names(languages)).to_owned(),
        }
    }

    pub fn chosen(&self, lang: &str) -> &str {
        match self {
            Text::Plain(text) => text,
            Text::Localized(map) => choose(map, variants(lang).into_iter()),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Text::Plain(text) => text.is_empty(),
            Text::Localized(map) => map.is_empty(),
        }
    }
}

impl Default for Text {
    fn default() -> Self {
        Text::Plain(String::new())
    }
}

impl From<&str> for Text {
    fn from(text: &str) -> Self {
        Text::Plain(text.into())
    }
}

pub fn tag(name: &str) -> String {
    let tag = name.replace('_', "-").to_ascii_lowercase();
    match tag.as_str() {
        "zh-chs" | "zh-cn" | "zh-sg" => "zh-hans".into(),
        "zh-cht" | "zh-tw" | "zh-hk" | "zh-mo" => "zh-hant".into(),
        _ => tag,
    }
}

pub fn alike(a: &str, b: &str) -> bool {
    let plain = |s: &str| !s.get(..2).is_some_and(|p| p.eq_ignore_ascii_case("zh"));
    if plain(a) && plain(b) {
        a.len() == b.len()
            && a.bytes().zip(b.bytes()).all(|(x, y)| {
                let fold = |c: u8| if c == b'_' { b'-' } else { c.to_ascii_lowercase() };
                fold(x) == fold(y)
            })
    } else {
        tag(a) == tag(b)
    }
}

pub fn choose<'a, 'b>(map: &'a BTreeMap<String, String>, names: impl Iterator<Item = &'b str>) -> &'a str {
    let find = |name: &str| {
        let exact = map.iter().find(|(k, _)| alike(k, name));
        let base = || {
            map.iter().find(|(k, _)| {
                !name.is_empty() && alike(k.split(['-', '_']).next().unwrap_or_default(), name)
            })
        };
        exact.or_else(base).map(|(_, v)| v.as_str())
    };
    let found = names.chain(["en"]).find_map(find);
    found.or_else(|| map.values().next().map(String::as_str)).unwrap_or_default()
}
