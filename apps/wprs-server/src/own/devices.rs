use crate::db::now;
use crate::lang::{since, t, tf};
use crate::sync::Head;
use crate::ui::{Kind, Row, button, confirm, link_button, pairs, rows, size};
use crate::web::text;
use crate::{App, icons::icon};
use maud::{Markup, html};
use serde_json::Value;

fn name(d: &Value) -> String {
    match text(d, "name").strip_prefix("Device ") {
        Some(number) => tf("Device {}", &[&number]),
        None => text(d, "name").to_string(),
    }
}

fn platform(d: &Value) -> String {
    let raw = text(d, "platform");
    let mut chars = raw.chars();
    chars.next().map(|c| c.to_uppercase().chain(chars).collect()).unwrap_or_default()
}

fn row(d: &Value, asking: bool) -> Row {
    let id = text(d, "id");
    let age = (now() - d["created_at"].as_i64().unwrap_or_default()) / 1000;
    let value = html! { (tf("{} · linked {}", &[&platform(d), &since(age)])) };
    let row = Row::new(html! { (name(d)) }).value(value);
    if asking {
        let ask = tf(
            "Remove \"{}\"? It stops syncing with your account and, to use it again, you will have to link it again.",
            &[&name(d)],
        );
        let action = format!("/api/v1/me/devices/{id}?_method=DELETE&back=/account");
        let buttons = html! {
            (button(Kind::Danger, t("Remove")))
            (link_button(Kind::Plain, "/account#dispositivos", t("Cancel")))
        };
        row.confirm(confirm(&ask, t("Confirm"), &action, buttons)).lit()
    } else {
        row.action(
            html! { (link_button(Kind::Danger, &format!("/account?remove={id}#dispositivos"), t("Remove"))) },
        )
    }
}

pub fn devices(app: &App, list: &[Value], asking: &str, sync: Option<&Head>) -> Markup {
    let items: Vec<Row> = list.iter().map(|d| row(d, text(d, "id") == asking)).collect();
    html! {
        section.seccion aria-labelledby="dispositivos" {
            h2 #dispositivos { (t("Devices")) }
            @if items.is_empty() {
                p.texto { (t("None yet. A phone that reads the code above shows up here.")) }
            } @else { (rows(items, false)) }
            @if let Some(h) = sync {
                (pairs(&[
                    (t("Synced data"), html! { (tf("{} of {}: lists, favourites and settings of your devices",
                        &[&size(h.used as u64), &size(app.cfg.sync_quota)])) }),
                    (t("Private files"), html! { (tf("{} of {}: encrypted files that your devices keep here; only they can read them",
                        &[&size(h.files as u64), &size(app.cfg.files_quota)])) }),
                ]))
            }
            div.acciones { a.boton href="/link" { (icon("link")) (t("Link a new device with its code")) } }
        }
    }
}
