use anyhow::{Context, Result};
use serde_json::{Value, json};
use wprs_editor::actions::Edit3d;

const EDIT: &str = "One edit as JSON, as the app sends it (see docs/specs/editor.md)";

#[derive(Debug, clap::Args)]
pub struct Edits3d {
    #[arg(help = EDIT, long, value_name = "JSON")]
    model: Option<String>,
    #[arg(help = EDIT, long, value_name = "JSON")]
    group: Option<String>,
    #[arg(help = EDIT, long, value_name = "JSON")]
    space: Option<String>,
    #[arg(help = EDIT, long, value_name = "JSON")]
    depth_test: Option<String>,
    #[arg(help = EDIT, long, value_name = "JSON")]
    material: Option<String>,
    #[arg(help = EDIT, long, value_name = "JSON")]
    paint: Option<String>,
    #[arg(help = EDIT, long, value_name = "JSON")]
    camera: Option<String>,
    #[arg(help = EDIT, long, value_name = "JSON")]
    view: Option<String>,
    #[arg(help = EDIT, long, value_name = "JSON")]
    tones: Option<String>,
    #[arg(help = EDIT, long, value_name = "JSON")]
    shadows: Option<String>,
    #[arg(help = EDIT, long, value_name = "JSON")]
    path: Option<String>,
    #[arg(help = EDIT, long, value_name = "JSON")]
    new_material: Option<String>,
    #[arg(help = EDIT, long, value_name = "JSON")]
    debug: Option<String>,
    #[arg(help = EDIT, long, value_name = "JSON")]
    key: Option<String>,
    #[arg(help = EDIT, long, value_name = "JSON")]
    comb: Option<String>,
}

impl Edits3d {
    pub fn edit(self) -> Result<Edit3d> {
        let all = [
            ("model", self.model),
            ("group", self.group),
            ("space", self.space),
            ("depth_test", self.depth_test),
            ("material", self.material),
            ("paint", self.paint),
            ("camera", self.camera),
            ("view", self.view),
            ("tones", self.tones),
            ("shadows", self.shadows),
            ("path", self.path),
            ("new_material", self.new_material),
            ("debug", self.debug),
            ("key", self.key),
            ("comb", self.comb),
        ];
        let given: Vec<(&str, String)> = all.into_iter().filter_map(|(k, v)| Some((k, v?))).collect();
        anyhow::ensure!(given.len() == 1, "give exactly one of the 3D edits");
        let (key, text) = &given[0];
        let value: Value = serde_json::from_str(text).with_context(|| format!("{text} is not JSON"))?;
        Ok(serde_json::from_value(json!({ *key: value }))?)
    }
}
