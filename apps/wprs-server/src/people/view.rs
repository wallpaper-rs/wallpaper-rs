use crate::admin::choose;
use crate::auth::ROLES;
use crate::lang::{count, since, t, tf, word};
use crate::settings::Access;
use crate::ui::{
    Choice, Kind, Row, Tone, button, button_named, choices, confirm, copyable, empty, field, fold,
    link_button, qr, rows, status,
};
use crate::units::{date, gigabytes, size};
use maud::{Markup, html};

pub struct Invite {
    pub id: i64,
    pub role: String,
    pub uses: i64,
    pub expires: i64,
}

pub struct Account {
    pub id: i64,
    pub handle: String,
    pub role: String,
    pub status: String,
    pub quota: u64,
    pub used: u64,
    pub created: i64,
}

pub struct Created {
    pub id: i64,
    pub link: String,
    pub svg: String,
    pub role: String,
    pub uses: i64,
    pub expires: i64,
}

pub struct Draft {
    pub role: String,
    pub uses: String,
    pub days: String,
    pub error: Option<(&'static str, String, String)>,
}

impl Default for Draft {
    fn default() -> Self {
        Draft { role: "user".into(), uses: "1".into(), days: "7".into(), error: None }
    }
}

pub struct People {
    pub me: i64,
    pub invites: Vec<Invite>,
    pub accounts: Vec<Account>,
    pub access: Access,
    pub created: Option<Created>,
    pub draft: Draft,
    pub revoke: Option<i64>,
    pub now: i64,
}

pub fn capital(text: &str) -> String {
    let mut chars = text.chars();
    chars.next().map(|c| c.to_uppercase().chain(chars).collect()).unwrap_or_default()
}

fn role(name: &str) -> String {
    capital(word(name))
}

pub fn headline(p: &People) -> Markup {
    let line = tf(
        "{} · {}",
        &[
            &count(p.accounts.len() as i64, "{} account", "{} accounts"),
            &count(p.invites.len() as i64, "{} live invitation", "{} live invitations"),
        ],
    );
    html! {
        p.linea-estado {
            @if p.created.is_some() { (status(Tone::Good, t("Invitation created"))) span.tenue { (line) } }
            @else if p.accounts.len() == 1 && p.invites.is_empty() { span.tenue { (t("Only you have an account · no live invitation")) } }
            @else { span.tenue { (line) } }
        }
    }
}

fn access_row(access: Access) -> Row {
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
    let now = options.iter().find(|c| c.value == access.value()).map_or(&options[1], |c| c);
    let form = html! {
        form.formulario method="post" action="/admin/personas/access" {
            (choices(t("Who gets in"), "access", &options, access.value()))
            div { (button(Kind::Plain, t("Save"))) }
        }
    };
    Row::new(html! { (now.title) }).value(html! { (now.text) }).more(fold(t("Change"), form, false))
}

fn invite_rows(p: &People) -> Markup {
    let items = p.invites.iter().map(|i| {
        let new = p.created.as_ref().is_some_and(|c| c.id == i.id);
        let name = html! { (role(&i.role)) @if new { " " span.tenue { "· " (t("the new one")) } } };
        let left = count(i.uses, "{} use left", "{} uses left");
        let row = Row::new(name).value(html! { (tf("{} · expires on {}", &[&left, &date(i.expires)])) });
        if p.revoke == Some(i.id) {
            let ask = confirm(
                t("Revoke this invitation? Whoever has the link can no longer create their account. The accounts that already got in with it stay."),
                t("Confirm"),
                &format!("/admin/personas/invites/{}/revoke", i.id),
                html! { (button_named(Kind::Danger, "confirm", "true", t("Revoke"))) (link_button(Kind::Plain, "/admin/personas", t("Cancel"))) },
            );
            row.lit().more(ask)
        } else {
            row.action(html! { a.boton href={ "/admin/personas?revoke=" (i.id) } { (t("Revoke")) } })
        }
    });
    rows(items.collect(), false)
}

fn create_form(p: &People) -> Markup {
    let d = &p.draft;
    let bad = |name: &str| d.error.as_ref().filter(|(f, ..)| *f == name).map(|(_, m, _)| m.as_str());
    let number = |name: &'static str, min: u32, max: u32, value: &str| {
        html! { input name=(name) type="number" min=(min) max=(max) value=(value) aria-invalid=[bad(name).map(|_| "true")]; }
    };
    let main = if p.created.is_some() || p.revoke.is_some() { Kind::Plain } else { Kind::Primary };
    html! {
        h3.subtitulo { (t("Create an invitation")) }
        form.formulario method="post" action="/admin/personas/invites" {
            div.campos-linea {
                (field(t("Role"), choose("role", t("Role"), &ROLES, &d.role), None, bad("role")))
                (field(t("How many people"), number("uses", 1, 1000, &d.uses), None, bad("uses")))
                (field(t("Days it works"), number("days", 1, 90, &d.days), None, bad("days")))
            }
            @if let Some((_, _, raw)) = &d.error {
                (fold(t("Details"), html! { pre.dato { (raw) } }, false))
            }
            div {
                (button(main, t("Create invitation")))
                p.ayuda { (t("The link and its code are shown only once, when it is created.")) }
            }
        }
    }
}

