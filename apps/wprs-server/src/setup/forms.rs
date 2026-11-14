use super::view::{stepped, summary};
use crate::App;
use crate::lang::{t, tf};
use crate::settings::Access;
use crate::ui::{Choice, Kind, Tone, button, checkbox, choices, field, fold, key_box, notice, qr};
use maud::{Markup, html};

pub fn account(app: &App, handle: Option<&str>, error: Option<&str>) -> Markup {
    let body = html! {
        (summary(app, 1))
        section {
            h2.titulo-seccion { (t("Administrator account")) }
            p.intro { (t("With this account you get in to manage, review and upload.")) }
            form.formulario.separado method="post" action="/setup/account" {
                (field(t("Handle"), html! {
                    input name="handle" autocomplete="username" required minlength="3" maxlength="32" value=[handle]
                        aria-invalid=[error.map(|_| "true")];
                }, Some(t("3 to 32 characters.")), error))
                (field(t("Password"), html! {
                    input type="password" name="password" autocomplete="new-password" required minlength="10";
                }, Some(t("10 characters or more.")), None))
                div { (button(Kind::Primary, t("Next"))) }
            }
        }
    };
    stepped(app, t("Set up your server"), 1, body)
}

pub fn repository(app: &App, name: Option<&str>) -> Markup {
    let error = name.map(|_| t("Write a name of 1 to 80 characters."));
    let body = html! {
        (summary(app, 2))
        section {
            h2.titulo-seccion { (t("Create the repository")) }
            p.intro { (t("It is the collection the app and the phone see when they connect to this server.")) }
            form.formulario.separado method="post" action="/setup/repository" {
                (field(t("Repository name"), html! {
                    input name="name" required maxlength="80" value=[name] aria-invalid=[error.map(|_| "true")];
                }, None, error))
                (fold(t("Advanced"), checkbox(html! { input type="checkbox" name="online_authors" checked; },
                    t("Online authors"),
                    Some(t("Each person adds their author keys from their account, without touching the root key."))), false))
                div { (button(Kind::Primary, t("Create the repository"))) }
            }
        }
    };
    stepped(app, t("Set up your server"), 2, body)
}

pub fn key(app: &App, held: Option<&(String, String)>, missing: Option<()>) -> Markup {
    let Some((name, secret)) = held else {
        let body = html! {
            (notice(Tone::Warn, t("The root key is no longer shown."),
                html! { p.texto { (t("The server started again before you marked «I saved it», and the key is never kept. Create the repository again to get a new key.")) } },
                Some(html! { form method="post" action="/setup/again" { (button(Kind::Primary, t("Create it again"))) } })))
        };
        return stepped(app, t("Set up your server"), 2, body);
    };
    let body = html! {
        section {
            h2.titulo-seccion { (t("Save the root key")) }
            p.intro { (t("It is shown only this time. Keep it outside this server: in a password manager or on paper.")) }
            form.formulario.ancho.separado method="post" action="/setup/saved" {
                (key_box(secret, &tf("Root key of «{}». Select it to copy it.", &[name])))
                (checkbox(html! { input type="checkbox" name="saved" required aria-invalid=[missing.map(|_| "true")]; }, t("I saved it"), None))
                div.acciones {
                    (button(Kind::Primary, t("Next")))
                    span.porque { (t("Mark «I saved it» to go on.")) }
                }
            }
        }
    };
    stepped(app, name, 2, body)
}

pub fn access(app: &App, code: &str) -> Markup {
    let settings = app.settings();
    let options = [
        Choice {
            value: "closed",
            title: t("Closed"),
            text: t("Nobody signs up on their own. What is uploaded is published at once."),
        },
        Choice {
            value: "invite",
            title: t("By invitation"),
            text: t("Whoever has an invitation from you gets in. What is uploaded is published at once."),
        },
        Choice {
            value: "open",
            title: t("Open"),
            text: t("Anyone can sign up. What new accounts upload waits for your review."),
        },
    ];
    let chosen = match settings.access() {
        Access::Closed => "invite",
        other => other.value(),
    };
    let link = app.cfg.url(&format!("/register?invite={code}"));
    let svg = crate::qr::own_connect_svg(app, Some(code)).unwrap_or_default();
    let name = settings.repo_name.clone().unwrap_or_default();
    let body = html! {
        (summary(app, 3))
        section {
            h2.titulo-seccion { (t("Who gets in")) }
            form.formulario.ancho.separado method="post" action="/setup/access" {
                (choices(t("Who gets in"), "access", &options, chosen))
                div {
                    h3.subtitulo { (t("Your first invitation")) }
                    (qr(&svg, t("QR code with the invitation link"), html! {
                        b { (t("Give it to whoever you want to get in.")) }
                        span.texto { (t("It is shown only this time; you can make more in People.")) }
                        span.dato { (link) }
                    }))
                }
                div { (button(Kind::Primary, t("Finish"))) }
            }
        }
    };
    stepped(app, if name.is_empty() { t("Set up your server") } else { &name }, 3, body)
}
