use futures_util::StreamExt;
use std::io::{Read, Write};
use std::num::NonZeroU32;
use std::os::unix::net::UnixStream;
use std::thread::JoinHandle;
use std::time::Duration;
use zbus::message::{Message, Type};
use zbus::wire::Endian;

fn line(server: &mut UnixStream) -> Option<Vec<u8>> {
    let mut text = Vec::new();
    let mut byte = [0];
    while !text.ends_with(b"\r\n") {
        server.read_exact(&mut byte).ok()?;
        text.push(byte[0]);
    }
    Some(text)
}

fn answer(serial: u32, body: &[u8], count: usize) -> Vec<u8> {
    let call = Message::method_call("/", "Echo").serial(NonZeroU32::new(serial).unwrap()).build(&()).unwrap();
    let header = call.header();
    let order = if count.is_multiple_of(2) { Endian::Little } else { Endian::Big };
    let reply = Message::method_return(&header).endian(order);
    let built = match body.get(body.len().saturating_sub(4)..) {
        Some(tag) if body.len() == 4 => reply.build(&u32::from_le_bytes([tag[0], tag[1], tag[2], tag[3]])),
        _ => reply.build(&":1.1"),
    };
    built.unwrap().data().to_vec()
}

fn fake_bus(mut server: UnixStream) -> JoinHandle<Vec<u32>> {
    std::thread::spawn(move || {
        server.set_read_timeout(Some(Duration::from_secs(10))).unwrap();
        loop {
            let Some(command) = line(&mut server) else { return Vec::new() };
            let text = command.strip_prefix(b"\0").unwrap_or(&command);
            let reply: &[u8] = match text.split(|b| *b == b' ' || *b == b'\r').next().unwrap_or_default() {
                b"AUTH" => b"OK 0123456789abcdef0123456789abcdef\r\n",
                b"NEGOTIATE_UNIX_FD" => b"AGREE_UNIX_FD\r\n",
                b"BEGIN" => break,
                other => panic!("unexpected handshake command {:?}", String::from_utf8_lossy(other)),
            };
            server.write_all(reply).unwrap();
        }
        let mut seen = Vec::new();
        let mut fixed = [0; 16];
        while server.read_exact(&mut fixed).is_ok() {
            let word =
                |at: usize| u32::from_le_bytes([fixed[at], fixed[at + 1], fixed[at + 2], fixed[at + 3]]);
            let total = (16 + word(12) as usize).div_ceil(8) * 8 + word(4) as usize;
            let mut rest = vec![0; total - 16];
            server.read_exact(&mut rest).unwrap();
            seen.push(word(8));
            if fixed[1] != 1 {
                continue;
            }
            let bytes = answer(word(8), &rest[rest.len() - word(4) as usize..], seen.len());
            let (first, second) = bytes.split_at(7);
            server.write_all(first).unwrap();
            std::thread::sleep(Duration::from_millis(3));
            server.write_all(second).unwrap();
        }
        seen
    })
}

fn echo(tag: u32) -> Message {
    Message::method_call("/io/github/wallpaper_rs", "Echo")
        .destination("io.github.wallpaper_rs")
        .build(&tag)
        .unwrap()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn messages_reach_the_bus_in_increasing_serial_order_and_replies_reach_their_callers() {
    let (client, server) = UnixStream::pair().unwrap();
    let fake = fake_bus(server);
    let builder = zbus::connection::Builder::socket(wprs_client::bus::split(client).unwrap());
    let connection = tokio::time::timeout(Duration::from_secs(10), builder.build()).await.unwrap().unwrap();
    let mut stream = zbus::MessageStream::from(&connection);
    let older = echo(1);
    let newer = echo(2);
    connection.send(&newer).await.unwrap();
    connection.send(&older).await.unwrap();
    let mut answered = Vec::new();
    while answered.len() < 2 {
        let message =
            tokio::time::timeout(Duration::from_secs(10), stream.next()).await.unwrap().unwrap().unwrap();
        if message.message_type() == Type::MethodReturn {
            let header = message.header();
            answered.push((header.reply_serial().unwrap(), message.body().deserialize::<u32>().unwrap()));
        }
    }
    answered.sort();
    let mut expected = [(older.primary_header().serial_num(), 1), (newer.primary_header().serial_num(), 2)];
    expected.sort();
    assert_eq!(answered, expected);
    let callers = (10..40u32).map(|tag| {
        let connection = connection.clone();
        tokio::spawn(async move {
            let reply = connection
                .call_method(
                    Some("io.github.wallpaper_rs"),
                    "/io/github/wallpaper_rs",
                    None::<&str>,
                    "Echo",
                    &tag,
                )
                .await
                .unwrap();
            (tag, reply.body().deserialize::<u32>().unwrap())
        })
    });
    for caller in callers.collect::<Vec<_>>() {
        let (tag, got) = tokio::time::timeout(Duration::from_secs(10), caller).await.unwrap().unwrap();
        assert_eq!(got, tag);
    }
    drop(stream);
    connection.graceful_shutdown().await;
    let seen = fake.join().unwrap();
    assert_eq!(seen.len(), 33);
    assert!(seen.windows(2).all(|w| w[0] < w[1]), "the bus saw serials out of order: {seen:?}");
}
