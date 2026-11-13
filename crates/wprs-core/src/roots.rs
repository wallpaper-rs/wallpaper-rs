use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

static ADDED: OnceLock<PathBuf> = OnceLock::new();

pub fn load_added(path: &Path) {
    let _ = ADDED.set(path.to_owned());
}

pub fn added() -> Vec<Vec<u8>> {
    ADDED.get().map_or_else(Vec::new, |path| split(&fs::read(path).unwrap_or_default()))
}

#[cfg(feature = "http")]
pub fn certificates() -> Vec<reqwest::Certificate> {
    let public = webpki_root_certs::TLS_SERVER_ROOT_CERTS.iter().map(|der| der.to_vec());
    public.chain(added()).filter_map(|der| reqwest::Certificate::from_der(&der).ok()).collect()
}

fn split(mut bytes: &[u8]) -> Vec<Vec<u8>> {
    let mut certs = Vec::new();
    while let Some((len, rest)) = bytes.split_first_chunk::<4>() {
        let len = u32::from_be_bytes(*len) as usize;
        let Some((cert, rest)) = rest.split_at_checked(len) else { break };
        certs.push(cert.to_vec());
        bytes = rest;
    }
    certs
}

#[cfg(test)]
mod tests {
    use super::split;

    #[test]
    fn splits_length_prefixed_certificates_and_drops_a_cut_one() {
        let bytes = [&[0, 0, 0, 2, 7, 8][..], &[0, 0, 0, 1, 9], &[0, 0, 0, 5, 1]].concat();
        assert_eq!(split(&bytes), vec![vec![7, 8], vec![9]]);
        assert!(split(&[0, 0]).is_empty());
    }
}
