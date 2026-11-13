pub const PREFIX: &str = "wprs://connect/1?";
const MAX_LEN: usize = 256;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    #[error("not a wallpaper-rs connection code")]
    NotOurs,
    #[error("unsupported version of the connection code")]
    Version,
    #[error("the connection code is too long")]
    TooLong,
    #[error("the server address must be https://host, https://host:port or with a path, host in lowercase")]
    Server,
    #[error("the certificate fingerprint must be 64 lowercase hexadecimal digits")]
    Fingerprint,
    #[error("the invitation code must be 8 to 64 letters, digits, dashes or underscores")]
    Invite,
    #[error("the connection code has unexpected, missing or repeated fields")]
    Fields,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Link {
    pub server: String,
    pub fingerprint: Option<String>,
    pub invite: Option<String>,
}

fn label(part: &str) -> bool {
    let edge = |c: Option<char>| c.is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit());
    edge(part.chars().next())
        && edge(part.chars().last())
        && part.len() <= 63
        && part.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

fn host(host: &str) -> bool {
    match host.strip_prefix('[').and_then(|h| h.strip_suffix(']')) {
        Some(v6) => {
            v6.contains(':')
                && v6.len() <= 39
                && v6.chars().all(|c| c.is_ascii_digit() || ('a'..='f').contains(&c) || c == ':')
        }
        None => !host.is_empty() && host.len() <= 253 && host.split('.').all(label),
    }
}

fn segment(part: &str) -> bool {
    !part.is_empty()
        && part != "."
        && part != ".."
        && part.chars().all(|c| c.is_ascii_alphanumeric() || "._~-".contains(c))
}

fn authority(rest: &str) -> bool {
    let (name, port) = match rest.rsplit_once(':') {
        Some((name, port)) if !name.ends_with(':') && (!name.contains(':') || name.ends_with(']')) => {
            (name, Some(port))
        }
        _ => (rest, None),
    };
    let port_ok = port.is_none_or(|p| {
        !p.starts_with('0')
            && p.len() <= 5
            && p.chars().all(|c| c.is_ascii_digit())
            && p.parse::<u16>().is_ok()
    });
    port_ok && host(name)
}

fn server_ok(server: &str) -> bool {
    let Some(rest) = server.strip_prefix("https://") else { return false };
    let (head, path) = rest.split_once('/').map_or((rest, None), |(h, p)| (h, Some(p)));
    authority(head) && path.is_none_or(|p| p.split('/').all(segment))
}

fn fingerprint_ok(text: &str) -> bool {
    text.len() == 64 && text.chars().all(|c| c.is_ascii_digit() || ('a'..='f').contains(&c))
}

fn invite_ok(code: &str) -> bool {
    (8..=64).contains(&code.len()) && code.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

impl Link {
    pub fn new(server: &str, fingerprint: Option<&str>, invite: Option<&str>) -> Result<Link, Error> {
        let server = server.trim().trim_end_matches('/');
        let (scheme, rest) = server.split_once("://").unwrap_or(("", server));
        let (head, path) = rest.split_once('/').map_or((rest, ""), |(h, p)| (h, p));
        let slash = if path.is_empty() { "" } else { "/" };
        let server = format!("{}://{}{slash}{path}", scheme.to_ascii_lowercase(), head.to_ascii_lowercase());
        let fingerprint = fingerprint.map(|f| f.trim().to_ascii_lowercase());
        let link = Link { server, fingerprint, invite: invite.map(String::from) };
        link.check()?;
        Ok(link)
    }

    fn check(&self) -> Result<(), Error> {
        if !server_ok(&self.server) {
            return Err(Error::Server);
        }
        if self.fingerprint.as_deref().is_some_and(|f| !fingerprint_ok(f)) {
            return Err(Error::Fingerprint);
        }
        if self.invite.as_deref().is_some_and(|code| !invite_ok(code)) {
            return Err(Error::Invite);
        }
        Ok(())
    }

    pub fn text(&self) -> String {
        let invite = self.invite.as_ref().map(|code| format!("&invite={code}")).unwrap_or_default();
        let pin = self.fingerprint.as_ref().map(|f| format!("&fp={f}")).unwrap_or_default();
        format!("{PREFIX}server={}{pin}{invite}", self.server)
    }

    pub fn parse(text: &str) -> Result<Link, Error> {
        if text.len() > MAX_LEN {
            return Err(Error::TooLong);
        }
        let Some(fields) = text.strip_prefix(PREFIX) else {
            let other = text.strip_prefix("wprs://connect/").and_then(|v| v.split('?').next());
            let numbered = other.is_some_and(|v| !v.is_empty() && v.chars().all(|c| c.is_ascii_digit()));
            return Err(if numbered { Error::Version } else { Error::NotOurs });
        };
        let mut parts = fields.split('&');
        let mut field = |name: &str| parts.next().and_then(|p| p.strip_prefix(name)?.strip_prefix('='));
        let server = field("server").ok_or(Error::Fields)?;
        let mut next = parts.next();
        let fingerprint = next.and_then(|p| p.strip_prefix("fp="));
        if fingerprint.is_some() {
            next = parts.next();
        }
        let invite = match next {
            None => None,
            Some(extra) => Some(extra.strip_prefix("invite=").ok_or(Error::Fields)?),
        };
        if parts.next().is_some() {
            return Err(Error::Fields);
        }
        let link = Link {
            server: server.into(),
            fingerprint: fingerprint.map(String::from),
            invite: invite.map(String::from),
        };
        link.check()?;
        Ok(link)
    }

    pub fn invitation_url(&self) -> Option<String> {
        self.invite.as_ref().map(|code| format!("{}/register?invite={code}", self.server))
    }
}
