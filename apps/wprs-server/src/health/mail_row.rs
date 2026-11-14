use super::mail::{Found, Mail};
use super::rows::{Line, post, state};
use crate::lang::{since, t, tf};
use crate::mail::{Security, Setup};
use crate::ui::{
    Kind, Row, Tone, button, button_named, confirm, field, fix_fold, fold, form_actions, link_button,
};
use maud::{Markup, html};
use serde::Deserialize;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Draft {
    #[serde(default)]
    pub host: String,
    #[serde(default)]
    pub port: String,
    #[serde(default)]
    pub security: String,
    #[serde(default)]
    pub user: String,
    #[serde(default)]
    pub password: String,
    #[serde(default)]
    pub from: String,
    #[serde(default)]
    pub to: String,
    #[serde(default)]
    pub action: String,
}

impl Draft {
    pub fn of(setup: Option<&Setup>) -> Draft {
        let Some(s) = setup else { return Draft { port: "587".into(), ..Draft::default() } };
        Draft {
            host: s.host.clone(),
            port: s.port.to_string(),
            security: s.security.value().into(),
            user: s.user.clone(),
            from: s.from.clone(),
            to: s.to.clone(),
            ..Draft::default()
        }
    }
}

#[derive(Default)]
pub struct Asked {
    pub draft: Option<Draft>,
    pub error: Option<(&'static str, &'static str)>,
    pub off: bool,
}

fn security_label(s: Security) -> &'static str {
    match s {
        Security::StartTls => t("STARTTLS (usually port 587)"),
        Security::Tls => t("TLS from the start (usually port 465)"),
        Security::Plain => t("None (only inside your own network)"),
    }
}

fn form(found: &Found, asked: &Asked) -> Markup {
    let draft = asked.draft.clone().unwrap_or_else(|| Draft::of(found.setup.as_ref()));
    let wrong = |name: &str| asked.error.filter(|(f, _)| *f == name).map(|(_, e)| e);
    let chosen = Security::parse(&draft.security).unwrap_or(Security::StartTls);
    let kept = found.secret.then(|| t("Saved; leave it empty to keep it."));
    html! {
        form.formulario method="post" action="/admin/mail" {
            p.texto { (t("Off until you set it up: while it is off, nothing is sent and nothing waits to be sent.")) }
            (field(t("Mail server"), html! { input name="host" value=(draft.host) placeholder="smtp.example.org" autocomplete="off" required; }, None, wrong("host")))
            (field(t("Port"), html! { input name="port" value=(draft.port) inputmode="numeric" required; }, None, wrong("port")))
            (field(t("Security"), html! {
                select name="security" {
                    @for s in Security::ALL { option value=(s.value()) selected[s == chosen] { (security_label(s)) } }
                }
            }, None, None))
            (field(t("User"), html! { input name="user" value=(draft.user) autocomplete="off"; }, Some(t("Leave it empty if your mail server asks for none.")), None))
            (field(t("Password"), html! { input type="password" name="password" autocomplete="new-password"; }, kept, None))
            (field(t("Sender"), html! { input name="from" value=(draft.from) placeholder="Server <server@example.org>" required; }, Some(t("The address the mails come from, as your mail provider allows it.")), wrong("from")))
            (field(t("Notices go to"), html! { input type="email" name="to" value=(draft.to) placeholder="you@example.org" required; }, Some(t("Who moderates and who administers get the notices here.")), wrong("to")))
            (form_actions(html! {
                (button_named(Kind::Primary, "action", "save", t("Save")))
                @if found.setup.is_some() { (link_button(Kind::Danger, "/admin?mail=off#salud", t("Turn the mail off"))) }
            }))
        }
    }
}

fn settings(found: &Found, asked: &Asked) -> Markup {
    let open = asked.draft.is_some();
    html! { details.mas open[open] { summary { (t("Mail settings")) } div.mas-cuerpo { (form(found, asked)) } } }
}

fn asking() -> Line {
    let ask = t("Turn the mail off? The server forgets the mail server, the user and the saved password, \
                 and drops the notices still waiting to go out. To send mail again you will have to set it up again.");
    let buttons = html! {
        (button_named(Kind::Danger, "action", "off", t("Turn the mail off")))
        (link_button(Kind::Plain, "/admin#salud", t("Cancel")))
    };
    let row =
        Row::new(html! { (t("Mail")) }).confirm(confirm(ask, t("Confirm"), "/admin/mail", buttons)).lit();
    Line { row, tone: None }
}

pub fn mail(found: &Found, asked: &Asked) -> Line {
    if asked.off && found.setup.is_some() {
        return asking();
    }
    let name = Row::new(html! { (t("Mail")) });
    let test = post("/admin/mail/test", button(Kind::Plain, t("Send a test mail")));
    match &found.mail {
        Mail::Works { to, sent } => {
            let detail = tf("the last one went out {}", &[&since((crate::db::now() - sent) / 1000)]);
            let row = name
                .value(state(Tone::Good, &tf("Reaches {}", &[to]), Some(detail)))
                .action(test)
                .more(settings(found, asked));
            Line { row, tone: Some(Tone::Good) }
        }
        Mail::Ready(to) => {
            let value = html! { span { (tf("Ready to write to {}", &[to])) } span { (t("nothing sent yet: send a test mail to check it")) } };
            Line { row: name.value(value).action(test).more(settings(found, asked)), tone: None }
        }
        Mail::Failed { when, why, raw } => {
            let detail = tf("{}; {}", &[&since((crate::db::now() - when) / 1000), &why.said()]);
            let steps = html! {
                ol { li { (t("Check the mail server, the user and the password your mail provider gave you, below in “Mail settings”.")) }
                     li { (t("Save, and press “Send a test mail”.")) }
                     li { (t("The next notice goes out by itself; this row turns green when it arrives.")) } }
                (settings(found, asked))
                (fold(t("Details"), html! { pre.dato { (raw) } }, false))
            };
            let row = name
                .value(state(Tone::Bad, t("The last notice did not go out"), Some(detail)))
                .action(test)
                .more(fix_fold(t("How to fix it"), steps, true));
            Line { row, tone: Some(Tone::Bad) }
        }
        Mail::NotSet => {
            let body = html! {
                p.texto { (t("The server sends no mail until you give it the mail server it should use. Then it writes when a report or a legal notice arrives, and when a row of this page turns red or has little time left.")) }
                (form(found, asked))
            };
            let row = name
                .value(html! { span { (t("Not set up: the server sends no notices by mail")) } })
                .more(fix_fold(t("Set up the mail"), body, asked.draft.is_some()));
            Line { row, tone: None }
        }
    }
}
