use std::io::{ErrorKind, Read, Write};
use std::os::unix::net::UnixStream;
use std::thread::JoinHandle;
use std::time::Duration;

fn lines(bytes: &[u8]) -> usize {
    bytes.windows(2).filter(|w| w == b"\r\n").count()
}

fn quiet(mut server: &UnixStream) -> bool {
    server.set_nonblocking(true).unwrap();
    let pending = server.read(&mut [0; 1]);
    server.set_nonblocking(false).unwrap();
    matches!(pending, Err(e) if e.kind() == ErrorKind::WouldBlock)
}

fn fake_bus(mut server: UnixStream, auth_answer: &'static [u8]) -> JoinHandle<Vec<String>> {
    std::thread::spawn(move || {
        server.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
        let mut seen = Vec::new();
        let mut buffer = [0; 4096];
        loop {
            let read = server.read(&mut buffer).unwrap_or(0);
            if read == 0 {
                break;
            }
            let got = &buffer[..read];
            let text = got.strip_prefix(b"\0").unwrap_or(got);
            if text.starts_with(b"BEGIN\r\n") {
                seen.push("BEGIN".to_string());
                break;
            }
            let shown = String::from_utf8_lossy(got).into_owned();
            assert_eq!(lines(text), 1, "more than one handshake command in one read: {shown:?}");
            assert!(text.ends_with(b"\r\n"), "a handshake command was cut: {shown:?}");
            std::thread::sleep(Duration::from_millis(50));
            assert!(quiet(&server), "the client went on before the answer to {shown:?}");
            let command =
                String::from_utf8_lossy(text).split_whitespace().next().unwrap_or_default().to_string();
            let answer: &[u8] = match command.as_str() {
                "AUTH" => auth_answer,
                "NEGOTIATE_UNIX_FD" => b"AGREE_UNIX_FD\r\n",
                _ => panic!("unexpected handshake command {shown:?}"),
            };
            seen.push(command);
            server.write_all(answer).unwrap();
        }
        seen
    })
}

async fn connect(client: UnixStream) -> zbus::Result<zbus::Connection> {
    let builder = zbus::connection::Builder::socket(wprs_client::bus::split(client)?);
    tokio::time::timeout(Duration::from_secs(10), builder.build()).await.expect("the handshake hung")
}

#[tokio::test(flavor = "multi_thread")]
async fn each_handshake_command_waits_for_the_answer_to_the_one_before() {
    let (client, server) = UnixStream::pair().unwrap();
    let fake = fake_bus(server, b"OK 0123456789abcdef0123456789abcdef\r\n");
    let _ = connect(client).await;
    assert_eq!(fake.join().unwrap(), ["AUTH", "NEGOTIATE_UNIX_FD", "BEGIN"]);
}

#[tokio::test(flavor = "multi_thread")]
async fn a_rejected_mechanism_sends_nothing_more_and_fails() {
    let (client, server) = UnixStream::pair().unwrap();
    let fake = fake_bus(server, b"REJECTED EXTERNAL\r\n");
    assert!(connect(client).await.is_err());
    assert_eq!(fake.join().unwrap(), ["AUTH"]);
}
