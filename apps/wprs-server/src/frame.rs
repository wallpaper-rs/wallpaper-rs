use crate::App;
use crate::band::Shown;
use crate::lang::{current, t, tf};
use maud::{DOCTYPE, Markup, html};

pub const NO_WALLPAPER: &str = "f-ninguno";

pub struct Frame<'a> {
    pub title: &'a str,
    pub line: Option<Markup>,
}

impl<'a> Frame<'a> {
    pub fn new(title: &'a str) -> Self {
        Frame { title, line: None }
    }

    pub fn line(mut self, line: Markup) -> Self {
        self.line = Some(line);
        self
    }
}

fn credit(shown: &Shown) -> String {
    match &shown.by {
        Some(by) => tf("Cover: “{}” by {}, the latest published", &[&shown.title, by]),
        None => tf("Cover: “{}”, the latest published", &[&shown.title]),
    }
}

pub fn render(app: &App, nav: Markup, frame: Frame, body: Markup) -> Markup {
    let shown = app.band();
    let band = if shown.is_some() { "banda" } else { "banda sin" };
    html! {
        (DOCTYPE)
        html lang=(current().code()) class=[shown.is_none().then_some(NO_WALLPAPER)] {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (frame.title) " · " (app.cfg.name) }
                link rel="stylesheet" href="/static/style.css";
                @if let Some(s) = &shown { link rel="stylesheet" href={ "/band/" (s.sha) ".css" }; }
                script src="/static/copy.js" defer {}
            }
            body {
                a.saltar href="#contenido" { (t("Skip to content")) }
                div.pagina {
                    header class=(band) {
                        @if let Some(s) = &shown { img.portada-img src={ "/media/" (s.sha) ".jpg" } alt=""; }
                        div.barra {
                            a.marca href="/" { (app.cfg.name) }
                            nav.nav aria-label=(t("Site")) { (nav) }
                        }
                        div.cabeza {
                            h1 { (frame.title) }
                            @if let Some(line) = &frame.line { (line) }
                            @if let Some(s) = &shown { p.portada-de { (credit(s)) } }
                        }
                    }
                    main.columna #contenido { (body) }
                    footer.pie {
                        nav.pie-enlaces aria-label=(t("Legal")) {
                            a href="/legal" { (t("Legal notice")) }
                            a href="/legal/terms" { (t("Terms")) }
                            a href="/legal/privacy" { (t("Privacy")) }
                            a href="/legal/content" { (t("Content policy")) }
                            a href="/legal/notice" { (t("Report content")) }
                        }
                    }
                }
            }
        }
    }
}
