pub const SCRIPT_TEMPLATES: [&str; 4] = ["empty", "media", "touch", "timer"];

pub fn script_template(name: &str) -> Option<&'static str> {
    match name {
        "empty" => Some(include_str!("molds/empty.js")),
        "media" => Some(include_str!("molds/media.js")),
        "touch" => Some(include_str!("molds/touch.js")),
        "timer" => Some(include_str!("molds/timer.js")),
        _ => None,
    }
}

pub(crate) fn free_name(dir: &std::path::Path, template: &str) -> String {
    let taken = |name: &String| dir.join("scripts").join(name).exists();
    let names = std::iter::once(format!("{template}.js")).chain((2..).map(|n| format!("{template}-{n}.js")));
    names.take(1000).find(|n| !taken(n)).unwrap_or_else(|| format!("{template}.js"))
}
