use crate::{Error, said};
use std::net::IpAddr;
use wprs_sync::check_url;

pub(crate) fn address(typed: &str) -> Result<String, Error> {
    let typed = typed.trim();
    let full = if typed.contains("://") { typed.to_owned() } else { format!("https://{typed}") };
    let url = reqwest::Url::parse(&full).ok().filter(|u| u.host_str().is_some_and(|h| !h.is_empty()));
    let Some(url) = url else {
        return Err(said("android.server_not_address", "this is not a server address", &[]));
    };
    let host = url.host_str().unwrap_or_default().trim_matches(['[', ']']);
    let local = host == "localhost" || host.parse::<IpAddr>().is_ok_and(|ip| ip.is_loopback());
    if url.scheme() != "https" && !(url.scheme() == "http" && local) {
        return Err(said("android.server_needs_https", "the server address must start with https://", &[]));
    }
    if !url.username().is_empty()
        || url.password().is_some()
        || url.query().is_some()
        || url.fragment().is_some()
    {
        return Err(said(
            "android.server_address_parts",
            "the server address takes only a host and a path",
            &[],
        ));
    }
    Ok(check_url(&full)?)
}

#[cfg(test)]
mod tests {
    use super::address;

    fn id(typed: &str) -> String {
        address(typed).unwrap_err().reply()["id"].as_str().unwrap().to_owned()
    }

    #[test]
    fn a_bare_host_gets_https() {
        assert_eq!(address("  mi-servidor.local ").unwrap(), "https://mi-servidor.local");
        assert_eq!(address("https://a.example/wp/").unwrap(), "https://a.example/wp");
        assert_eq!(address("http://localhost:8080").unwrap(), "http://localhost:8080");
    }

    #[test]
    fn every_refusal_has_its_own_message() {
        assert_eq!(id("hola mundo"), "android.server_not_address");
        assert_eq!(id("https://"), "android.server_not_address");
        assert_eq!(id("http://example.org"), "android.server_needs_https");
        assert_eq!(id("ftp://example.org"), "android.server_needs_https");
        assert_eq!(id("https://u:p@example.org"), "android.server_address_parts");
        assert_eq!(id("https://example.org/?a=b"), "android.server_address_parts");
    }
}
