use super::Tone;
use crate::icons::icon;
use maud::{Markup, html};

pub fn section(title: &str, body: Markup) -> Markup {
    html! { section.seccion { h2 { (title) } (body) } }
}

pub fn notice(tone: Tone, title: &str, body: Markup, actions: Option<Markup>) -> Markup {
    html! {
        div class={ "nota " (tone.class()) } role=(if tone == Tone::Bad { "alert" } else { "status" }) {
            (icon(tone.icon()))
            div.nota-cuerpo {
                b { (title) }
                (body)
                @if let Some(a) = actions { div.acciones { (a) } }
            }
        }
    }
}

pub fn empty(title: &str, first_step: Markup) -> Markup {
    html! { div.vacio { b { (title) } p { (first_step) } } }
}

pub fn fold(summary: &str, body: Markup, big: bool) -> Markup {
    html! { details.mas.grande[big] { summary { (summary) } div.mas-cuerpo { (body) } } }
}

pub fn copyable(text: &str, label: &str, done: &str) -> Markup {
    html! {
        div.copiar {
            code.dato { (text) }
            button.boton type="button" hidden data-copied=(done) { (icon("copy")) span data-label { (label) } }
        }
    }
}
