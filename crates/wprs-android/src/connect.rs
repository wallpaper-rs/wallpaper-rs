use crate::{Error, said};
use serde_json::{Value, json};
use wprs_qr::{Error as Refused, Link};

fn refusal(error: Refused) -> Error {
    match error {
        Refused::NotOurs => {
            said("android.connect_not_ours", "this is not a wallpaper-rs connection code", &[])
        }
        Refused::Version => {
            said("android.connect_version", "this code is from a newer version; update the app", &[])
        }
        Refused::TooLong => said("android.connect_malformed", "this connection code is damaged", &[]),
        Refused::Server => {
            said("android.connect_server", "the server address in this code is not valid", &[])
        }
        Refused::Fingerprint => {
            said("android.connect_fingerprint", "the certificate fingerprint in this code is not valid", &[])
        }
        Refused::Invite => said("android.connect_invite", "the invitation in this code is not valid", &[]),
        Refused::Fields => said("android.connect_malformed", "this connection code is damaged", &[]),
    }
}

pub(crate) fn parse(text: &str) -> Result<Value, Error> {
    let link = Link::parse(text.trim()).map_err(refusal)?;
    Ok(json!({"server": link.server, "fingerprint": link.fingerprint, "invite": link.invite}))
}

pub(crate) fn reply(text: &str) -> String {
    parse(text).unwrap_or_else(|e| e.reply()).to_string()
}

#[cfg(test)]
mod tests {
    use super::{parse, reply};

    const FP: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    #[test]
    fn a_good_code_gives_its_parts() {
        let text = format!("wprs://connect/1?server=https://a.example&fp={FP}&invite=abcdefgh");
        let got = parse(&format!(" {text}\n")).unwrap();
        assert_eq!(
            (got["server"].as_str(), got["fingerprint"].as_str()),
            (Some("https://a.example"), Some(FP))
        );
        assert_eq!(got["invite"], "abcdefgh");
    }

    #[test]
    fn every_refusal_names_its_message() {
        let id = |t: &str| {
            serde_json::from_str::<serde_json::Value>(&reply(t)).unwrap()["id"].as_str().unwrap().to_owned()
        };
        assert_eq!(id("https://a.example"), "android.connect_not_ours");
        assert_eq!(id("wprs://connect/9?x"), "android.connect_version");
        assert_eq!(id("wprs://connect/1?server=http://a.example&fp=1"), "android.connect_server");
        assert_eq!(id("wprs://connect/1?server=https://a.example&fp=1"), "android.connect_fingerprint");
        assert_eq!(id("wprs://connect/1?server=https://a.example"), "android.connect_malformed");
    }
}
