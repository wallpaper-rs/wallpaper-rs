use serde::Deserialize;

#[derive(Deserialize, Default, Clone, Copy)]
#[serde(default)]
pub(crate) struct Moved {
    pub offset: Option<[f32; 2]>,
    pub rotation: Option<f32>,
    pub scale: Option<[f32; 2]>,
    #[serde(rename = "texCoord")]
    pub set: Option<usize>,
}

#[derive(Deserialize, Default, Clone, Copy)]
#[serde(default)]
pub(crate) struct Moves {
    #[serde(rename = "KHR_texture_transform")]
    pub moved: Option<Moved>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub(crate) struct Sampling {
    #[serde(rename = "magFilter")]
    pub mag: Option<u32>,
    #[serde(rename = "minFilter")]
    pub min: Option<u32>,
    #[serde(rename = "wrapS")]
    pub s: Option<u32>,
    #[serde(rename = "wrapT")]
    pub t: Option<u32>,
}
