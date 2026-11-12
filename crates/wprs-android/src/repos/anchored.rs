use crate::{Error, said};
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
#[serde(tag = "op", rename_all = "snake_case")]
pub(crate) enum Request {
    AddAnchored { url: String, fingerprint: Option<String> },
}

pub(crate) fn request(request: Request) -> Result<Value, Error> {
    let Request::AddAnchored { url, fingerprint } = request;
    super::add(&anchored(&url, fingerprint.as_deref())?)
}

fn hex64(text: &str) -> bool {
    text.len() == 64 && text.chars().all(|c| c.is_ascii_hexdigit())
}

fn anchored(url: &str, fingerprint: Option<&str>) -> Result<String, Error> {
    let url = url.trim();
    let typed: String =
        fingerprint.unwrap_or_default().chars().filter(|c| !c.is_whitespace() && *c != ':').collect();
    if !typed.is_empty() && !hex64(&typed.to_ascii_lowercase()) {
        return Err(said(
            "android.repo_fingerprint",
            "the fingerprint has 64 letters and digits (0 to 9 and a to f): copy it again as the publisher gave it",
            &[],
        ));
    }
    let pasted = url.contains("fingerprint=") || url.contains("repo_id=");
    match (typed.is_empty(), pasted) {
        (true, true) => Ok(url.to_owned()),
        (true, false) => Err(said(
            "android.repo_needs_fingerprint",
            "ask the publisher for the fingerprint of the repository: it is how this phone knows nobody swapped it",
            &[],
        )),
        (false, true) => Err(said(
            "android.repo_two_fingerprints",
            "the address already holds a fingerprint: leave the fingerprint box empty",
            &[],
        )),
        (false, false) => {
            let glue = if url.contains('?') { "&" } else { "?" };
            Ok(format!("{url}{glue}fingerprint={}", typed.to_ascii_lowercase()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::anchored;

    fn id(url: &str, fingerprint: Option<&str>) -> String {
        anchored(url, fingerprint).unwrap_err().reply()["id"].as_str().unwrap().to_owned()
    }

    #[test]
    fn the_fingerprint_is_asked_for_and_joined_to_the_address() {
        let print = "AB".repeat(32);
        let spaced: String =
            print.as_bytes().chunks(4).map(|c| std::str::from_utf8(c).unwrap()).collect::<Vec<_>>().join(" ");
        let wanted = format!("https://a.example/repo/?fingerprint={}", "ab".repeat(32));
        assert_eq!(anchored(" https://a.example/repo/ ", Some(&spaced)).unwrap(), wanted);
        assert_eq!(
            anchored("https://a.example/repo/?x=1", Some(&print)).unwrap(),
            format!("https://a.example/repo/?x=1&fingerprint={}", "ab".repeat(32))
        );
        let pasted = format!("https://a.example/?fingerprint={}", "cd".repeat(32));
        assert_eq!(anchored(&pasted, None).unwrap(), pasted);
        assert_eq!(anchored(&pasted, Some("")).unwrap(), pasted);
    }

    #[test]
    fn a_missing_or_doubtful_fingerprint_is_refused_in_plain_words() {
        assert_eq!(id("https://a.example/", None), "android.repo_needs_fingerprint");
        assert_eq!(id("https://a.example/", Some("")), "android.repo_needs_fingerprint");
        assert_eq!(id("https://a.example/", Some("abc")), "android.repo_fingerprint");
        assert_eq!(id("https://a.example/", Some(&"zz".repeat(32))), "android.repo_fingerprint");
        let pasted = format!("https://a.example/?fingerprint={}", "cd".repeat(32));
        assert_eq!(id(&pasted, Some(&"ab".repeat(32))), "android.repo_two_fingerprints");
    }
}
