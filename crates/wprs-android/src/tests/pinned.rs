use crate::repos;
use crate::state::{read, update};
use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::PathBuf;
use std::sync::Arc;
use tokio_rustls::rustls::pki_types::pem::PemObject;
use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer};
use tokio_rustls::rustls::{ServerConfig, ServerConnection, StreamOwned};
use wprs_format::{Settings, Signer, add, init, keygen, publish, sign_root};

const PEM: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/fixtures/loopback-tls.pem");

fn secure(dir: PathBuf) -> u16 {
    let pem = fs::read(PEM).unwrap();
    let chain = CertificateDer::pem_slice_iter(&pem).collect::<Result<Vec<_>, _>>().unwrap();
    let key = PrivateKeyDer::from_pem_slice(&pem).unwrap();
    let config =
        Arc::new(ServerConfig::builder().with_no_client_auth().with_single_cert(chain, key).unwrap());
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    std::thread::spawn(move || {
        for socket in listener.incoming().flatten() {
            let mut tls = StreamOwned::new(ServerConnection::new(config.clone()).unwrap(), socket);
            let mut head = Vec::new();
            let mut byte = [0u8];
            while !head.ends_with(b"\r\n\r\n") && tls.read_exact(&mut byte).is_ok() {
                head.push(byte[0]);
            }
            let text = String::from_utf8_lossy(&head).into_owned();
            let path = text.split_whitespace().nth(1).unwrap_or("/").trim_start_matches('/');
            let reply = match fs::read(dir.join(path.split('?').next().unwrap_or_default())) {
                Ok(body) => [
                    format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n", body.len())
                        .into_bytes(),
                    body,
                ]
                .concat(),
                Err(_) => {
                    b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n".to_vec()
                }
            };
            drop(tls.write_all(&reply));
            tls.conn.send_close_notify();
            drop(tls.flush());
        }
    });
    port
}

fn connected(url: String, pin: Option<String>) {
    update(|s| (s.url, s.pin) = (url, pin)).unwrap();
}

pub(super) fn the_servers_own_certificate_is_trusted_only_for_its_own_origin() {
    let tmp = tempfile::tempdir().unwrap();
    let repo = tmp.path().join("repo");
    let (root, online) = (keygen(None).unwrap(), keygen(None).unwrap());
    let settings = Settings {
        name: "Home".into(),
        root_keys: vec![root.public.clone()],
        online_keys: vec![online.public.clone()],
        ..Settings::default()
    };
    let now = crate::library::now();
    init(&repo, &settings, now).unwrap();
    sign_root(&repo, &Signer::new(&root.secret, None).unwrap()).unwrap();
    add(&repo, &super::previewed::package(tmp.path(), "org.example.home"), None, now).unwrap();
    publish(&repo, &Signer::new(&online.secret, None).unwrap(), 7, now).unwrap();
    let (server, other) = (secure(repo.clone()), secure(repo));
    let address = |port: u16| format!("https://127.0.0.1:{port}/?fingerprint={}", root.fingerprint);
    let pem = fs::read(PEM).unwrap();
    let leaf = CertificateDer::pem_slice_iter(&pem).next().unwrap().unwrap();
    let print = wprs_sync::fingerprint(&leaf);
    let before = read(|s| (s.url.clone(), s.pin.clone()));
    connected(format!("https://127.0.0.1:{server}"), None);
    assert!(repos::add(&address(server)).is_err(), "the system does not trust the server's own certificate");
    connected(format!("https://127.0.0.1:{server}"), Some("0".repeat(64)));
    assert!(repos::add(&address(server)).is_err(), "another pin is refused");
    connected(format!("https://127.0.0.1:{server}"), Some(print));
    let elsewhere = repos::add(&address(other));
    assert!(elsewhere.is_err(), "another origin with the same certificate is not trusted: {elsewhere:?}");
    let added = repos::add(&address(server)).unwrap();
    let repo_id = added["repo_id"].as_str().unwrap().to_owned();
    let found = repos::browse("").unwrap();
    assert!(found.as_array().unwrap().iter().any(|i| i["id"] == "org.example.home"), "{found}");
    repos::remove(&repo_id).unwrap();
    connected(before.0, before.1);
}
