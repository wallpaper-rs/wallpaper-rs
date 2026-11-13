use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use tokio_rustls::rustls::ServerConfig;
use tokio_rustls::rustls::pki_types::pem::PemObject;
use tokio_rustls::rustls::pki_types::{CertificateDer, PrivateKeyDer};
use wprs_core::roots;

const PEM: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../tests/fixtures/loopback-tls.pem");

async fn serve(chain: Vec<CertificateDer<'static>>) -> String {
    let key = PrivateKeyDer::from_pem_slice(&std::fs::read(PEM).unwrap()).unwrap();
    let config = ServerConfig::builder().with_no_client_auth().with_single_cert(chain, key).unwrap();
    let (acceptor, listener) =
        (TlsAcceptor::from(Arc::new(config)), TcpListener::bind("127.0.0.1:0").await.unwrap());
    let url = format!("https://127.0.0.1:{}/", listener.local_addr().unwrap().port());
    tokio::spawn(async move {
        while let Ok((socket, _)) = listener.accept().await {
            let acceptor = acceptor.clone();
            tokio::spawn(async move {
                let Ok(mut tls) = acceptor.accept(socket).await else { return };
                let mut asked = Vec::new();
                while !asked.ends_with(b"\r\n\r\n") && tls.read_u8().await.map(|b| asked.push(b)).is_ok() {}
                let reply = "HTTP/1.1 200 OK\r\ncontent-length: 2\r\nconnection: close\r\n\r\nok";
                drop(tls.write_all(reply.as_bytes()).await);
                drop(tls.shutdown().await);
            });
        }
    });
    url
}

fn client() -> reqwest::Client {
    reqwest::Client::builder().tls_certs_only(roots::certificates()).build().unwrap()
}

#[tokio::test(flavor = "multi_thread")]
async fn the_phone_trusts_the_public_roots_and_what_the_user_added_from_the_next_client_on() {
    let chain =
        CertificateDer::pem_slice_iter(&std::fs::read(PEM).unwrap()).collect::<Result<Vec<_>, _>>().unwrap();
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("roots.der");
    roots::load_added(&file);
    let public = roots::certificates().len();
    assert!(public > 100, "{public}");
    let url = serve(chain.clone()).await;
    let refused = client().get(&url).send().await.unwrap_err();
    assert!(refused.is_connect(), "{refused:?}");
    let der = chain[0].as_ref();
    std::fs::write(&file, [&(der.len() as u32).to_be_bytes()[..], der].concat()).unwrap();
    assert_eq!(roots::certificates().len(), public + 1);
    let answer = client().get(&url).send().await.unwrap();
    assert_eq!(answer.text().await.unwrap(), "ok");
}
