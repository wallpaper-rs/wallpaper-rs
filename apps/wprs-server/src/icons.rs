use maud::{Markup, PreEscaped, html};

macro_rules! icons {
    ($($name:literal),* $(,)?) => {
        fn svg(name: &str) -> &'static str {
            match name {
                $($name => include_str!(concat!("../assets/icons/", $name, ".svg")),)*
                _ => "",
            }
        }
        #[cfg(test)]
        pub const NAMES: &[&str] = &[$($name),*];
    };
}

icons!(
    "arrow-left",
    "check-circle-fill",
    "copy",
    "download-simple",
    "folder-open",
    "info",
    "link",
    "upload-simple",
    "warning-fill",
    "warning-octagon-fill",
);

pub fn icon(name: &str) -> Markup {
    html! { i.ms aria-hidden="true" { (PreEscaped(svg(name))) } }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_icon_is_an_inline_svg_that_follows_the_text_colour() {
        for name in NAMES {
            let markup = icon(name).into_string();
            assert!(markup.contains("<svg") && markup.contains("currentColor"), "{name}");
        }
        assert!(!icon("no-such-icon").into_string().contains("<svg"));
    }
}
