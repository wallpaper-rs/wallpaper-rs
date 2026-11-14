use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use tokio_rustls::rustls::ServerConfig;
use tokio_rustls::rustls::crypto::aws_lc_rs;
use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};

pub const SERVICE: &str = "_wprs._tcp.local.";

pub fn host(public_url: &str) -> String {
    let rest = public_url.split_once("://").map_or(public_url, |(_, rest)| rest);
    let host = rest.split('/').next().unwrap_or_default();
    match host.strip_prefix('[') {
        Some(v6) => v6.split(']').next().unwrap_or_default().to_owned(),
        None => host.rsplit_once(':').map_or(host, |(name, _)| name).to_owned(),
    }
}

fn keep(path: &Path, bytes: &[u8]) -> Result<()> {
    let mut file = tempfile::NamedTempFile::new_in(path.parent().unwrap_or(Path::new(".")))?;
    file.write_all(bytes)?;
    file.as_file().sync_all()?;
    Ok(file.persist(path).map(drop)?)
}

pub fn certificate(data: &Path, host: &str) -> Result<(Vec<u8>, Vec<u8>)> {
    let dir = data.join("tls");
    let (cert, key) = (dir.join("cert.der"), dir.join("key.der"));
    if let (Ok(c), Ok(k)) = (fs::read(&cert), fs::read(&key)) {
        return Ok((c, k));
    }
    fs::create_dir_all(&dir)?;
    let names = vec![host.to_owned(), "localhost".to_owned()];
    let made = rcgen::generate_simple_self_signed(names).context("cannot make the server certificate")?;
    let (c, k) = (made.cert.der().to_vec(), made.signing_key.serialize_der());
    keep(&key, &k)?;
    keep(&cert, &c)?;
    Ok((c, k))
}

pub fn fingerprint(der: &[u8]) -> String {
    Sha256::digest(der).iter().map(|b| format!("{b:02x}")).collect()
}

pub fn acceptor(cert: Vec<u8>, key: Vec<u8>) -> Result<tokio_rustls::TlsAcceptor> {
    chain_acceptor(vec![CertificateDer::from(cert)], PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(key)))
}

pub fn chain_acceptor(
    chain: Vec<CertificateDer<'static>>,
    key: PrivateKeyDer<'static>,
) -> Result<tokio_rustls::TlsAcceptor> {
    let provider = Arc::new(aws_lc_rs::default_provider());
    let config = ServerConfig::builder_with_provider(provider).with_safe_default_protocol_versions()?;
    let config = config.with_no_client_auth().with_single_cert(chain, key)?;
    Ok(tokio_rustls::TlsAcceptor::from(Arc::new(config)))
}

pub fn advertise(port: u16) -> Result<mdns_sd::ServiceDaemon> {
    let daemon = mdns_sd::ServiceDaemon::new().context("cannot start mDNS")?;
    let name = format!("wallpaper-rs-{}", fingerprint(&getrandom::u64()?.to_le_bytes())[..6].to_owned());
    let host = format!("{name}.local.");
    let info =
        mdns_sd::ServiceInfo::new(SERVICE, &name, &host, "", port, &[("v", "0")][..])?.enable_addr_auto();
    daemon.register(info).context("cannot announce the server on the local network")?;
    Ok(daemon)
}
