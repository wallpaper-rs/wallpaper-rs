use crate::admin::choose;
use crate::lang::{t, tf};
use crate::ui::{Kind, Row, button_named, checkbox, disabled_button, field, rows, subheading};
use crate::web::text;
use maud::{Markup, html};
use serde_json::Value;

pub struct Draft {
    pub statement: String,
    pub missing: Option<&'static str>,
}

fn restrict(label: &str, value: &str, kind: Kind) -> Markup {
    button_named(kind, "decision", value, label)
}

fn more_rows(case: &Value) -> Vec<Row> {
    let state = text(case, "version_state");
    let mut list = Vec::new();
    if state == "published" {
        list.push(
            Row::new(html! { (t("Withdraw")) })
                .value(html! { (t("It leaves the catalog and search; whoever installed it keeps it")) })
                .action(restrict(t("Withdraw"), "remove", Kind::Danger)),
        );
        let reasons = ["other", "malware", "license", "takedown", "broken"];
        let choice = choose("reason", t("Revocation reason"), &reasons, "other");
        list.push(
            Row::new(html! { (t("Revoke")) })
                .value(html! { (t("It is revoked in the repository, with its reason")) })
                .action(html! { (choice) (restrict(t("Revoke"), "revoke", Kind::Danger)) }),
        );
    }
    if matches!(state, "removed" | "revoked") {
        list.push(
            Row::new(html! { (t("Restore")) })
                .value(html! { (t("It goes back to how it was before the case")) })
                .action(restrict(t("Restore"), "restore", Kind::Plain)),
        );
    }
    list.push(
        Row::new(html! { (t("Dismiss the case")) })
            .value(html! { (t("Closes the case without touching the wallpaper")) })
            .action(restrict(t("Dismiss"), "dismiss", Kind::Plain)),
    );
    list
}

pub fn decision_form(case: &Value, own: bool, draft: &Draft) -> Markup {
    let (pending, who) = (text(case, "version_state") == "pending", text(case, "uploader"));
    let label = tf("Reason {} receives", &[&who]);
    let area = html! { textarea name="statement" rows="3" { (draft.statement) } };
    let help = t("Needed to reject, withdraw or revoke.");
    let error = draft.missing.map(|d| tf("Write the reason: {} receives it with the {}.", &[&who, &d]));
    let strike = checkbox(
        html! { input type="checkbox" name="strike" value="true"; },
        &tf("Give {} a strike", &[&who]),
        Some(t("With three strikes in twelve months they can no longer upload.")),
    );
    html! {
        form.formulario method="post" action={ "/mod/cases/" (case["id"]) } {
            @if pending {
                div.acciones {
                    @if own { (disabled_button(t("Publish"), t("You cannot publish your own upload."))) }
                    @else { (button_named(Kind::Primary, "decision", "approve", t("Publish"))) }
                }
                (subheading(t("If you do not publish it")))
            } @else { (subheading(t("If it has to be acted on"))) }
            (field(&label, area, Some(help), error.as_deref()))
            @if pending { div.acciones { (restrict(t("Reject"), "reject", Kind::Danger)) } }
            details.mas open[draft.missing.is_some() || !pending] {
                summary { (t("More")) }
                div.mas-cuerpo { (rows(more_rows(case), false)) (strike) }
            }
        }
    }
}
