use crate::steps::edit_all;
use anyhow::{Context, Result, bail};
use clap::Args;
use serde_json::{Map, Value, json};
use std::path::{Path, PathBuf};
use wprs_editor::actions::Action;

#[derive(Args, Debug, Default)]
pub struct Starting {
    #[arg(long, value_name = "ID", help = "With --from preset: the package the preset is for")]
    base: Option<String>,
    #[arg(
        long,
        requires = "base",
        help = "With --base: the versions of the base it suits, such as ^1.2; any version when omitted"
    )]
    range: Option<String>,
    #[arg(
        long,
        value_name = "FILE",
        requires = "base",
        help = "With --base: a JSON file with the values the preset sets, by setting id"
    )]
    values: Option<PathBuf>,
    #[arg(long, value_name = "ID", help = "With --from preset: license of the preset, such as CC-BY-4.0")]
    license: Option<String>,
    #[arg(long, value_name = "FILE", help = "With --from preset: a picture to keep as its thumbnail")]
    preview: Option<PathBuf>,
}

impl Starting {
    fn used(&self) -> bool {
        self.base.is_some() || self.license.is_some() || self.preview.is_some()
    }

    pub fn allowed(&self, from: &str) -> Result<()> {
        if self.used() && from != "preset" {
            bail!("--base, --range, --values, --license and --preview start a preset: add --from preset");
        }
        Ok(())
    }

    pub fn apply(self, dir: &Path) -> Result<()> {
        let mut actions = Vec::new();
        if let Some(id) = &self.base {
            let values: Map<String, Value> = match &self.values {
                Some(file) => serde_json::from_slice(&std::fs::read(file).context("read the values file")?)
                    .context("the values file is not a JSON object")?,
                None => Map::new(),
            };
            let base = json!({ "id": id, "version": self.range });
            actions.push(serde_json::from_value(
                json!({ "manifest": { "preset": { "base": base, "values": values } } }),
            )?);
        }
        if let Some(license) = self.license {
            actions.push(Action::Set { pointer: "/manifest/license".into(), value: json!(license) });
        }
        if let Some(file) = self.preview {
            actions.push(Action::Thumbnail {
                file: std::fs::canonicalize(&file).unwrap_or(file),
                animated: false,
            });
        }
        if actions.is_empty() {
            return Ok(());
        }
        edit_all(dir, actions, false)
    }
}
