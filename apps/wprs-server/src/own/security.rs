use crate::App;
use crate::icons::icon;
use crate::lang::t;
use crate::ui::{Kind, button, checkbox, field, fold, form_actions, subheading};
use maud::{Markup, html};

fn password() -> Markup {
    html! {
        form.formulario method="post" action="/api/v1/me/password?back=/account" {
            (subheading(t("Change the password")))
            (field(t("Current password"), html! { input type="password" name="current" autocomplete="current-password"; }, None, None))
            (field(t("New password"), html! { input type="password" name="new" autocomplete="new-password" minlength="10" required; },
                Some(t("10 characters or more.")), None))
            (checkbox(html! { input type="checkbox" name="devices" value="true"; }, t("Also sign out every device"), None))
            p.texto { (t("A new password signs out every other session and deletes your access tokens. Your \
                devices stay linked unless you also sign them out.")) }
            p.texto { (t("Signed-out devices have to be linked again and to get the sync key from a device \
                that still holds it; without one, or a recovery passphrase, the encrypted sync data \
                has to be reset.")) }
            (form_actions(button(Kind::Plain, t("Change password"))))
        }
    }
}

pub fn security(app: &App) -> Markup {
    html! {
        section.seccion aria-labelledby="clave" {
            h2 #clave { (t("Password and data")) }
            (password())
            @if app.cfg.oidc.is_some() {
                (fold(t("Single sign-on"), html! {
                    p.texto { (t("If your home uses one account to sign in to several services, link it and sign in here with it, without this password.")) }
                    div.acciones { a.boton href="/api/v1/auth/oidc/start?link=true" { (t("Link single sign-on")) } }
                }, false))
            }
            div.acciones {
                a.boton href="/api/v1/me/export" { (icon("download-simple")) (t("Download all my data")) }
                span.porque { (t("A file with your account, your wallpapers and your devices.")) }
            }
            (fold(t("Delete my account"), html! {
                form.formulario method="post" action="/api/v1/me?_method=DELETE&back=/" {
                    p.texto { (t("Deletes the account and its data.")) }
                    (field(t("Password"), html! { input type="password" name="password" autocomplete="current-password"; }, None, None))
                    (form_actions(button(Kind::Danger, t("Delete my account"))))
                }
            }, false))
        }
    }
}
