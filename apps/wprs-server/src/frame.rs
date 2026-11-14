use crate::App;
use crate::lang::{current, t};
use maud::{DOCTYPE, Markup, html};

pub const NO_WALLPAPER: &str = "f-ninguno";

pub struct Cover<'a> {
    pub src: &'a str,
    pub credit: Option<&'a str>,
}

pub struct Frame<'a> {
    pub title: &'a str,
    pub line: Option<Markup>,
    pub tone: &'a str,
    pub cover: Option<Cover<'a>>,
}

impl<'a> Frame<'a> {
    pub fn new(title: &'a str) -> Self {
        Frame { title, line: None, tone: NO_WALLPAPER, cover: None }
    }

    pub fn line(mut self, line: Markup) -> Self {
        self.line = Some(line);
        self
    }

    #[allow(dead_code)]
    pub fn tone(mut self, tone: &'a str) -> Self {
        self.tone = tone;
        self
    }

    #[allow(dead_code)]
    pub fn cover(mut self, cover: Cover<'a>) -> Self {
        self.cover = Some(cover);
        self
    }
}

pub fn render(app: &App, nav: Markup, frame: Frame, body: Markup) -> Markup {
    let band = if frame.cover.is_some() { "banda" } else { "banda sin" };
    html! {
        (DOCTYPE)
        html lang=(current().code()) class=(frame.tone) {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
                title { (frame.title) " · " (app.cfg.name) }
                link rel="stylesheet" href="/static/style.css";
                script src="/static/copy.js" defer {}
            }
            body {
                a.saltar href="#contenido" { (t("Skip to content")) }
                div.pagina {
                    header class=(band) {
                        @if let Some(c) = &frame.cover { img.portada-img src=(c.src) alt="" ; }
                        div.barra {
                            a.marca href="/" { (app.cfg.name) }
                            nav.nav aria-label=(t("Site")) { (nav) }
                        }
                        div.cabeza {
                            h1 { (frame.title) }
                            @if let Some(line) = &frame.line { (line) }
                            @if let Some(credit) = frame.cover.as_ref().and_then(|c| c.credit) {
                                p.portada-de { (credit) }
                            }
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
