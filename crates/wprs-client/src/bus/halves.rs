use super::serials::{FIXED, Fixed};
use super::shared::Shared;
use std::io::{self, IoSlice};
use std::os::fd::{AsFd, BorrowedFd, OwnedFd};
use std::os::unix::net::UnixStream;
use std::sync::Arc;
use zbus::connection::socket::{BoxedSplit, ReadHalf, Split, WriteHalf};

#[derive(Debug)]
struct Reader {
    shared: Arc<Shared>,
    message: Vec<u8>,
    filled: usize,
    sized: bool,
    given: usize,
    fds: Vec<OwnedFd>,
}

#[derive(Debug)]
struct Writer {
    shared: Arc<Shared>,
    header: [u8; FIXED],
    total: usize,
    left: usize,
}

pub fn split(stream: UnixStream) -> io::Result<BoxedSplit> {
    stream.set_nonblocking(true)?;
    let shared = Arc::new(Shared::new(tokio::net::UnixStream::from_std(stream)?));
    let reader = Reader {
        shared: shared.clone(),
        message: vec![0; FIXED],
        filled: 0,
        sized: false,
        given: 0,
        fds: Vec::new(),
    };
    Ok(Split::new(Box::new(reader), Box::new(Writer { shared, header: [0; FIXED], total: 0, left: 0 })))
}

impl Reader {
    async fn receive_message(&mut self, buffer: &mut [u8]) -> io::Result<(usize, Vec<OwnedFd>)> {
        while !self.sized || self.filled < self.message.len() {
            if self.filled == self.message.len() {
                let total = Fixed::read(&self.message)?.len()?;
                self.message.resize(total, 0);
                self.sized = true;
                if self.filled == total {
                    break;
                }
            }
            let (read, fds) = self.shared.receive(&mut self.message[self.filled..]).await?;
            self.fds.extend(fds);
            self.filled += read;
        }
        if self.given == 0 {
            self.shared.serials().restore(&mut self.message);
        }
        let taken = (self.message.len() - self.given).min(buffer.len());
        buffer[..taken].copy_from_slice(&self.message[self.given..self.given + taken]);
        self.given += taken;
        if self.given == self.message.len() {
            self.message.truncate(FIXED);
            self.filled = 0;
            self.sized = false;
            self.given = 0;
        }
        Ok((taken, std::mem::take(&mut self.fds)))
    }
}

#[async_trait::async_trait]
impl ReadHalf for Reader {
    async fn recvmsg(&mut self, buffer: &mut [u8]) -> io::Result<(usize, Vec<OwnedFd>)> {
        {
            let mut answers = self.shared.answers();
            if !answers.is_empty() {
                let taken = answers.len().min(buffer.len());
                buffer[..taken].copy_from_slice(&answers[..taken]);
                answers.drain(..taken);
                return Ok((taken, Vec::new()));
            }
        }
        if self.shared.begun() {
            return self.receive_message(buffer).await;
        }
        self.shared.receive(buffer).await
    }

    fn can_pass_unix_fd(&self) -> bool {
        true
    }
}

impl Writer {
    async fn send_message(&mut self, buffer: &[u8], fds: &[BorrowedFd<'_>]) -> io::Result<usize> {
        if self.left == 0 {
            let start = buffer.get(..FIXED).ok_or_else(|| io::Error::from(io::ErrorKind::InvalidData))?;
            self.header.copy_from_slice(start);
            self.total = self.shared.serials().renumber(&mut self.header)?.len()?;
            self.left = self.total;
        }
        let at = self.total - self.left;
        let own = &buffer[..buffer.len().min(self.left)];
        let head = &self.header[at.min(FIXED)..];
        let cut = head.len().min(own.len());
        let sent = self.shared.send(&[IoSlice::new(&head[..cut]), IoSlice::new(&own[cut..])], fds).await?;
        self.left -= sent;
        Ok(sent)
    }
}

#[async_trait::async_trait]
impl WriteHalf for Writer {
    async fn sendmsg(&mut self, buffer: &[u8], fds: &[BorrowedFd<'_>]) -> io::Result<usize> {
        if self.shared.begun() {
            return self.send_message(buffer, fds).await;
        }
        let mut sent = 0;
        while let Some(end) = buffer[sent..].windows(2).position(|w| w == b"\r\n") {
            let line = &buffer[sent..sent + end + 2];
            let before = self.shared.lines();
            self.shared.send_all(line).await?;
            sent += line.len();
            if line.strip_prefix(b"\0").unwrap_or(line).starts_with(b"BEGIN") {
                self.shared.begin();
                return Ok(sent);
            }
            if !self.shared.await_answer(before).await? {
                return Ok(buffer.len());
            }
        }
        if sent == buffer.len() {
            return Ok(sent);
        }
        Ok(sent + self.shared.send(&[IoSlice::new(&buffer[sent..])], fds).await?)
    }

    async fn close(&mut self) -> io::Result<()> {
        Ok(rustix::net::shutdown(self.shared.socket.as_fd(), rustix::net::Shutdown::Both)?)
    }

    fn can_pass_unix_fd(&self) -> bool {
        true
    }
}
