use crate::{Config, lan};
use anyhow::Result;
use wprs_qr::{Code, Link};

pub fn connect_svg(server: &str, fingerprint: &str, invite: Option<&str>) -> Result<String> {
    let link = Link::new(server, fingerprint, invite)?;
    Ok(Code::new(&link.text())?.svg())
}

pub fn own_connect_svg(cfg: &Config, invite: Option<&str>) -> Result<Option<String>> {
    if !lan::wanted()? {
        return Ok(None);
    }
    let (cert, _) = lan::certificate(&cfg.data, &lan::host(&cfg.public_url))?;
    connect_svg(&cfg.public_url, &lan::fingerprint(&cert), invite).map(Some)
}

#[cfg(test)]
mod tests {
    use super::connect_svg;

    const FP: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    #[test]
    fn the_inline_code_is_one_colour_and_refuses_bad_parts() {
        let svg = connect_svg("https://wall.example", FP, Some("0123456789abcdef0123456789abcdef")).unwrap();
        assert!(svg.starts_with("<svg ") && svg.contains("currentColor"));
        assert!(connect_svg("http://wall.example", FP, None).is_err());
        assert!(connect_svg("https://wall.example", "00", None).is_err());
    }
}
