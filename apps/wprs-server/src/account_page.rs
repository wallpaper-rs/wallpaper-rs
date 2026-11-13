use crate::St;
use crate::account::profile;
use crate::auth::Auth;
use crate::error::{Problem, Res};
use crate::lang::{t, tf, tm, word};
use crate::me::listing;
use crate::sync::head;
use crate::web::{page, text};
use axum::extract::State;
use axum::response::{IntoResponse, Redirect, Response};
use maud::{Markup, html};
use serde_json::Value;

fn remove(path: String) -> Markup {
    html! { form method="post" action={ (path) "?_method=DELETE&back=/account" } { button { (t("Remove")) } } }
}

const VISIBILITY: [&str; 2] = ["public", "unlisted"];

fn visibility(v: &Value) -> Markup {
    let (id, version, now) = (text(v, "package"), text(v, "version"), text(v, "visibility"));
    let other = if now == "public" { "unlisted" } else { "public" };
    html! { form method="post" action={ "/api/v1/me/packages/" (id) "/visibility?back=/account" } {
        input type="hidden" name="version" value=(version);
        input type="hidden" name="visibility" value=(other);
        (word(now)) " " button { (tf("Make {}", &[&word(other)])) }
    } }
}

pub async fn account(State(app): St, auth: Option<Auth>) -> Res<Response> {
    let Some(auth) = auth else { return Ok(Redirect::to("/login?next=/account").into_response()) };
    let user = auth.user;
    let data = app.db.run(move |c| {
        let lists = ["versions", "uploads", "devices", "tokens", "keys"].map(|w| listing(c, user, w));
        Ok((profile(c, user)?, lists.into_iter().collect::<Res<Vec<_>>>()?, head(c, user).ok()))
    });
    let (me, lists, sync) = data.await?;
    let [versions, uploads, devices, tokens, keys] =
        <[Vec<Value>; 5]>::try_from(lists).map_err(|_| Problem::internal("account lists"))?;
    let body = html! {
        p { (tm("Signed in as {} ({}). Package storage used: {} of {} bytes.", &[html! { strong { (auth.handle) } },
            html! { (word(auth.role.name())) }, html! { (me["used_bytes"]) }, html! { (me["quota_bytes"]) }])) }
        @if let Some(h) = sync {
            p { (tf("Sync data: {} of {} bytes. Private files: {} of {} bytes.",
                &[&h.used, &app.cfg.sync_quota, &h.files, &app.cfg.files_quota])) }
        }
        section.seccion { h2 { (t("Upload a wallpaper package")) }
            form method="post" action="/upload?back=/account" enctype="multipart/form-data" {
                label { (t("Visibility")) (crate::admin::choose("visibility", t("Visibility"), &VISIBILITY, "public")) }
                label { (t(".wpak file")) input type="file" name="package" accept=".wpak" required; }
                button { (t("Upload")) }
            }
            p.texto { (t("Unlisted versions stay out of the catalog and search; whoever has the link to the package \
                        can still see and install them. Unlisted is not private: they stay in the signed \
                        repository files, which anyone can read; keep private wallpapers in sync or in a \
                        repository of your own.")) }
            table {
                thead { tr { th { (t("Package")) } th { (t("Version")) } th { (t("State")) } th { (t("Visibility")) } th { (t("Moderation")) } } }
                tbody {
                    @for v in &versions { tr {
                        td { (text(v, "title")) br; code { (text(v, "package")) } }
                        td { (text(v, "version")) } td { (word(text(v, "state"))) }
                        td { (visibility(v)) } td { (text(v, "statement")) }
                    } }
                    @for u in &uploads { tr {
                        td { (tf("Upload {}", &[&text(u, "id")])) } td {}
                        td { (word(text(u, "state"))) } td {} td { (text(u, "errors")) }
                    } }
                }
            }
        }
        section.seccion { h2 { (t("Devices")) }
            p { a href="/link" { (t("Link a new device with its code")) } }
            table { tbody { @for d in &devices { tr {
                td { (text(d, "name")) } td { (text(d, "platform")) }
                td { (remove(format!("/api/v1/me/devices/{}", text(d, "id")))) }
            } } } }
        }
        section.seccion { h2 { (t("Access tokens")) }
            table { tbody { @for t in &tokens { tr {
                td { (text(t, "name")) } td { code { (text(t, "scopes")) } }
                td { @if t["device_id"].is_null() { (remove(format!("/api/v1/me/tokens/{}", t["id"]))) } }
            } } } }
            form method="post" action="/api/v1/me/tokens?back=/account" {
                label { (t("Name")) input name="name" required maxlength="64"; }
                label { (t("Scopes")) input name="scopes" value="catalog:read upload" required; }
                label { (t("Days")) input type="number" name="days" min="1" max="365" value="30"; }
                button { (t("Create token")) }
            }
        }
        section.seccion { h2 { (t("Author keys")) }
            @if app.repo.online() {
                p { (tm("Public minisign keys that sign your packages. Clients trust the key that signed the first \
                    version they installed of each package, so a new key needs a rotation signed by the old \
                    one: {}", &[html! { code { "wprs-ctl pack rotate-key OLD.key NEW.pub --author " (crate::repo::author(auth.user)) } }])) }
            } @else {
                p { (t("Public minisign keys that sign your packages. The repository root must list them too.")) }
            }
            table { tbody { @for k in &keys { tr {
                td { code { (text(k, "fingerprint")) } }
                td { (remove(format!("/api/v1/me/author-keys/{}", text(k, "fingerprint")))) }
            } } } }
            form method="post" action="/api/v1/me/author-keys?back=/account" {
                label { (t("Public key")) input name="public_key" required; } button { (t("Add key")) }
            }
            @if app.repo.online() {
                form.formulario method="post" action="/api/v1/me/author-keys/rotate?back=/account" {
                    label { (t("Old key fingerprint")) input name="from" required pattern="[0-9a-f]{64}"; }
                    label { (t("New public key")) input name="public_key" required; }
                    label { (t("Rotation signature")) textarea name="signature" rows="4" required {} }
                    button { (t("Rotate to the new key")) }
                }
            }
        }
        section.seccion { h2 { (t("Password and data")) }
            @if app.cfg.oidc.is_some() {
                p { a href="/api/v1/auth/oidc/start?link=true" { (t("Link single sign-on")) } }
            }
            p.texto { (t("A new password signs out every other session and deletes your access tokens. Your \
                devices stay linked unless you also sign them out.")) }
            form method="post" action="/api/v1/me/password?back=/account" {
                label { (t("Current password"))
                    input type="password" name="current" autocomplete="current-password"; }
                label { (t("New password"))
                    input type="password" name="new" autocomplete="new-password" minlength="10" required; }
                label { input type="checkbox" name="devices" value="true"; (t("Also sign out every device")) }
                p.texto { (t("Signed-out devices have to be linked again and to get the sync key from a device \
                    that still holds it; without one, or a recovery passphrase, the encrypted sync data \
                    has to be reset.")) }
                button { (t("Change password")) }
            }
            p { a href="/api/v1/me/export" { (t("Download all your data (JSON)")) } }
            form method="post" action="/api/v1/me?_method=DELETE&back=/" {
                label { (t("Password")) input type="password" name="password" autocomplete="current-password"; }
                button { (t("Delete my account")) }
            }
        }
    };
    Ok(page(&app, Some(&auth), "/account", t("Your account"), body).into_response())
}
