use crate::icons::icon;
use maud::{Markup, PreEscaped, html};

pub fn steps(label: &str, names: &[&str], current: usize) -> Markup {
    html! {
        ol.pasos aria-label=(label) {
            @for (i, name) in names.iter().enumerate() {
                @if i < current {
                    li { (icon("check-circle-fill")) (name) }
                } @else if i == current {
                    li.actual aria-current="step" { (name) }
                } @else {
                    li { (name) }
                }
            }
        }
    }
}

pub struct Choice<'a> {
    pub value: &'a str,
    pub title: &'a str,
    pub text: &'a str,
}

pub fn choices(legend: &str, name: &str, options: &[Choice], chosen: &str) -> Markup {
    html! {
        fieldset.elecciones {
            legend.oculto { (legend) }
            @for c in options {
                label.eleccion {
                    input type="radio" name=(name) value=(c.value) checked[c.value == chosen];
                    span { b { (c.title) } small { (c.text) } }
                }
            }
        }
    }
}

pub fn qr(svg: &str, alt: &str, text: Markup) -> Markup {
    html! {
        div.qr {
            div role="img" aria-label=(alt) { (PreEscaped(svg)) }
            div.qr-texto { (text) }
        }
    }
}

pub fn key_box(secret: &str, help: &str) -> Markup {
    html! {
        div.clave {
            p.dato { (secret) }
            p.ayuda { (help) }
        }
    }
}
