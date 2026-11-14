use super::facts::Facts;
use super::mail::{Found, Mail};
use super::mail_row::{self, Asked};
use super::rows::{self, Line};
use crate::icons::icon;
use crate::lang::{count, since, t, tf, tm};
use crate::ui::{Kind, Row, Tone, copyable, empty, link_button, notice, qr, rows as list, status};
use maud::{Markup, html};

pub struct Extras {
    pub phone_svg: String,
    pub fingerprint: bool,
    pub repo_url: Option<String>,
    pub featured: i64,
    pub failed: Option<String>,
    pub advanced: bool,
    pub asked: Asked,
    pub tested: Option<String>,
}

pub fn lines(facts: &Facts, mail: &Found, asked: &Asked, failed: bool) -> Vec<Line> {
    let mut out = Vec::new();
    out.extend(facts.cert.as_ref().map(rows::certificate));
    out.extend(facts.sign.as_ref().map(|s| rows::signature(s, failed)));
    out.push(rows::backup(facts.backup));
    out.extend(facts.disk.as_ref().map(rows::disk));
    out.push(mail_row::mail(mail, asked));
    out.push(rows::cases(facts.cases, facts.reports, facts.last_report));
    out.push(rows::invites(facts.invites, facts.first_invite));
    out
}

fn headline(facts: &Facts, lines: &[Line], failed: bool) -> Markup {
    let seen = |tone| lines.iter().filter(|l| l.tone == Some(tone)).count() as i64;
    let (bad, near) = (seen(Tone::Bad), seen(Tone::Warn));
    let published = facts.published.map(|p| since((crate::db::now() - p) / 1000));
    let tail = if facts.packages == 0 {
        let signed = since((crate::db::now() - facts.signed_at) / 1000);
        tf("no wallpapers yet · signed {}", &[&signed])
    } else {
        let mut parts = vec![count(facts.packages, "{} wallpaper", "{} wallpapers")];
        parts.extend(published.map(|p| tf("last published {}", &[&p])));
        parts.extend(facts.sign.as_ref().map(|s| tf("next signing {}", &[&since(-s.next_in)])));
        parts.join(" · ")
    };
    let (tone, text) = match (failed, bad, near) {
        (true, ..) => (Tone::Bad, t("The signing could not be done").to_string()),
        (_, 0, 0) if facts.packages == 0 => (Tone::Good, t("Ready for the first wallpaper").to_string()),
        (_, 0, 0) => (Tone::Good, t("All in order").to_string()),
        (_, 0, n) => (Tone::Warn, tf("{} to check", &[&n])),
        (_, b, 0) => (Tone::Bad, tf("{} to fix", &[&b])),
        (_, b, n) => (Tone::Bad, tf("{} to fix, {} to check", &[&b, &n])),
    };
    html! { p.linea-estado { (status(tone, &text)) span.tenue { (tail) } } }
}

fn failure(facts: &Facts, raw: &str) -> Markup {
    let seen = facts.published.map(|p| since((crate::db::now() - p) / 1000)).unwrap_or_default();
    let again =
        html! { form method="post" action="/admin/sign" { button.boton.principal { (t("Sign again")) } } };
    let body = html! {
        p.texto { (tf("Fix what the details say and try again. Meanwhile the apps keep seeing what was published {}.", &[&seen])) }
        details.mas { summary { (t("Details")) } div.mas-cuerpo { pre.dato { (raw) } } }
    };
    notice(Tone::Bad, t("The index could not be signed."), body, Some(again))
}

fn phones(extras: &Extras) -> Markup {
    let (read, alt) = if extras.fingerprint {
        (
            t("It brings the address and the fingerprint of the certificate."),
            t("QR code with the address and the fingerprint of the server"),
        )
    } else {
        (t("It brings the address of the server."), t("QR code with the address of the server"))
    };
    html! {
        section.seccion #telefonos aria-labelledby="tel" {
            h2 #tel { (t("For the phones")) }
            (qr(&extras.phone_svg, alt, html! { b { (t("The phone reads this code.")) } span.texto { (read) } }))
            @if let Some(url) = &extras.repo_url {
                (copyable(url, t("Copy the address"), t("Copied")))
            }
        }
    }
}

fn advanced(extras: &Extras) -> Markup {
    let download =
        html! { a.boton href="/api/v1/admin/transparency" { (icon("download-simple")) (t("Download")) } };
    let items = vec![
        Row::new(html! { (t("Transparency report")) })
            .value(html! { (t("Notices and decisions, in a spreadsheet")) })
            .action(download),
        Row::new(html! { (t("Audit log")) })
            .value(html! { (t("Every moderation decision, with date and reason")) })
            .action(link_button(Kind::Plain, "/api/v1/mod/audit", t("Open"))),
        Row::new(html! { (t("Featured wallpapers")) })
            .value(html! { (tf("{}, they are signed into the repository", &[&extras.featured])) })
            .action(link_button(Kind::Plain, "/admin/featured", t("Open"))),
    ];
    html! {
        details.mas.grande.seccion #avanzado open[extras.advanced] {
            summary { (t("Advanced")) " " span.ayuda { (t("logs and featured")) } }
            div.mas-cuerpo {
                @if extras.advanced {
                    p.texto { (tm("The space of each account is now in {}.", &[html! { a href="/admin/personas#accounts" { (t("People")) } }])) }
                    h3.subtitulo { (t("Logs and featured")) }
                    (list(items, false))
                } @else {
                    p.texto { (t("Open it to download the logs or change the featured wallpapers.")) }
                }
            }
        }
    }
}

pub fn render(facts: &Facts, mail: &Found, extras: &Extras) -> (Markup, Markup) {
    let found = lines(facts, mail, &extras.asked, extras.failed.is_some());
    let writes = match &mail.mail {
        Mail::NotSet => None,
        _ => mail.setup.as_ref().map(|s| s.to.clone()),
    };
    let line = headline(facts, &found, extras.failed.is_some());
    let items = found.into_iter().map(|l| l.row).collect();
    let body = html! {
        @if let Some(raw) = &extras.failed { (failure(facts, raw)) }
        @if let Some(to) = &extras.tested {
            (notice(Tone::Good, &tf("The test mail went out to {}.", &[to]), html! { p.texto { (t("If it does not arrive in a few minutes, look in the spam folder of that address.")) } }, None))
        }
        @if facts.packages == 0 && extras.failed.is_none() {
            (empty(t("Everything is ready for the first wallpaper."), html! {
                (t("Upload it from the desktop app (Account → Add server) or from your Account page. For someone else to install what you publish, give them an invitation from People and the code for phones on this page."))
            }))
            div.acciones { (link_button(Kind::Primary, "/account", t("Upload a wallpaper"))) (link_button(Kind::Plain, "/admin/personas", t("Go to People"))) }
        }
        section.seccion aria-labelledby="salud" {
            h2 #salud { (t("Health")) }
            (list(items, false))
            @if let Some(to) = &writes { p.intro { (tf("If a row turns red or has little time left, we write to {}.", &[to])) } }
        }
        @if extras.repo_url.is_some() { (phones(extras)) }
        (advanced(extras))
    };
    (line, body)
}
