use super::facts::{Cert, Disk, Sign};
use crate::lang::{count, since, t, tf, tm};
use crate::ui::{Kind, Row, Tone, button, fix_fold, link_button, status};
use crate::units::size;
use maud::{Markup, html};

pub struct Line {
    pub row: Row,
    pub tone: Option<Tone>,
}

pub fn state(tone: Tone, text: &str, detail: Option<String>) -> Markup {
    html! { (status(tone, text)) @if let Some(d) = detail { span { (d) } } }
}

pub fn post(action: &str, button: Markup) -> Markup {
    html! { form method="post" action=(action) { (button) } }
}

fn days(n: i64) -> String {
    count(n, "{} day", "{} days")
}

fn capital(text: String) -> String {
    let mut chars = text.chars();
    chars.next().map(|c| c.to_uppercase().chain(chars).collect()).unwrap_or_default()
}

pub fn certificate(cert: &Cert) -> Line {
    let phones = t(
        "Phones and apps recognise this server by the fingerprint of its certificate. After it is renewed they stop connecting until they read the new code: it appears below, in \"For the phones\".",
    );
    let (tone, text, detail) = match cert.days {
        d if d < 0 => (Tone::Bad, tf("Certificate expired {} ago", &[&days(-d)]), None),
        d if d <= 30 => (Tone::Warn, tf("Expires in {}", &[&days(d)]), None),
        d => (Tone::Good, t("Valid").into(), (d <= 3650).then(|| tf("expires in {}", &[&days(d)]))),
    };
    let mut row = Row::new(html! { (t("Certificate")) });
    if tone == Tone::Good {
        row = row.value(state(tone, &text, detail));
    } else if cert.own {
        row = row.value(state(
            tone,
            &text,
            Some(t("it is your own certificate: replace its files and restart the server").into()),
        ));
    } else {
        let note = t("when it is renewed its fingerprint changes, and so does the code for phones");
        row = row
            .value(state(tone, &text, Some(note.into())))
            .action(post("/admin/certificate", button(Kind::Plain, t("Renew now"))))
            .more(crate::ui::fold(t("What happens to the phones"), html! { p.texto { (phones) } }, false));
    }
    Line { row, tone: Some(tone) }
}

pub fn signature(sign: &Sign, failed: bool) -> Line {
    let (tone, text, detail, kind, label) = if failed {
        let note = tf("the current signature is valid for {}", &[&days(sign.valid_for.max(0) / 86400)]);
        (Tone::Bad, t("It could not be signed").into(), note, Kind::Plain, t("Sign again"))
    } else if sign.valid_for < 0 {
        let text = tf("Expired {} ago", &[&days(-sign.valid_for / 86400)]);
        (Tone::Bad, text, t("the apps accept no news until you sign").into(), Kind::Primary, t("Sign now"))
    } else if sign.next_in < 0 {
        (
            Tone::Warn,
            t("Due").into(),
            t("it signs itself soon; you can also do it now").into(),
            Kind::Plain,
            t("Sign now"),
        )
    } else {
        let next = tf("next {}", &[&since(-sign.next_in)]);
        (Tone::Good, t("Up to date").into(), next, Kind::Plain, t("Sign now"))
    };
    let row = Row::new(html! { (t("Index signature")) })
        .value(state(tone, &text, Some(detail)))
        .action(post("/admin/sign", button(kind, label)));
    Line { row, tone: Some(tone) }
}

fn schedule(open: bool) -> Markup {
    let body = html! {
        ol { li { (t("On the computer of the server, open its folder.")) }
            li { (tm("Add a daily task to your task scheduler (cron) that runs {}.", &[html! { span.dato { "wprs-server backup" } }])) }
            li { (tm("The copy is left in {}; this row turns green the next day.", &[html! { span.dato { "./data/backups" } }])) } }
    };
    fix_fold(t("How to schedule it"), body, open)
}

pub fn backup(age: Option<i64>) -> Line {
    let (tone, text, detail) = match age {
        None => (Tone::Warn, t("Never").to_string(), t("this page only looks at the date of the last one")),
        Some(a) if a > 2 * 86400 => (Tone::Warn, capital(since(a)), t("the daily task stopped running")),
        Some(a) => (Tone::Good, capital(since(a)), t("in ./data/backups")),
    };
    let row = Row::new(html! { (t("Last backup")) })
        .value(state(tone, &text, Some(detail.into())))
        .more(schedule(tone == Tone::Warn));
    Line { row, tone: Some(tone) }
}

pub fn disk(disk: &Disk) -> Line {
    let used = (disk.total.saturating_sub(disk.free) * 100).checked_div(disk.total).unwrap_or(0);
    let tone = match (used, &disk.full) {
        (95.., _) => Tone::Bad,
        (85.., _) | (_, Some(_)) => Tone::Warn,
        _ => Tone::Good,
    };
    let mut detail = tf("{} left", &[&size(disk.free)]);
    if let Some((who, used, quota)) = &disk.full {
        detail = tf("{}; {} uses {} of its {}", &[&detail, who, &size(*used), &size(*quota)]);
    }
    let kind = if tone == Tone::Good { Kind::Link } else { Kind::Plain };
    let row = Row::new(html! { (t("Disk and quotas")) })
        .value(state(tone, &tf("{} % used", &[&used]), Some(detail)))
        .action(link_button(kind, "/admin/personas#accounts", t("Space per account")));
    Line { row, tone: Some(tone) }
}

pub fn cases(open: i64, reports: i64, last: Option<i64>) -> Line {
    let name = Row::new(html! { (t("Pending cases")) });
    if open == 0 {
        let row = name.value(html! { (t("None")) }).action(link_button(Kind::Link, "/mod", t("Review")));
        return Line { row, tone: None };
    }
    let text = match reports {
        0 => open.to_string(),
        1 => tf("{}, one is a report", &[&open]),
        r => tf("{}, {} are reports", &[&open, &r]),
    };
    let arrived = last.map(|at| {
        let ago = since((crate::db::now() - at) / 1000);
        match reports {
            1 => tf("the report arrived {}", &[&ago]),
            _ => tf("the last report arrived {}", &[&ago]),
        }
    });
    let row = name.value(state(Tone::Warn, &text, arrived)).action(link_button(
        Kind::Plain,
        "/mod",
        t("To review"),
    ));
    Line { row, tone: Some(Tone::Warn) }
}

pub fn invites(live: i64, first: Option<i64>) -> Line {
    let name = Row::new(html! { (t("Live invitations")) });
    let value = match (live, first) {
        (0, _) | (_, None) => t("None live").to_string(),
        (1, Some(at)) => tf("1, it expires on {}", &[&crate::units::date(at)]),
        (n, Some(at)) => tf("{}, the first expires on {}", &[&n, &crate::units::date(at)]),
    };
    let row =
        name.value(html! { (value) }).action(link_button(Kind::Link, "/admin/personas", t("See in People")));
    Line { row, tone: None }
}
