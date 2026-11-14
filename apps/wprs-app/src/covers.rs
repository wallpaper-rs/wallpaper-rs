use crate::grid;
use crate::items::qobject::ItemModel;
use crate::tr::tr;
use cxx_qt::CxxQtType;
use cxx_qt_lib::{QModelIndex, QString, QVariant};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::pin::Pin;
use wprs_core::model::ItemSummary;

#[derive(Default)]
pub struct Shown {
    pub on: BTreeMap<String, Vec<String>>,
    pub screens: usize,
}

impl Shown {
    pub fn read(outputs: &str) -> Self {
        let map: BTreeMap<String, String> = serde_json::from_str(outputs).unwrap_or_default();
        let mut on: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for (output, uid) in &map {
            if !uid.is_empty() {
                on.entry(uid.clone()).or_default().push(output.clone());
            }
        }
        Shown { on, screens: map.len() }
    }

    pub fn screens_of(&self, uid: &str) -> Option<&Vec<String>> {
        self.on.get(uid)
    }
}

pub fn caption(item: &ItemSummary) -> String {
    let kind = match grid::kind(item).as_str() {
        "scene" => tr("Scene"),
        "image" => tr("Image"),
        "video" => tr("Video"),
        "package" => tr("Package"),
        "assets" => tr("Assets"),
        other => other.to_owned(),
    };
    [kind, grid::info(item)].into_iter().filter(|t| !t.is_empty()).collect::<Vec<_>>().join(" · ")
}

pub fn note(item: &ItemSummary) -> (String, &'static str, &'static str) {
    let flashing = item.package.as_ref().is_some_and(|p| p.flash_disabled);
    if item.missing {
        (tr("The file is missing"), "warning-circle", "coral")
    } else if item.error.is_some() {
        (tr("The file is damaged"), "warning-circle", "coral")
    } else if item.revoked.is_some() {
        (tr("Withdrawn by its repository"), "prohibit", "coral")
    } else if flashing {
        (tr("Has flashes, turned off"), "warning", "ambar")
    } else {
        (String::new(), "", "")
    }
}

pub fn data(item: &ItemSummary, shown: &Shown, role: i32) -> QVariant {
    let text = |t: &str| QVariant::from(&QString::from(t));
    let on = shown.screens_of(item.uid.as_str());
    match role {
        15 => text(&caption(item)),
        16 => text(&note(item).0),
        17 => text(note(item).1),
        18 => text(note(item).2),
        19 => QVariant::from(&(note(item).2 == "ambar")),
        20 => QVariant::from(&on.is_some()),
        21 => text(&on.filter(|_| shown.screens > 1).map(|o| o.join(", ")).unwrap_or_default()),
        _ => QVariant::default(),
    }
}

impl ItemModel {
    pub fn showing(mut self: Pin<&mut Self>, outputs: &QString) {
        self.as_mut().rust_mut().shown = Shown::read(&outputs.to_string());
        let count = self.grid.rows.len();
        if count > 0 {
            let root = QModelIndex::default();
            let (first, last) = (self.index(0, 0, &root), self.index(count as i32 - 1, 0, &root));
            self.as_mut().data_changed(&first, &last, &Default::default());
        }
    }

    pub fn chosen(&self) -> QString {
        let rows: Vec<Value> =
            self.grid.rows.iter().enumerate().filter(|(_, r)| r.selected).map(|(i, _)| json!(i)).collect();
        QString::from(&Value::Array(rows).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn items() -> Vec<ItemSummary> {
        let fixtures: Value = serde_json::from_str(include_str!("fixtures.json")).unwrap();
        serde_json::from_value(fixtures["Library1.Search"]["items"].clone()).unwrap()
    }

    #[test]
    fn a_missing_file_gets_a_coral_note() {
        let lost = items().into_iter().find(|i| i.missing).unwrap();
        assert_eq!(note(&lost).2, "coral");
        let fine =
            items().into_iter().find(|i| !i.missing && i.error.is_none() && i.revoked.is_none()).unwrap();
        assert!(note(&fine).0.is_empty() || note(&fine).2 == "ambar");
    }

    #[test]
    fn the_screens_of_a_wallpaper_name_it_only_with_several_screens() {
        let shown = Shown::read(r#"{"eDP-1": "a", "HDMI-A-1": "b", "DP-2": ""}"#);
        assert_eq!(shown.screens, 3);
        assert_eq!(shown.screens_of("a").unwrap(), &vec!["eDP-1".to_owned()]);
        assert!(shown.screens_of("c").is_none());
        assert_eq!(Shown::read("{}").screens, 0);
    }

    #[test]
    fn a_caption_starts_with_the_kind() {
        let first = items().into_iter().next().unwrap();
        assert!(caption(&first).contains(" · "));
    }
}
