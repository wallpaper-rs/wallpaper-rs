use crate::BatteryPause;
use toml_edit::{DocumentMut, Item, TableLike, value};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Platform {
    #[default]
    Desktop,
    Phone,
}

pub(crate) fn upgrade(text: &str, platform: Platform) -> Option<String> {
    let mut file = text.parse::<DocumentMut>().ok()?;
    let playback = upgrade_playback(&mut file);
    let pause = upgrade_pause(&mut file, platform);
    (playback || pause).then(|| file.to_string())
}

fn section<'a>(file: &'a mut DocumentMut, name: &str) -> Option<&'a mut dyn TableLike> {
    file.get_mut(name).and_then(Item::as_table_like_mut)
}

fn upgrade_playback(file: &mut DocumentMut) -> bool {
    let Some(playback) = section(file, "playback") else { return false };
    let Some(guard) = playback.remove("flash_guard") else { return false };
    if guard.as_bool() == Some(true) && !playback.contains_key("allow_flashing") {
        playback.insert("allow_flashing", value(false));
    }
    true
}

fn upgrade_pause(file: &mut DocumentMut, platform: Platform) -> bool {
    let Some(pause) = section(file, "pause") else { return false };
    let below = pause.remove("below_percent");
    let always = match pause.get("on_battery").map(Item::as_bool) {
        Some(Some(on)) => on,
        Some(None) => return below.is_some(),
        None if below.is_some() => false,
        None => return false,
    };
    let percent = below.and_then(|v| v.as_integer()).unwrap_or_default().clamp(0, 100) as u8;
    let choice = match (platform, always, percent) {
        (Platform::Desktop, true, _) => BatteryPause::Always,
        (_, _, 0) => BatteryPause::Never,
        (_, _, n) => BatteryPause::Below(n),
    };
    pause.insert("on_battery", value(choice.word()));
    true
}
