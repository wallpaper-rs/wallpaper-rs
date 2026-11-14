use crate::App;
use crate::lang::{t, tm};
use crate::ui::{Kind, Row, button, field, fold, form_actions, rows, subheading};
use crate::web::text;
use maud::{Markup, html};
use serde_json::Value;

fn drop(path: String) -> Markup {
    html! { form method="post" action={ (path) "?_method=DELETE&back=/account" } { (button(Kind::Danger, t("Remove"))) } }
}

fn tokens(list: &[Value]) -> Markup {
    let items: Vec<Row> = list
        .iter()
        .map(|k| {
            let row =
                Row::new(html! { (text(k, "name")) }).value(html! { code.dato { (text(k, "scopes")) } });
            match k["device_id"].is_null() {
                true => row.action(drop(format!("/api/v1/me/tokens/{}", k["id"]))),
                false => row,
            }
        })
        .collect();
    html! {
        (subheading(t("Access tokens")))
        (rows(items, true))
        form.formulario method="post" action="/api/v1/me/tokens?back=/account" {
            (field(t("Name"), html! { input name="name" required maxlength="64"; }, None, None))
            div.campos-linea {
                (field(t("Scopes"), html! { input name="scopes" value="catalog:read upload" required; },
                    Some(t("Separated by spaces: catalog:read, upload, sync, mod, admin.")), None))
                (field(t("Days"), html! { input type="number" name="days" min="1" max="365" value="30"; }, None, None))
            }
            (form_actions(button(Kind::Plain, t("Create token"))))
        }
    }
}

fn keys(app: &App, author: i64, list: &[Value]) -> Markup {
    let items: Vec<Row> = list
        .iter()
        .map(|k| {
            Row::new(html! { (t("Key")) })
                .value(html! { code.dato { (text(k, "fingerprint")) } })
                .action(drop(format!("/api/v1/me/author-keys/{}", text(k, "fingerprint"))))
        })
        .collect();
    html! {
        (subheading(t("Author keys")))
        @if app.repo.online() {
            p.texto { (tm("Public minisign keys that sign your packages. Clients trust the key that signed the first \
                version they installed of each package, so a new key needs a rotation signed by the old \
                one: {}", &[html! { code { "wprs-ctl pack rotate-key OLD.key NEW.pub --author " (crate::repo::author(author)) } }])) }
        } @else {
            p.texto { (t("Public minisign keys that sign your packages. The repository root must list them too.")) }
        }
        (rows(items, true))
        form.formulario method="post" action="/api/v1/me/author-keys?back=/account" {
            (field(t("Public key"), html! { input name="public_key" required; }, None, None))
            (form_actions(button(Kind::Plain, t("Add key"))))
        }
        @if app.repo.online() {
            form.formulario method="post" action="/api/v1/me/author-keys/rotate?back=/account" {
                (field(t("Old key fingerprint"), html! { input name="from" required pattern="[0-9a-f]{64}"; }, None, None))
                (field(t("New public key"), html! { input name="public_key" required; }, None, None))
                (field(t("Rotation signature"), html! { textarea name="signature" rows="4" required {} }, None, None))
                (form_actions(button(Kind::Plain, t("Rotate to the new key"))))
            }
        }
    }
}

pub fn advanced(app: &App, author: i64, tokens_list: &[Value], keys_list: &[Value]) -> Markup {
    html! {
        div.seccion { (fold(t("Advanced"), html! { (tokens(tokens_list)) (keys(app, author, keys_list)) }, true)) }
    }
}
