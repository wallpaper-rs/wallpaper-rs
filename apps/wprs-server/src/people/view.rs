use super::invites::{create_form, created, invite_rows};
use crate::auth::ROLES;
use crate::lang::{count, since, t, tf, word};
use crate::settings::Access;
use crate::ui::{Choice, Kind, Row, Tone, button, choices, empty, fold, rows, status};
use crate::units::{gigabytes, size};
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

pub fn role(name: &str) -> String {
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
