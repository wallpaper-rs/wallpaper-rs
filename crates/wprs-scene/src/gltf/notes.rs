use super::json::Doc;
use serde::Deserialize;

#[derive(Deserialize, Default)]
#[serde(default)]
pub(super) struct Surface {
    pub name: Option<String>,
    #[serde(rename = "pbrMetallicRoughness")]
    pub pbr: Option<Pbr>,
    #[serde(rename = "normalTexture")]
    pub normal: Option<Slot>,
    #[serde(rename = "occlusionTexture")]
    pub occlusion: Option<Slot>,
    #[serde(rename = "emissiveTexture")]
    pub glow: Option<Slot>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub(super) struct Pbr {
    #[serde(rename = "baseColorTexture")]
    pub base: Option<Slot>,
    #[serde(rename = "metallicRoughnessTexture")]
    pub orm: Option<Slot>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub(super) struct Slot {
    #[serde(rename = "texCoord")]
    pub set: u32,
    pub extensions: Moves,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub(super) struct Moves {
    #[serde(rename = "KHR_texture_transform")]
    pub moved: Option<Moved>,
}

#[derive(Deserialize, Default, Clone, Copy)]
#[serde(default)]
pub(super) struct Moved {
    #[serde(rename = "texCoord")]
    pub set: Option<u32>,
}

pub(super) fn notes(doc: &Doc) -> Vec<String> {
    let mut out = Vec::new();
    let slots = |m: &Surface| {
        let pbr = m.pbr.as_ref();
        [
            pbr.and_then(|p| p.base.as_ref()),
            pbr.and_then(|p| p.orm.as_ref()),
            m.normal.as_ref(),
            m.occlusion.as_ref(),
            m.glow.as_ref(),
        ]
        .into_iter()
        .flatten()
        .any(|s| s.set > 1 || s.extensions.moved.is_some_and(|m| m.set.is_some_and(|k| k > 1)))
    };
    if let Some(m) = doc.materials.iter().find(|m| slots(m)) {
        let name = m.name.clone().unwrap_or_else(|| "a material".to_owned());
        out.push(format!("{name} reads a third set of texture coordinates, drawn with the first"));
    }
    out
}
