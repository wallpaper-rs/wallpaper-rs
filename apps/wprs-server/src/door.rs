use crate::settings::{Network, Settings};
use crate::{App, Config, certs, lan};
use anyhow::Result;
use axum::serve::Listener;
use std::io;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, PoisonError, RwLock};
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, ReadBuf};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio_rustls::TlsAcceptor;
use tokio_rustls::server::TlsStream;

#[derive(Default)]
pub struct Door {
    tls: RwLock<Option<TlsAcceptor>>,
    announced: Mutex<Option<mdns_sd::ServiceDaemon>>,
    live: AtomicBool,
}

pub fn acceptor(cfg: &Config, network: &Network) -> Result<Option<TlsAcceptor>> {
    Ok(match network {
        Network::Home => {
            let (cert, key) = lan::certificate(&cfg.data, &lan::host(&cfg.public_url))?;
            tracing::info!("serving HTTPS with its own certificate, fingerprint {}", lan::fingerprint(&cert));
            Some(lan::acceptor(cert, key)?)
        }
        Network::Own { cert, key } if !cert.is_empty() => {
            let (chain, secret) = certs::load(cert, key).map_err(anyhow::Error::msg)?;
            Some(lan::chain_acceptor(chain, secret)?)
        }
        Network::Own { .. } => None,
    })
}

impl Door {
    pub fn new(cfg: &Config, network: &Network) -> Result<Door> {
        let door = Door::default();
        match acceptor(cfg, network) {
            Ok(found) => *door.tls.write().unwrap_or_else(PoisonError::into_inner) = found,
            Err(e) => tracing::warn!("serving without TLS: {e:#}"),
        }
        Ok(door)
    }

    fn current(&self) -> Option<TlsAcceptor> {
        self.tls.read().unwrap_or_else(PoisonError::into_inner).clone()
    }

    pub fn apply(&self, cfg: &Config, settings: &Settings) -> Result<()> {
        let found = acceptor(cfg, &settings.network)?;
        *self.tls.write().unwrap_or_else(PoisonError::into_inner) = found;
        let mut announced = self.announced.lock().unwrap_or_else(PoisonError::into_inner);
        match (settings.advertise && self.live.load(Ordering::Relaxed), announced.is_some()) {
            (true, false) => *announced = Some(lan::advertise(cfg.listen.port())?),
            (false, true) => drop(announced.take()),
            _ => {}
        }
        Ok(())
    }
}

impl App {
    pub async fn serve_door(self: Arc<App>, tcp: TcpListener) -> Result<()> {
        self.door.live.store(true, Ordering::Relaxed);
        self.door.apply(&self.cfg, &self.settings())?;
        let (sent, got) = mpsc::channel(64);
        let local = tcp.local_addr()?;
        let app = self.clone();
        tokio::spawn(async move {
            loop {
                let Ok((socket, peer)) = tcp.accept().await else {
                    tokio::time::sleep(Duration::from_millis(100)).await;
                    continue;
                };
                let (app, sent) = (app.clone(), sent.clone());
                tokio::spawn(async move {
                    let opened = tokio::time::timeout(Duration::from_secs(10), open(&app, socket, peer));
                    if let Ok(Some(io)) = opened.await {
                        drop(sent.send((io, peer)).await);
                    }
                });
            }
        });
        self.serve(Mixed { got, local }).await
    }
}

async fn open(app: &App, socket: TcpStream, peer: SocketAddr) -> Option<Io> {
    let mut first = [0u8; 1];
    socket.peek(&mut first).await.ok()?;
    match (first[0] == 0x16, app.door.current()) {
        (true, Some(tls)) => tls.accept(socket).await.ok().map(|s| Io::Tls(Box::new(s))),
        (false, Some(_)) if !peer.ip().to_canonical().is_loopback() => {
            moved(socket, &app.cfg.public_url).await;
            None
        }
        _ => Some(Io::Plain(socket)),
    }
}

async fn moved(mut socket: TcpStream, public: &str) {
    let mut head = [0u8; 2048];
    let read = socket.read(&mut head).await.unwrap_or(0);
    let line = String::from_utf8_lossy(&head[..read]);
    let path = line.split(' ').nth(1).filter(|p| crate::pages::local(p) == *p).unwrap_or("/");
    let reply = format!(
        "HTTP/1.1 308 Permanent Redirect\r\nLocation: {public}{path}\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
    );
    drop(socket.write_all(reply.as_bytes()).await);
}

pub enum Io {
    Plain(TcpStream),
    Tls(Box<TlsStream<TcpStream>>),
}

impl AsyncRead for Io {
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &mut ReadBuf<'_>) -> Poll<io::Result<()>> {
        match self.get_mut() {
            Io::Plain(s) => Pin::new(s).poll_read(cx, buf),
            Io::Tls(s) => Pin::new(s.as_mut()).poll_read(cx, buf),
        }
    }
}

impl AsyncWrite for Io {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<io::Result<usize>> {
        match self.get_mut() {
            Io::Plain(s) => Pin::new(s).poll_write(cx, buf),
            Io::Tls(s) => Pin::new(s.as_mut()).poll_write(cx, buf),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match self.get_mut() {
            Io::Plain(s) => Pin::new(s).poll_flush(cx),
            Io::Tls(s) => Pin::new(s.as_mut()).poll_flush(cx),
        }
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match self.get_mut() {
            Io::Plain(s) => Pin::new(s).poll_shutdown(cx),
            Io::Tls(s) => Pin::new(s.as_mut()).poll_shutdown(cx),
        }
    }
}

pub struct Mixed {
    got: mpsc::Receiver<(Io, SocketAddr)>,
    local: SocketAddr,
}

impl Listener for Mixed {
    type Io = Io;
    type Addr = SocketAddr;

    async fn accept(&mut self) -> (Self::Io, Self::Addr) {
        match self.got.recv().await {
            Some(found) => found,
            None => std::future::pending().await,
        }
    }

    fn local_addr(&self) -> io::Result<SocketAddr> {
        Ok(self.local)
    }
}
