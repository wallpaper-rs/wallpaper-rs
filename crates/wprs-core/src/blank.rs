use crate::config::{Config, OutputConfig};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Kept {
    pub image: Option<PathBuf>,
    pub playlist: Option<String>,
    pub preset: Option<String>,
}

impl Kept {
    pub fn is_empty(&self) -> bool {
        self.image.is_none() && self.playlist.is_none()
    }
}

impl OutputConfig {
    fn put_away(&mut self, shown: Option<Kept>) {
        let own =
            Kept { image: self.image.take(), playlist: self.playlist.take(), preset: self.preset.take() };
        let kept = if own.is_empty() { shown.or(self.blank.take()) } else { Some(own) };
        self.blank = Some(kept.unwrap_or_default());
    }

    fn bring_back(&mut self) -> bool {
        let Some(kept) = self.blank.take() else { return false };
        let back = !kept.is_empty();
        (self.image, self.playlist, self.preset) = (kept.image, kept.playlist, kept.preset);
        back
    }
}

impl Config {
    pub fn is_blank(&self, output: &str) -> bool {
        let (image, playlist) = {
            let resolved = self.resolve(output);
            (resolved.image, resolved.playlist)
        };
        image.is_none() && playlist.is_none()
    }

    pub fn has_previous(&self, output: &str) -> bool {
        let kept = |o: &OutputConfig| o.blank.as_ref().is_some_and(|k| !k.is_empty());
        match output {
            "" => kept(&self.default) || self.outputs.values().any(kept),
            name => self.outputs.get(name).is_some_and(kept) || (kept(&self.default) && self.is_blank(name)),
        }
    }

    pub fn shown_among(&self, outputs: &[String], files: &[PathBuf]) -> Vec<String> {
        let shows = |name: &str| self.resolve(name).image.is_some_and(|i| files.contains(&i));
        let mut found: Vec<String> = Vec::new();
        for name in self.outputs.keys().chain(outputs) {
            if shows(name) && !found.contains(name) {
                found.push(name.clone());
            }
        }
        if self.default.image.as_ref().is_some_and(|i| files.contains(i)) {
            found.push(String::new());
        }
        found
    }

    pub fn clear_shown(&mut self, outputs: &[String], files: &[PathBuf]) {
        for name in self.shown_among(outputs, files) {
            match name.is_empty() {
                true => self.default.put_away(None),
                false => self.clear_content(&name),
            }
        }
    }

    pub fn restored_files(&self, output: &str) -> Vec<PathBuf> {
        let images = |o: &OutputConfig| o.image.clone();
        match output {
            "" => std::iter::once(&self.default).chain(self.outputs.values()).filter_map(images).collect(),
            name => self.resolve(name).image.into_iter().collect(),
        }
    }

    pub fn clear_content(&mut self, output: &str) {
        if output.is_empty() {
            self.default.put_away(None);
            self.outputs.values_mut().filter(|o| o.sets_content()).for_each(|o| o.put_away(None));
            return;
        }
        let shown = self.outputs.get(output).filter(|o| o.sets_content()).is_none().then(|| {
            let d = &self.default;
            Kept { image: d.image.clone(), playlist: d.playlist.clone(), preset: d.preset.clone() }
        });
        self.outputs.entry(output.to_owned()).or_default().put_away(shown);
    }

    pub fn restore_content(&mut self, output: &str) -> bool {
        let mut back = false;
        if output.is_empty() {
            back |= self.default.bring_back();
            back |= self.outputs.values_mut().fold(false, |any, o| o.bring_back() | any);
        } else if let Some(o) = self.outputs.get_mut(output) {
            back |= o.bring_back();
        }
        if !back && !output.is_empty() && self.default.blank.as_ref().is_some_and(|k| !k.is_empty()) {
            let mut kept = self.default.blank.clone().unwrap_or_default();
            let section = self.outputs.entry(output.to_owned()).or_default();
            (section.image, section.playlist, section.preset) =
                (kept.image.take(), kept.playlist.take(), kept.preset.take());
            back = section.sets_content();
        }
        self.outputs.retain(|_, o| *o != OutputConfig::default());
        back
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clearing_removed_files_blanks_only_the_screens_showing_them() {
        let mut config = Config::default();
        config.set_image("", "/a.png".into(), None);
        config.set_image("DP-2", "/b.png".into(), None);
        let known = vec!["DP-1".to_owned(), "DP-2".to_owned()];
        assert_eq!(config.shown_among(&known, &["/a.png".into()]), ["DP-1", ""]);
        config.clear_shown(&known, &["/a.png".into()]);
        assert!(config.is_blank("DP-1") && !config.is_blank("DP-2") && config.default.image.is_none());
        assert!(config.has_previous("DP-1"));
        assert!(config.restore_content("DP-1"));
        assert_eq!(config.restored_files("DP-1"), [PathBuf::from("/a.png")]);
    }
}
