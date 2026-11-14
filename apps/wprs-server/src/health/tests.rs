use super::facts::{Cert, Disk, Sign};
use super::mail::Mail;
use super::rows::{self, Line};
use crate::lang::{Lang, within};
use crate::ui::{Row, Tone};

const GB: u64 = 1 << 30;

fn shown(line: &Line) -> String {
    let row = &line.row;
    let one = Row {
        name: row.name.clone(),
        value: row.value.clone(),
        action: row.action.clone(),
        lit: false,
        thumb: None,
        more: row.more.clone(),
    };
    crate::ui::rows(vec![one], false).into_string()
}

fn tone(line: &Line) -> Option<Tone> {
    line.tone
}

#[test]
fn the_certificate_row_is_good_near_and_bad() {
    let good = rows::certificate(&Cert { days: 89, own: false });
    assert_eq!(tone(&good), Some(Tone::Good));
    assert!(!shown(&good).contains("Renew now"));
    let near = rows::certificate(&Cert { days: 9, own: false });
    assert_eq!(tone(&near), Some(Tone::Warn));
    let html = shown(&near);
    assert!(
        html.contains("Expires in 9 days")
            && html.contains("Renew now")
            && html.contains("What happens to the phones")
    );
    let bad = rows::certificate(&Cert { days: -3, own: false });
    assert_eq!(tone(&bad), Some(Tone::Bad));
    assert!(shown(&bad).contains("expired 3 days ago"));
    let own = shown(&rows::certificate(&Cert { days: 5, own: true }));
    assert!(!own.contains("Renew now") && own.contains("your own certificate"));
    let es = within(Lang::Es, || shown(&rows::certificate(&Cert { days: 9, own: false })));
    assert!(es.contains("Vence en 9 días") && es.contains("Renovar ahora"));
}

#[test]
fn the_signature_row_is_good_due_overdue_and_failed() {
    let at = |next_in, valid_for| Sign { next_in, valid_for };
    assert_eq!(tone(&rows::signature(&at(5 * 86400, 9 * 86400), false)), Some(Tone::Good));
    assert_eq!(tone(&rows::signature(&at(-60, 9 * 86400), false)), Some(Tone::Warn));
    let late = rows::signature(&at(-4 * 86400, -3 * 86400), false);
    assert_eq!(tone(&late), Some(Tone::Bad));
    let html = shown(&late);
    assert!(
        html.contains("Expired 3 days ago") && html.contains("boton principal") && html.contains("Sign now")
    );
    let failed = shown(&rows::signature(&at(5 * 86400, 5 * 86400), true));
    assert!(failed.contains("It could not be signed") && failed.contains("Sign again"));
    let es = within(Lang::Es, || shown(&rows::signature(&at(5 * 86400, 9 * 86400), false)));
    assert!(es.contains("Al día") && es.contains("próxima en 5 días") && es.contains("Firmar ahora"));
}

#[test]
fn the_backup_row_is_good_old_and_missing() {
    let good = rows::backup(Some(86400));
    assert_eq!(tone(&good), Some(Tone::Good));
    assert!(!shown(&good).contains("arreglo\" open"));
    let old = rows::backup(Some(12 * 86400));
    assert_eq!(tone(&old), Some(Tone::Warn));
    assert!(shown(&old).contains("the daily task stopped running"));
    let none = rows::backup(None);
    assert_eq!(tone(&none), Some(Tone::Warn));
    let es = within(Lang::Es, || shown(&rows::backup(None)));
    assert!(
        es.contains("Nunca")
            && es.contains("Cómo programarla")
            && es.contains("wprs-server backup")
            && es.contains("<details class=\"mas arreglo\" open")
    );
}

#[test]
fn the_disk_row_warns_on_a_nearly_full_disk_and_on_a_full_account() {
    let disk = |free, full| Disk { free, total: 100 * GB, full };
    assert_eq!(tone(&rows::disk(&disk(61 * GB, None))), Some(Tone::Good));
    assert_eq!(tone(&rows::disk(&disk(9 * GB, None))), Some(Tone::Warn));
    assert_eq!(tone(&rows::disk(&disk(2 * GB, None))), Some(Tone::Bad));
    let full = rows::disk(&disk(61 * GB, Some(("mar".into(), 2 * GB, 2 * GB))));
    assert_eq!(tone(&full), Some(Tone::Warn));
    assert!(shown(&full).contains("mar uses 2\u{a0}GB of its 2\u{a0}GB"));
    let es = within(Lang::Es, || shown(&rows::disk(&disk(9 * GB, Some(("mar".into(), 2 * GB, 2 * GB))))));
    assert!(
        es.contains("Usado el 91 %")
            && es.contains("mar usa 2\u{a0}GB de sus 2\u{a0}GB")
            && es.contains("Espacio por cuenta")
    );
}

#[test]
fn the_mail_row_waits_for_the_mail_package_and_shows_what_it_will_report() {
    let none = rows::mail(&Mail::NotSet);
    assert_eq!(tone(&none), None);
    assert!(shown(&none).contains("How to fix it"));
    assert_eq!(tone(&rows::mail(&Mail::Works("ana@casa.example".into()))), Some(Tone::Good));
    let failed = rows::mail(&Mail::Failed {
        when: 3 * 86400,
        why: "the mail server refused the password".into(),
        raw: "535 5.7.8".into(),
    });
    assert_eq!(tone(&failed), Some(Tone::Bad));
    assert!(shown(&failed).contains("535 5.7.8"));
}

#[test]
fn cases_and_invitations_link_to_their_pages() {
    assert_eq!(tone(&rows::cases(0, 0)), None);
    let some = rows::cases(4, 1);
    assert_eq!(tone(&some), Some(Tone::Warn));
    let html = shown(&some);
    assert!(html.contains("4, one is a report") && html.contains("href=\"/mod\""));
    let live = shown(&rows::invites(2, Some(1_800_000_000_000)));
    assert!(live.contains("2, the first expires on") && live.contains("/admin/personas"));
    assert!(shown(&rows::invites(0, None)).contains("None live"));
}
