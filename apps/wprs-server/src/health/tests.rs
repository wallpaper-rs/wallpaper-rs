use super::facts::{Cert, Disk, Sign};
use super::mail::{Found, Mail};
use super::mail_row::{self, Asked};
use super::rows::{self, Line};
use crate::lang::{Lang, within};
use crate::mail::{Security, Setup, Why};
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
        confirm: None,
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
    assert!(shown(&good).contains("expires in"));
    assert!(!shown(&rows::certificate(&Cert { days: 755_734, own: false })).contains("expires in"));
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

fn found(mail: Mail, set: bool) -> Found {
    let setup = set.then(|| Setup {
        host: "smtp.example.org".into(),
        port: 587,
        security: Security::StartTls,
        user: "ana".into(),
        from: "Server <server@example.org>".into(),
        to: "ana@example.org".into(),
        lang: Lang::En,
    });
    Found { mail, setup, secret: set }
}

#[test]
fn the_mail_row_is_off_ready_working_and_failed_in_both_languages() {
    let asked = Asked::default();
    let off = mail_row::mail(&found(Mail::NotSet, false), &asked);
    assert_eq!(tone(&off), None);
    let html = shown(&off);
    assert!(
        html.contains("Not set up") && html.contains("Set up the mail") && html.contains("name=\"host\"")
    );
    assert!(!html.contains("Send a test mail") && !html.contains("Turn the mail off"));
    let ready = shown(&mail_row::mail(&found(Mail::Ready("ana@example.org".into()), true), &asked));
    assert!(ready.contains("Ready to write to ana@example.org") && ready.contains("Send a test mail"));
    assert!(ready.contains("leave it empty to keep it") && !ready.contains("value=\"secret"));
    let sent = crate::db::now() - 3_600_000;
    let works = mail_row::mail(&found(Mail::Works { to: "ana@example.org".into(), sent }, true), &asked);
    assert_eq!(tone(&works), Some(Tone::Good));
    assert!(shown(&works).contains("Reaches ana@example.org") && shown(&works).contains("Mail settings"));
    let when = crate::db::now() - 3 * 86_400_000;
    let raw = "smtp smtp.example.org:587: permanent error (535): 5.7.8 authentication failed".to_string();
    let failed = found(Mail::Failed { when, why: Why::Password, raw }, true);
    let line = mail_row::mail(&failed, &asked);
    assert_eq!(tone(&line), Some(Tone::Bad));
    let html = shown(&line);
    assert!(html.contains("The last notice did not go out") && html.contains("refused the password"));
    assert!(html.contains("5.7.8 authentication failed") && html.contains("How to fix it"));
    let es = within(Lang::Es, || shown(&mail_row::mail(&failed, &asked)));
    assert!(
        es.contains("El último aviso no salió")
            && es.contains("hace 3 días")
            && es.contains("Cómo arreglarlo")
    );
    assert!(es.contains("Enviar un correo de prueba") && es.contains("Ajustes del correo"));
    let off = within(Lang::Es, || shown(&mail_row::mail(&found(Mail::NotSet, false), &asked)));
    assert!(
        off.contains("Configurar el correo")
            && off.contains("Servidor de correo")
            && off.contains("Los avisos llegan a")
    );
}

#[test]
fn cases_and_invitations_link_to_their_pages() {
    assert_eq!(tone(&rows::cases(0, 0, None)), None);
    let some = rows::cases(4, 1, Some(crate::db::now() - 3 * 3_600_000));
    assert_eq!(tone(&some), Some(Tone::Warn));
    let html = shown(&some);
    assert!(html.contains("4, one is a report") && html.contains("href=\"/mod\""));
    assert!(html.contains("the report arrived 3 h ago"));
    let es = within(Lang::Es, || shown(&rows::cases(4, 1, Some(crate::db::now() - 3 * 3_600_000))));
    assert!(es.contains("la denuncia llegó hace 3 h"));
    let live = shown(&rows::invites(2, Some(1_800_000_000_000)));
    assert!(live.contains("2, the first expires on") && live.contains("/admin/personas"));
    assert!(shown(&rows::invites(0, None)).contains("None live"));
}
