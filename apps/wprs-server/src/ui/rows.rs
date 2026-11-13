use super::Tone;
use crate::icons::icon;
use maud::{Markup, html};

pub struct Row {
    pub name: Markup,
    pub value: Option<Markup>,
    pub action: Option<Markup>,
    pub lit: bool,
}

impl Row {
    pub fn new(name: Markup) -> Self {
        Row { name, value: None, action: None, lit: false }
    }

    pub fn value(mut self, value: Markup) -> Self {
        self.value = Some(value);
        self
    }

    pub fn action(mut self, action: Markup) -> Self {
        self.action = Some(action);
        self
    }

    pub fn lit(mut self) -> Self {
        self.lit = true;
        self
    }
}

pub fn rows(items: Vec<Row>, compact: bool) -> Markup {
    html! {
        ul.filas.compactas[compact] {
            @for r in items {
                li.fila.foco[r.lit] {
                    div.fila-texto {
                        div.fila-nombre { (r.name) }
                        @if let Some(v) = r.value { div.fila-valor { (v) } }
                    }
                    @if let Some(a) = r.action { div.fila-accion { (a) } }
                }
            }
        }
    }
}

pub fn status(tone: Tone, text: &str) -> Markup {
    html! { span class={ "estado " (tone.class()) } { (icon(tone.icon())) (text) } }
}

pub fn status_line(tone: Tone, text: &str, detail: Option<&str>) -> Markup {
    html! {
        p class={ "linea-estado " (tone.class()) } {
            (icon(tone.icon())) (text)
            @if let Some(d) = detail { span.tenue { (d) } }
        }
    }
}

pub fn pairs(items: &[(&str, Markup)]) -> Markup {
    html! { dl.pares { @for (k, v) in items { dt { (k) } dd { (v) } } } }
}
