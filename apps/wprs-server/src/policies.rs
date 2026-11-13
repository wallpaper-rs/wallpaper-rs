use crate::config::Config;
use crate::lang::{t, tf, tm};
use crate::legal::or_unset;
use maud::{Markup, html};

fn link(href: &str, label: &str) -> Markup {
    html! { a href=(href) { (label) } }
}

pub fn terms(c: &Config) -> Markup {
    html! {
        p { (tf("This server is run by {}. By creating an account you accept these terms.", &[&or_unset(&c.operator)])) }
        h2 { (t("Your content")) }
        p { (t("You keep your rights. You confirm that you may share what you upload under the licence you \
            declare, and you allow the operator to store, sign and distribute it in the package \
            repository of this server.")) }
        h2 { (t("Rules")) }
        p { (tm("Uploads follow the {}. Do not upload illegal content, malware or work you have no right to share.",
            &[link("/legal/content", t("content policy"))])) }
        h2 { (t("Moderation")) }
        p { (t("Moderators may reject, remove or revoke content. Every restriction comes with a statement \
            of reasons on your account page. Three strikes within twelve months suspend uploads; \
            repeated or serious abuse can suspend the account.")) }
        h2 { (t("Notices and appeals")) }
        p { (tm("Anyone can send a {}. Authors can reply with a {} or write to {}.", &[
            link("/legal/notice", t("notice")),
            link("/legal/counter-notice", t("counter-notice")),
            html! { (or_unset(&c.contact)) },
        ])) }
        h2 { (t("No warranty")) }
        p { (t("The service is provided as is, without any warranty of availability or fitness.")) }
    }
}

pub fn privacy(c: &Config) -> Markup {
    let contact = or_unset(&c.contact);
    html! {
        p { (tf("Controller: {}, contact {}.", &[&or_unset(&c.operator), &contact])) }
        h2 { (t("What is stored")) }
        ul {
            li { (t("Your handle, a password hash (Argon2id) and your role.")) }
            li { (t("Your devices under generic names (Device 1, Device 2…) with their public keys, \
                 and hashes of your tokens, never the tokens.")) }
            li { (t("Packages you upload and moderation records about them.")) }
            li { (t("Synced library, playlists and settings. With end-to-end encryption, the default, \
                 the server only holds ciphertext it cannot read.")) }
        }
        h2 { (t("What is not")) }
        p { (tf("No tracking, analytics or third-party resources, and one strictly necessary session cookie. \
            IP addresses and user agents are not logged unless the operator turns on abuse logging, \
            which keeps them at most {} days.", &[&c.log_days])) }
        h2 { (t("How long")) }
        p { (tf("Sessions 30 days, device codes 10 minutes, device messages 7 days, deleted sync items \
            180 days, the name, email and signature of a legal notice {} days, a \
            package withdrawn by the moderators {} days, as evidence and for a \
            counter-notice. Everything else until you delete it or your account.",
            &[&c.notice_days, &c.takedown_days])) }
        h2 { (t("Your rights")) }
        p { (tm("Download or delete all your data from your {}. For anything else write to {}.",
            &[link("/account", t("account page")), html! { (contact) }])) }
    }
}

pub fn content(c: &Config) -> Markup {
    html! {
        ul {
            li { (t("Wallpapers only: images, videos and scenes in .wpak packages with a preview image.")) }
            li { (tf("Accepted licences: {}.", &[&c.licenses.join(", ")])) }
            li { (t("Mature content is accepted only when the package rates it mature in its manifest \
                (\"content\": {\"rating\": \"mature\"} with its reasons): it carries that label in the catalog, \
                in search and in the API, and clients and this site blur its previews until the viewer \
                chooses to show them. Mature content without that rating is removed.")) }
            li { (t("No illegal content, no sexual content involving minors, no malware, no impersonation.")) }
            li { (t("Upload only your own work or work whose licence allows sharing, and credit its authors.")) }
            li { (t("Content converted from other formats or products needs proof that you may share it.")) }
            li { (t("Packages that flash or show fast patterns must set \"photosensitive\": true in their \
                wallpaper-rs.wpak/2 manifest: they can trigger seizures in people with photosensitive \
                epilepsy, and clients warn before showing them.")) }
        }
    }
}
