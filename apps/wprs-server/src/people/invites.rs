use super::view::{Created, People, role};
use crate::admin::choose;
use crate::auth::ROLES;
use crate::lang::{count, t, tf};
use crate::ui::*;
use crate::units::date;
use maud::{Markup, html};

pub fn invite_rows(p: &People) -> Markup {
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

pub fn create_form(p: &People) -> Markup {
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

pub fn created(c: &Created) -> Markup {
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
