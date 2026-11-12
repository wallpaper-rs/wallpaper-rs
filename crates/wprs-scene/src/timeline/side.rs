use schemars::{JsonSchema, Schema, SchemaGenerator, json_schema};
use serde::de::{self, Deserializer};
use serde::ser::Serializer;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub const MOST_PARTS: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Side {
    All([f32; 2]),
    Parts(u8, [[f32; 2]; MOST_PARTS]),
}

impl Side {
    pub fn parts(list: &[[f32; 2]]) -> Option<Side> {
        let n = list.len();
        ((1..=MOST_PARTS).contains(&n)).then(|| {
            let mut each = [[0.0; 2]; MOST_PARTS];
            each[..n].copy_from_slice(list);
            Side::Parts(n as u8, each)
        })
    }

    pub fn each(&self) -> &[[f32; 2]] {
        match self {
            Side::All(pair) => std::slice::from_ref(pair),
            Side::Parts(n, each) => &each[..usize::from(*n)],
        }
    }

    pub fn split(&self) -> bool {
        matches!(self, Side::Parts(..))
    }
}

impl Serialize for Side {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        match self {
            Side::All(pair) => pair.serialize(s),
            Side::Parts(..) => self.each().serialize(s),
        }
    }
}

impl<'de> Deserialize<'de> for Side {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Repr {
            All([f32; 2]),
            Parts(Vec<[f32; 2]>),
        }
        match Repr::deserialize(d) {
            Ok(Repr::All(pair)) => Ok(Side::All(pair)),
            Ok(Repr::Parts(list)) => Side::parts(&list).ok_or_else(|| {
                de::Error::custom("a handle per part lists from 1 to 4 pairs [dt, dv], one for each part")
            }),
            Err(_) => Err(de::Error::custom(
                "a handle is [dt, dv] for every part or a list of [dt, dv], one for each part",
            )),
        }
    }
}

impl JsonSchema for Side {
    fn schema_name() -> Cow<'static, str> {
        "HandleSide".into()
    }

    fn json_schema(_: &mut SchemaGenerator) -> Schema {
        let pair =
            serde_json::json!({"type": "array", "items": {"type": "number"}, "minItems": 2, "maxItems": 2});
        let parts =
            serde_json::json!({"type": "array", "items": pair, "minItems": 1, "maxItems": MOST_PARTS});
        json_schema!({"anyOf": [pair, parts]})
    }
}
