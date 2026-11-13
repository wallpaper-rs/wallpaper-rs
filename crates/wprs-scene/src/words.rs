use crate::props::Text;
use crate::props::choose;
use crate::resolve::Context;
use crate::value::Value;
use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::BTreeMap;
use wprs_core::locale::names;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Words {
    Said(Value<String>),
    Told(BTreeMap<String, String>),
}

impl From<Value<String>> for Words {
    fn from(value: Value<String>) -> Self {
        Words::Said(value)
    }
}

impl JsonSchema for Words {
    fn schema_name() -> Cow<'static, str> {
        "Words".into()
    }

    fn json_schema(generator: &mut SchemaGenerator) -> Schema {
        let (said, told) = (generator.subschema_for::<Value<String>>(), generator.subschema_for::<Text>());
        json_schema!({"anyOf": [said, told]})
    }
}

impl Words {
    pub fn said(&self) -> Option<&Value<String>> {
        match self {
            Words::Said(value) => Some(value),
            Words::Told(_) => None,
        }
    }

    pub fn languages(&self) -> impl Iterator<Item = &str> {
        match self {
            Words::Told(map) => Some(map.keys().map(String::as_str)),
            Words::Said(_) => None,
        }
        .into_iter()
        .flatten()
    }

    pub(crate) fn get<'a>(&'a self, cx: &Context<'a>) -> Cow<'a, str> {
        match self {
            Words::Told(map) => Cow::Borrowed(choose(map, names(cx.languages))),
            Words::Said(Value::Fixed(text)) => Cow::Borrowed(text.as_str()),
            Words::Said(bound) => match bound.binding().and_then(|b| cx.said(b)) {
                Some(said) => Cow::Borrowed(said),
                None => Cow::Owned(bound.get(cx)),
            },
        }
    }
}

pub const MAX_LANGUAGES: usize = 64;
pub const MAX_WORDS: usize = 8000;

impl crate::validate::Check<'_> {
    pub(crate) fn words(&mut self, path: &str, words: &Words) {
        match words {
            Words::Said(value) => self.value(path, value),
            Words::Told(map) => {
                self.uses(true, "text_languages");
                self.count(path, map.len(), MAX_LANGUAGES);
                for (language, text) in map {
                    let at = format!("{path}/{language}");
                    if !wprs_core::locale::valid(language) {
                        self.issues.push(crate::issue::Issue::Type(at.clone()));
                    }
                    self.filled(&at, text);
                    self.count(&at, text.chars().count(), MAX_WORDS);
                }
            }
        }
    }
}
