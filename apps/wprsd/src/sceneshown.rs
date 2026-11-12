use crate::apply::target_output;
use crate::daemon::Daemon;
use crate::error::{Result, invalid, not_found};
use crate::sceneprops::Props;
use std::path::PathBuf;
use std::sync::Arc;

impl Daemon {
    pub fn props_on(&self, output: &str) -> Result<(String, Arc<Props>)> {
        let output = target_output(output)?;
        let (shown, path) = {
            let st = self.state();
            let info = st.outputs.get(&output).filter(|_| !output.is_empty());
            let shown =
                info.and_then(|i| i.scene.as_ref().map(|s| s.props.clone()).or_else(|| i.video.clone()));
            let path = if output.is_empty() {
                st.config.default.image.clone()
            } else {
                st.config.resolve(&output).image
            };
            (shown, path)
        };
        let missing = || not_found(crate::words::no_settings(&output));
        let package = |p: &PathBuf| wprs_library::Kind::detect(p) == Some(wprs_library::Kind::Package);
        let props = match (shown, path) {
            (Some(props), _) => props,
            (None, Some(path)) if package(&path) => self.package_props(&path).map_err(|_| missing())?,
            _ if output.is_empty() => self.props_anywhere()?,
            _ => return Err(missing()),
        };
        Ok((output, props))
    }

    fn props_anywhere(&self) -> Result<Arc<Props>> {
        let st = self.state();
        let real = st.outputs.iter().filter(|(name, _)| !crate::drafts::virtual_output(name));
        let mut found: Vec<Arc<Props>> = Vec::new();
        for (_, info) in real {
            let shown = info.scene.as_ref().map(|s| s.props.clone()).or_else(|| info.video.clone());
            if let Some(props) = shown.filter(|p| !found.iter().any(|f| f.id == p.id)) {
                found.push(props);
            }
        }
        match found.len() {
            0 => Err(not_found(crate::words::no_settings(""))),
            1 => Ok(found.remove(0)),
            _ => Err(invalid(crate::words::which_screen())),
        }
    }
}