fn created(c: &Created) -> Markup {
    let who = if c.uses == 1 {
        tf(
            "For one person, with the role \"{}\"; it works until {}. It is shown only this time: if you lose it, revoke it and create another.",
            &[&role(&c.role), &date(c.expires)],
        )
    } else {
        tf(
            "For {} people, with the role \"{}\"; it works until {}. It is shown only this time: if you lose it, revoke it and create another.",
            &[&c.uses, &role(&c.role), &date(c.expires)],
        )
    };
    html! {
        section.seccion aria-labelledby="nueva" {
            h2 #nueva { (t("Your new invitation")) }
            p.intro { (who) }
            (qr(&c.svg, t("QR code with the invitation link"), html! {
                b { (t("Have them read it with their phone camera, or send them the link.")) }
                span.texto { (t("It opens the page to create their account, with the invitation already filled in.")) }
            }))
            (copyable(&c.link, t("Copy the link"), t("Copied")))
        }
    }
}

fn account_rows(p: &People) -> Markup {
    let items = p.accounts.iter().map(|a| {
        let me = a.id == p.me;
        let name = html! { (a.handle) @if me { " " span.tenue { "(" (t("you")) ")" } } };
        let (used, quota) = (size(a.used), size(a.quota));
        let full = a.quota > 0 && a.used >= a.quota;
        let since_when = since((p.now - a.created) / 1000);
        let rest = if a.status == "suspended" {
            tf("{} · suspended · uses {} of {}", &[&role(&a.role), &used, &quota])
        } else {
            tf("{} · active since {} · uses {} of {}", &[&role(&a.role), &since_when, &used, &quota])
        };
        let value = if full {
            html! { (status(Tone::Warn, &tf("Uses all its space: {} of {}", &[&used, &quota]))) span { (tf("{} · active since {}", &[&role(&a.role), &since_when])) } }
        } else if me {
            html! { (tf("{} · active · uses {} of {}", &[&role(&a.role), &used, &quota])) }
        } else {
            html! { (rest) }
        };
        let space = html! {
            div.campo { label for={ "q" (a.id) } { (if me { t("Space for wallpapers, in GB") } else { t("Space, in GB") }) }
                input id={ "q" (a.id) } name="quota_gb" type="number" min="0" step="0.5" value=(gigabytes(a.quota)); }
        };
        let who = html! {
            @if !me {
                div.campo { label for={ "r" (a.id) } { (t("Role")) } (choose_id(&format!("r{}", a.id), "role", &ROLES, &a.role)) }
                div.campo { label for={ "s" (a.id) } { (t("Status")) } (choose_id(&format!("s{}", a.id), "status", &["active", "suspended"], &a.status)) }
            }
        };
        let form = html! {
            form.campos-linea method="post" action={ "/admin/personas/accounts/" (a.id) } {
                (who) (space)
                div.campo { (button(Kind::Plain, t("Save"))) }
            }
        };
        let label = if me { t("Change your space") } else { t("Change") };
        Row::new(name).value(value).more(fold(label, form, false))
    });
    rows(items.collect(), false)
}

fn choose_id(id: &str, name: &str, options: &[&str], chosen: &str) -> Markup {
    html! { select id=(id) name=(name) { @for o in options { option value=(o) selected[*o == chosen] { (capital(word(o))) } } } }
}

pub fn body(p: &People) -> Markup {
    html! {
        @if p.accounts.len() == 1 && p.invites.is_empty() && p.created.is_none() {
            (empty(t("For now it is only you."), html! { (t("Create an invitation and give it to whoever you want to get in: it reaches them as a link or a QR code. Their account appears here when they use it.")) }))
        }
        @if let Some(c) = &p.created { (created(c)) }
        section.seccion aria-labelledby="entra" { h2 #entra { (t("Who gets in")) } (rows(vec![access_row(p.access)], false)) }
        section.seccion aria-labelledby="inv" {
            h2 #inv { (t("Invitations")) }
            @if p.invites.is_empty() { p.texto { (t("None alive.")) } } @else { (invite_rows(p)) }
            (create_form(p))
        }
        section.seccion #accounts aria-labelledby="cue" {
            h2 #cue { (t("Accounts")) }
            p.intro { (t("The role of each account is changed only here. Suspending an account closes its sessions.")) }
            (account_rows(p))
        }
    }
}
