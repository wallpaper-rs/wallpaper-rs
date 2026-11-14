use crate::App;
use crate::lang::t;
use crate::qr;
use crate::ui::{Kind, copyable_with, pairs};
use maud::{Markup, PreEscaped, html};

pub fn address(app: &App) -> String {
    format!("{}/repo", app.cfg.public_url.trim_end_matches('/'))
}

pub fn phone(app: &App, lit: bool) -> Markup {
    let pin = qr::own_fingerprint(app).ok().flatten();
    let code = qr::own_connect_svg(app, None).ok();
    let kind = if lit { Kind::Primary } else { Kind::Plain };
    html! {
        section.seccion aria-labelledby="telefono" {
            h2 #telefono { (t("On your phone")) }
            @if let Some(svg) = code {
                div.qr {
                    div role="img" aria-label=(if pin.is_some() {
                        t("QR code with this server's address and fingerprint")
                    } else { t("QR code with this server's address") }) { (PreEscaped(svg)) }
                    div.qr-texto {
                        b { (t("In the phone app, choose \"Scan the code\" and point it here.")) }
                        span.texto { (if pin.is_some() {
                            t("The code carries this server's address and fingerprint: your phone recognises it \
                               without typing anything. Then sign in with your handle and password.")
                        } else {
                            t("The code carries this server's address: your phone recognises it without typing \
                               anything. Then sign in with your handle and password.")
                        }) }
                    }
                }
            } @else {
                p.texto { (t("A code needs a secure address (https), and this server has none. Type the address below in the app.")) }
            }
            (copyable_with(&address(app), t("Copy the address"), t("Copied"), kind))
            @if let Some(pin) = pin { (pairs(&[(t("Fingerprint"), html! { code.dato { (pin) } })])) }
            p.ayuda { (t("For the desktop app: Account → Add server, and paste the address.")) }
        }
    }
}
