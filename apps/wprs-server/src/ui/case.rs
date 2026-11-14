use crate::icons::icon;
use maud::{Markup, html};

pub fn case_head(src: &str, alt: &str, title: &str, foot: &str) -> Markup {
    html! {
        img.caso-portada src=(src) alt=(alt);
        h2.caso-titulo { (title) }
        p.caso-pie { (foot) }
    }
}

pub fn back_link(href: &str, label: &str) -> Markup {
    html! { a.volver href=(href) { (icon("arrow-left")) (label) } }
}

pub fn subheading(text: &str) -> Markup {
    html! { h3.subtitulo { (text) } }
}

pub fn confirm(question: &str, aria: &str, action: &str, actions: Markup) -> Markup {
    html! {
        form.confirmar method="post" action=(action) role="group" aria-label=(aria) {
            p { (question) }
            div.acciones { (actions) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_case_pieces_carry_their_kit_classes() {
        let head = case_head("/media/a.jpg", "Preview", "Dawn", "Scene").into_string();
        for part in
            ["class=\"caso-portada\"", "class=\"caso-titulo\"", "class=\"caso-pie\"", "alt=\"Preview\""]
        {
            assert!(head.contains(part), "{part}");
        }
        assert!(back_link("/mod", "Review").into_string().contains("class=\"volver\""));
        let ask = confirm("Sure?", "Confirm", "/x", html! { button { "Yes" } }).into_string();
        assert!(
            ask.contains("class=\"confirmar\"")
                && ask.contains("role=\"group\"")
                && ask.contains("action=\"/x\"")
        );
    }
}
