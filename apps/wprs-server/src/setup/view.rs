use crate::App;
use crate::certs::Checked;
use crate::frame::Frame;
use crate::lang::{t, tf, tm};
use crate::layout::setup_screen;
use crate::settings::{Network, Settings};
use crate::ui::{Choice, Kind, Row, Tone, button, choices, field, fold, notice, rows, status, steps};
use maud::{Markup, html};

pub use super::forms::{access, account, key, repository};

pub fn frame_line(step: usize) -> Markup {
    html! { p.linea-estado { span.tenue { (tf("Setting up the server · step {} of 4", &[&(step + 1)])) } } }
}

pub fn stepped(app: &App, title: &str, step: usize, body: Markup) -> Markup {
    let names = [t("Network"), t("Account"), t("Repository"), t("Who gets in")];
    let body = html! { (steps(t("Setup steps"), &names, step)) (body) };
    setup_screen(app, Frame::new(title).line(frame_line(step)), body)
}

pub fn summary(app: &App, upto: usize) -> Markup {
    let settings = app.settings();
    let change = |href: &str| html! { a.boton.enlace href=(href) { (t("Change")) } };
    let mut items = vec![
        Row::new(html! { (t("Network")) })
            .value(html! { (network_name(&settings.network)) })
            .action(change("/setup?step=network")),
    ];
    if upto > 1 {
        let handle = settings.admin_handle.clone().unwrap_or_default();
        items.push(
            Row::new(html! { (t("Account")) })
                .value(html! { (handle) })
                .action(change("/setup?step=account")),
        );
    }
    if upto > 2 {
        let name = settings.repo_name.clone().unwrap_or_default();
        items.push(Row::new(html! { (t("Repository")) }).value(html! { (name) }));
    }
    html! { div.resumen { (rows(items, true)) } }
}

pub fn network_name(n: &Network) -> &'static str {
    match n {
        Network::Home => t("Only at home"),
        Network::Own { .. } => t("I have my certificate"),
    }
}

pub fn code(app: &App, wrong: Option<&str>) -> Markup {
    let line = html! {
        p.linea-estado {
            (status(Tone::Good, t("It is running")))
            span.tenue { (t("It still needs setting up: four steps")) }
        }
    };
    let error = wrong.map(|_| {
        t("This code does not work: it is copied wrong, it was used already or more than 24 hours went by. \
           Ask for a new one and paste it again.")
    });
    let input = html! { input name="token" autocomplete="off" required value=[wrong] aria-invalid=[wrong.map(|_| "true")]; };
    let help = error.is_none().then(|| t("The server writes it in its messages the first time it starts."));
    let where_ = html! {
        p.texto { (tm("On the server's computer, open its folder and look at its messages with {}: look for the line «setup token». If more than 24 hours went by or you used it already, ask for a new one in that folder:",
            &[html! { span.dato { "docker compose logs" } }])) }
        pre.dato { "wprs-server setup-token" }
    };
    let body = html! {
        div.vacio {
            b { (t("Your server is running.")) }
            p { (t("Four steps are left before other people install wallpapers from here. To start, write the setup token.")) }
        }
        form.formulario.separado method="post" action="/setup" {
            (field(t("Setup token"), input, help, error))
            details.mas open[wrong.is_some()] { summary { (t("Where the code is")) } div.mas-cuerpo { (where_) } }
            div { (button(Kind::Primary, t("Start"))) }
            @if wrong.is_some() { (fold(t("Details"), html! { pre.dato { "403 Forbidden: the setup token is wrong, used or expired" } }, false)) }
        }
    };
    setup_screen(app, Frame::new(t("Your wallpaper server")).line(line), body)
}

pub enum NetFault {
    Plain,
    Cert(Checked),
}

pub fn network(app: &App, settings: &Settings, fault: Option<(&Network, NetFault)>) -> Markup {
    let chosen = fault.as_ref().map_or(&settings.network, |(n, _)| *n);
    let (cert, key) = match chosen {
        Network::Own { cert, key } => (cert.as_str(), key.as_str()),
        Network::Home => ("", ""),
    };
    let value = if *chosen == Network::Home { "home" } else { "own" };
    let options = [
        Choice {
            value: "home",
            title: t("Only at home"),
            text: t(
                "For the devices of your network. The server makes its own certificate and announces itself on the local network; each phone recognises it by reading a QR code at the end.",
            ),
        },
        Choice {
            value: "own",
            title: t("I have my certificate"),
            text: t("If you already have a name of your own and a certificate for it."),
        },
    ];
    let checked = match fault.as_ref() {
        Some((_, NetFault::Cert(c))) => Some(c),
        _ => None,
    };
    let cert_error = checked.and_then(|c| c.cert.as_ref()).map(|f| f.message());
    let key_error = checked.and_then(|c| c.key.as_ref()).map(|f| f.message());
    let path = |name: &str, value: &str, bad: bool| {
        html! { input name=(name) value=(value) autocomplete="off" aria-invalid=[bad.then_some("true")]; }
    };
    let body = html! {
        section {
            h2.titulo-seccion { (t("How do the others reach this server?")) }
            @match fault.as_ref().map(|(_, f)| f) {
                Some(NetFault::Plain) => (notice(Tone::Bad, t("This server's public address starts with http://."),
                    html! { p.texto { (t("Phones and the app need https://. Change WPRS_PUBLIC_URL in the server's .env file, start it again and come back.")) } }, None)),
                Some(NetFault::Cert(_)) => (notice(Tone::Bad, t("With this certificate the others could not get in."),
                    html! { p.texto { (t("Correct what is marked below or choose «Only at home».")) } }, None)),
                None => {}
            }
            form.formulario.ancho.separado method="post" action="/setup/network" {
                (choices(t("Network"), "network", &options, value))
                div.campos-ruta {
                    (field(t("Certificate file"), path("cert", cert, cert_error.is_some()),
                        Some(t("The path inside the server. Leave both empty when the program in front of this server holds the certificate.")),
                        cert_error.as_deref()))
                    (field(t("Key file"), path("key", key, key_error.is_some()), None, key_error.as_deref()))
                    p.texto { (status(Tone::Warn, t("If the certificate comes from an authority of your own and not from a public one, each phone needs it installed before connecting."))) }
                }
                div { (button(Kind::Primary, if checked.is_some() { t("Check again") } else { t("Next") })) }
                @if let Some(c) = checked.filter(|c| !c.detail.is_empty()) {
                    (fold(t("Details"), html! { pre.dato { (c.detail.join("\n")) } }, false))
                }
            }
        }
    };
    stepped(app, t("Set up your server"), 0, body)
}

pub fn network_failed(app: &App, chosen: &Network, fault: NetFault) -> Markup {
    network(app, &app.settings(), Some((chosen, fault)))
}
