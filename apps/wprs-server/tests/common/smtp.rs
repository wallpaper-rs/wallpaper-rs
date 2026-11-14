use std::sync::{Arc, Mutex};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Answer {
    Accept,
    RefusePassword,
}

pub struct Inbox {
    pub port: u16,
    pub got: Arc<Mutex<Vec<String>>>,
    pub answer: Arc<Mutex<Answer>>,
}

impl Inbox {
    pub fn mails(&self) -> Vec<String> {
        self.got.lock().unwrap().clone()
    }

    pub fn answer(&self, answer: Answer) {
        *self.answer.lock().unwrap() = answer;
    }
}

pub async fn listen() -> Inbox {
    let socket = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = socket.local_addr().unwrap().port();
    let (got, answer) = (Arc::new(Mutex::new(Vec::new())), Arc::new(Mutex::new(Answer::Accept)));
    let (kept, said) = (got.clone(), answer.clone());
    tokio::spawn(async move {
        while let Ok((stream, _)) = socket.accept().await {
            let (kept, said) = (kept.clone(), said.clone());
            tokio::spawn(async move { drop(talk(stream, kept, said).await) });
        }
    });
    Inbox { port, got, answer }
}

pub fn closed_port() -> u16 {
    std::net::TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap().port()
}

fn unfold(text: &str) -> String {
    let joined = text.replace("=\n", "");
    let (mut bytes, raw) = (Vec::new(), joined.as_bytes());
    let mut i = 0;
    while i < raw.len() {
        let hex =
            raw.get(i + 1..i + 3).and_then(|h| u8::from_str_radix(std::str::from_utf8(h).ok()?, 16).ok());
        match (raw[i], hex) {
            (b'=', Some(byte)) => {
                bytes.push(byte);
                i += 3;
            }
            (byte, _) => {
                bytes.push(byte);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

async fn talk(
    stream: TcpStream,
    got: Arc<Mutex<Vec<String>>>,
    answer: Arc<Mutex<Answer>>,
) -> std::io::Result<()> {
    let (read, mut write) = stream.into_split();
    let mut lines = BufReader::new(read).lines();
    write.write_all(b"220 mail.example.org ESMTP\r\n").await?;
    let mut data: Option<String> = None;
    while let Some(line) = lines.next_line().await? {
        if let Some(text) = data.as_mut() {
            if line == "." {
                got.lock().unwrap().push(unfold(&data.take().unwrap_or_default()));
                write.write_all(b"250 2.0.0 queued\r\n").await?;
            } else {
                text.push_str(&line);
                text.push('\n');
            }
            continue;
        }
        let verb = line.split(' ').next().unwrap_or_default().to_ascii_uppercase();
        let refuse = *answer.lock().unwrap() == Answer::RefusePassword;
        let reply: &[u8] = match verb.as_str() {
            "EHLO" | "HELO" => b"250-mail.example.org\r\n250-AUTH PLAIN LOGIN\r\n250 8BITMIME\r\n",
            "AUTH" if refuse => b"535 5.7.8 authentication failed\r\n",
            "AUTH" => b"235 2.7.0 accepted\r\n",
            "DATA" => {
                data = Some(String::new());
                b"354 go on\r\n"
            }
            "QUIT" => {
                write.write_all(b"221 2.0.0 bye\r\n").await?;
                return Ok(());
            }
            _ => b"250 2.0.0 ok\r\n",
        };
        write.write_all(reply).await?;
    }
    Ok(())
}
