use crate::settings::Network;
use crate::{App, Config, lan};
use anyhow::Result;
use wprs_qr::{Code, Link};

pub fn connect_svg(server: &str, fingerprint: Option<&str>, invite: Option<&str>) -> Result<String> {
    let link = Link::new(server, fingerprint, invite)?;
    Ok(Code::new(&link.text())?.svg())
}

pub fn own_connect_svg(app: &App, invite: Option<&str>) -> Result<String> {
    own_with(&app.cfg, app.settings().network == Network::Home, invite)
}

pub fn own_fingerprint(app: &App) -> Result<Option<String>> {
    pin_with(&app.cfg, app.settings().network == Network::Home)
}

fn pin_with(cfg: &Config, self_signed: bool) -> Result<Option<String>> {
    match self_signed {
        true => {
            let (cert, _) = lan::certificate(&cfg.data, &lan::host(&cfg.public_url))?;
            Ok(Some(lan::fingerprint(&cert)))
        }
        false => Ok(None),
    }
}

fn own_with(cfg: &Config, self_signed: bool, invite: Option<&str>) -> Result<String> {
    connect_svg(&cfg.public_url, pin_with(cfg, self_signed)?.as_deref(), invite)
}

#[cfg(test)]
mod tests {
    use super::{connect_svg, own_with};
    use crate::Config;
    use wprs_qr::{Code, Link};

    const FP: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    #[test]
    fn the_inline_code_is_one_colour_and_refuses_bad_parts() {
        let svg =
            connect_svg("https://wall.example", Some(FP), Some("0123456789abcdef0123456789abcdef")).unwrap();
        assert!(svg.starts_with("<svg ") && svg.contains("currentColor"));
        assert!(connect_svg("http://wall.example", Some(FP), None).is_err());
        assert!(connect_svg("https://wall.example", Some("00"), None).is_err());
        assert!(connect_svg("https://wall.example", None, None).is_ok());
    }

    #[test]
    fn a_self_signed_server_always_carries_its_fingerprint() {
        let dir = tempfile::tempdir().unwrap();
        let cfg = Config {
            data: dir.path().to_owned(),
            public_url: "https://wall.example:8443".into(),
            ..Config::from_vars(|_| None).unwrap()
        };
        let own = own_with(&cfg, true, None).unwrap();
        assert_eq!(own, connect_svg(&cfg.public_url, Some(&pin(&cfg)), None).unwrap());
        assert_ne!(own, own_with(&cfg, false, None).unwrap());
        let plain = Link::new(&cfg.public_url, None, None).unwrap().text();
        assert_eq!(own_with(&cfg, false, None).unwrap(), Code::new(&plain).unwrap().svg());
    }

    fn pin(cfg: &Config) -> String {
        let (cert, _) = crate::lan::certificate(&cfg.data, &crate::lan::host(&cfg.public_url)).unwrap();
        crate::lan::fingerprint(&cert)
    }
}
