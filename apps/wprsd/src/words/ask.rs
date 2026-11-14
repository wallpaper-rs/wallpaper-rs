use super::tr;

pub fn answers() -> (String, String) {
    (tr("Allow"), tr("Not now"))
}

pub fn request(title: &str, input: wprs_core::model::Input) -> (String, String) {
    let safe = |c| match c {
        '<' => '‹',
        '>' => '›',
        '&' => '＆',
        c => c,
    };
    let title: String = title.chars().map(safe).collect();
    match input {
        wprs_core::model::Input::Media => (
            tr("Let a wallpaper show what plays?"),
            tr("Let “%1” show the title, artist, album, cover and progress of what the media player plays? Nothing of it is kept, logged or sent.").replace("%1", &title),
        ),
        wprs_core::model::Input::Audio => (
            tr("Let a wallpaper follow the sound?"),
            tr("Let “%1” follow the sound that plays? The sound is turned into levels on this computer; nothing is recorded, kept or sent.").replace("%1", &title),
        ),
        _ => (
            tr("Let a wallpaper follow the pointer?"),
            tr("Let “%1” follow the pointer? Only its latest position over the screen reaches the wallpaper, and it is never kept.").replace("%1", &title),
        ),
    }
}

pub fn flashing_disabled() -> String {
    tr(
        "This wallpaper flashes or shows fast patterns that can trigger seizures in people with photosensitive epilepsy, and wallpapers that flash are turned off: allow them in the settings first.",
    )
}

pub fn request_group(title: &str, inputs: &[wprs_core::model::Input]) -> (String, String) {
    if let [only] = inputs {
        return request(title, *only);
    }
    let safe = |c| match c {
        '<' => '‹',
        '>' => '›',
        '&' => '＆',
        c => c,
    };
    let title: String = title.chars().map(safe).collect();
    let name = |input: &wprs_core::model::Input| match input {
        wprs_core::model::Input::Media => tr("what plays"),
        wprs_core::model::Input::Audio => tr("the sound"),
        wprs_core::model::Input::Clicks => tr("clicks"),
        _ => tr("the pointer"),
    };
    let list = inputs.iter().map(name).collect::<Vec<_>>().join(", ");
    (
        tr("Let a wallpaper use these?"),
        tr("“%1” asks to use: %2. Until you answer it runs without them, and nothing is kept or sent.")
            .replace("%1", &title)
            .replace("%2", &list),
    )
}
