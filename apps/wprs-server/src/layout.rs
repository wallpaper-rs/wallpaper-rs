use crate::App;
use crate::auth::{Auth, Role};
use crate::frame::{Frame, render};
use crate::lang::{t, tf};
use maud::{Markup, html};

pub fn nav(auth: Option<&Auth>, current: &str, sign_in: bool) -> Markup {
    let link = |href: &str, label: &str| {
        html! { a href=(href) aria-current=[(href == current).then_some("page")] class=(if href == current { "actual" } else { "" }) { (label) } }
    };
    html! {
        (link("/", t("Catalog")))
        @if let Some(a) = auth {
            (link("/account", &tf("Account ({})", &[&a.handle])))
            @if a.role >= Role::Moderator { (link("/mod", t("Moderation"))) }
            @if a.role >= Role::Admin { (link("/admin", t("Administration"))) }
            form method="post" action="/logout" { button { (t("Sign out")) } }
        } @else if sign_in {
            (link("/login", t("Sign in")))
        }
    }
}

pub fn page(app: &App, auth: Option<&Auth>, current: &str, title: &str, body: Markup) -> Markup {
    page_in(app, auth, Frame::new(title), current, body)
}

#[allow(dead_code)]
pub fn page_in(app: &App, auth: Option<&Auth>, frame: Frame, current: &str, body: Markup) -> Markup {
    render(app, nav(auth, current, true), frame, body)
}

pub fn setup_screen(app: &App, frame: Frame, body: Markup) -> Markup {
    render(app, nav(None, "", false), frame, body)
}

pub fn bare_page(app: &App, frame: Frame, body: Markup) -> Markup {
    render(app, html! {}, frame, body)
}

pub fn preview(src: &str, alt: &str, width: u32, mature: bool) -> Markup {
    html! {
        @if mature {
            label.veil {
                input type="checkbox" aria-label=(t("Show this mature preview"));
                img src=(src) alt=(alt) width=(width) loading="lazy";
                span { (t("Mature content: select to show the preview")) }
            }
        } @else {
            img src=(src) alt=(alt) width=(width) loading="lazy";
        }
    }
}
