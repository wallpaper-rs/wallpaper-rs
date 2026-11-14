use crate::lang::{t, tf};
use crate::ui::{Kind, Tone, link_button, notice};
use crate::web::text;
use maud::{Markup, html};
use serde_json::Value;

pub fn newer_link(case: &Value) -> Markup {
    let target = match case["newer_case"].as_i64() {
        Some(next) => format!("/mod/cases/{next}"),
        None => "/mod".to_string(),
    };
    link_button(Kind::Primary, &target, t("Open the new version"))
}

pub fn again(case: &Value) -> Markup {
    let who = text(case, "uploader");
    html! { p.texto { (tf("{} uploaded another version while you were looking at this one. Review the new one before deciding.", &[&who])) } }
}

pub fn superseded(case: &Value) -> Markup {
    let body = again(case);
    notice(
        Tone::Warn,
        &tf("There is a newer version: {}", &[&text(case, "newer")]),
        body,
        Some(newer_link(case)),
    )
}
