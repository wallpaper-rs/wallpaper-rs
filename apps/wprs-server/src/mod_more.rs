use crate::lang::{t, tf};
use crate::mod_page::ago;
use crate::ui::{Kind, Row, button, rows, subheading};
use crate::web::text;
use maud::{Markup, html};
use serde_json::Value;

pub fn withdrawn(items: &[Value]) -> Markup {
    let row = |h: &Value| {
        let name = html! { a href={ "/packages/" (text(h, "id")) } { (h["title"].as_str().unwrap_or(text(h, "id"))) } };
        let line = tf(
            "by {} · withdrawn {} · {}",
            &[&text(h, "owner"), &ago(h["hidden_at"].as_i64().unwrap_or(0)), &text(h, "reason")],
        );
        let back = html! { form method="post" action={ "/api/v1/mod/hidden/" (text(h, "id")) "?_method=DELETE&back=/mod" } {
            (button(Kind::Plain, t("Return to the catalog")))
        } };
        let row = Row::new(name).value(html! { (line) }).action(back);
        match h["sha256"].as_str() {
            Some(sha) => row.thumb(format!("/media/{sha}.jpg")),
            None => row,
        }
    };
    html! { section.seccion {
        h2 { (t("Withdrawn")) }
        p.intro { (t("A withdrawn wallpaper leaves the catalog and search; whoever installed it keeps it with its updates.")) }
        @if items.is_empty() { p.texto { (t("None.")) } } @else { (rows(items.iter().map(row).collect(), false)) }
    } }
}

pub fn more() -> Markup {
    html! { details.mas.grande.seccion {
        summary { (t("More")) " " span.ayuda { (t("withdraw by identifier, blocked files")) } }
        div.mas-cuerpo {
            (subheading(t("Withdraw a wallpaper by its identifier")))
            p.texto { (t("For a wallpaper that did not arrive as a case.")) }
            form.formulario method="post" action="/api/v1/mod/hidden?back=/mod" {
                div.campo { label for="hide-id" { (t("Wallpaper identifier")) } input id="hide-id" name="id" required placeholder="org.example.dawn"; }
                div.campo { label for="hide-why" { (t("Reason sent to the author")) }
                    input id="hide-why" name="reason" required minlength="10";
                    span.ayuda { (t("10 characters or more.")) } }
                div { (button(Kind::Plain, t("Withdraw"))) }
            }
            (subheading(t("Block a file")))
            p.texto { (t("Stops that same file from being uploaded again, even if it is renamed.")) }
            form.formulario method="post" action="/api/v1/mod/blocklist?back=/mod" {
                div.campo { label for="sha" { (t("SHA-256 of the .wpak file")) } input id="sha" name="sha256" required pattern="[0-9a-f]{64}"; }
                div.campo { label for="block-why" { (t("Reason")) } input id="block-why" name="reason" required; }
                div { (button(Kind::Plain, t("Block"))) }
            }
        }
    } }
}
