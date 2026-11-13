use crate::locale::has;
use crate::{Error, Manifest};
use std::collections::{BTreeMap, BTreeSet};
use wprs_scene::{Kind, Scene, Text, Words, tag};

fn told(layer: &wprs_scene::Layer) -> Option<(&str, &BTreeMap<String, String>)> {
    match &layer.kind {
        Kind::Text(t) => match &t.text {
            Words::Told(map) => Some((layer.id.as_str(), map)),
            Words::Said(_) => None,
        },
        _ => None,
    }
}

impl Manifest {
    pub fn scene_texts(&self, scene: &Scene) -> Vec<Error> {
        let found: Vec<_> = scene.drawable().filter_map(told).collect();
        let default = self.default_locale.as_deref();
        let invalid =
            |field: &str, reason: &str| Error::Invalid { field: field.into(), reason: reason.into() };
        let mut out = Vec::new();
        if default.is_none() && !found.is_empty() && self.v2() {
            out.push(invalid("default_locale", "is required with strings or localized texts"));
        }
        for (id, map) in found {
            if default.is_some_and(|d| !has(map, d)) {
                let field = format!("scene.layers.{id}.text");
                out.push(invalid(&field, "needs a text in the default language"));
            }
        }
        out
    }

    pub fn scene_locales(&self, scene: &Scene) -> BTreeSet<String> {
        let mut found = self.locales();
        for (_, map) in scene.drawable().filter_map(told) {
            found.extend(map.keys().map(|l| tag(l)));
        }
        found
    }

    pub fn scene_locale_in(&self, scene: &Scene, user: &[String]) -> String {
        let found = self.scene_locales(scene).into_iter().map(|l| (l.clone(), l)).collect();
        Text::Localized(found).pick(&self.languages(user))
    }
}
