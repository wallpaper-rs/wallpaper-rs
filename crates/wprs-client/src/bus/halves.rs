use rustix::net::{
    RecvAncillaryBuffer, RecvAncillaryMessage, RecvFlags, SendAncillaryBuffer, SendAncillaryMessage,
    SendFlags, recvmsg, sendmsg,
};
use std::io::{self, IoSlice, IoSliceMut};
use std::mem::MaybeUninit;
use std::os::fd::{AsFd, BorrowedFd, OwnedFd};
use std::os::unix::net::UnixStream;
use std::sync::{Arc, Mutex, PoisonError};
use tokio::io::Interest;
use zbus::connection::socket::{BoxedSplit, ReadHalf, Split, WriteHalf};

const FDS_MAX: usize = 1024;

#[derive(Debug)]
struct Shared {
    socket: tokio::net::UnixStream,
    answers: Mutex<Vec<u8>>,
}

#[derive(Debug)]
struct Reader(Arc<Shared>);

#[derive(Debug)]
struct Writer {
    shared: Arc<Shared>,
    begun: bool,
}

pub fn split(stream: UnixStream) -> io::Result<BoxedSplit> {
    stream.set_nonblocking(true)?;
    let shared =
        Arc::new(Shared { socket: tokio::net::UnixStream::from_std(stream)?, answers: Mutex::default() });
    Ok(Split::new(Box::new(Reader(shared.clone())), Box::new(Writer { shared, begun: false })))
}

impl Shared {
    fn answers(&self) -> std::sync::MutexGuard<'_, Vec<u8>> {
        self.answers.lock().unwrap_or_else(PoisonError::into_inner)
    }

    async fn receive(&self, buffer: &mut [u8]) -> io::Result<(usize, Vec<OwnedFd>)> {
        loop {
            self.socket.readable().await?;
            match self.socket.try_io(Interest::READABLE, || receive(self.socket.as_fd(), buffer)) {
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                result => return result,
            }
        }
    }

    async fn send(&self, buffer: &[u8], fds: &[BorrowedFd<'_>]) -> io::Result<usize> {
        loop {
            self.socket.writable().await?;
            match self.socket.try_io(Interest::WRITABLE, || send(self.socket.as_fd(), buffer, fds)) {
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                result => return result,
            }
        }
    }

    async fn send_all(&self, mut buffer: &[u8]) -> io::Result<()> {
        while !buffer.is_empty() {
            buffer = &buffer[self.send(buffer, &[]).await?..];
        }
        Ok(())
    }

    fn lines(&self) -> usize {
        self.answers().windows(2).filter(|w| w == b"\r\n").count()
    }

    async fn await_answer(&self, before: usize) -> io::Result<bool> {
        let mut chunk = [0; 256];
        loop {
            {
                let answers = self.answers();
                if answers.windows(2).filter(|w| w == b"\r\n").count() > before {
                    let last = answers[..answers.len() - 2].windows(2).rposition(|w| w == b"\r\n");
                    return Ok(!answers[last.map_or(0, |at| at + 2)..].starts_with(b"REJECTED"));
                }
            }
            let (read, _) = self.receive(&mut chunk).await?;
            self.answers().extend_from_slice(&chunk[..read]);
        }
    }
}

#[async_trait::async_trait]
impl ReadHalf for Reader {
    async fn recvmsg(&mut self, buffer: &mut [u8]) -> io::Result<(usize, Vec<OwnedFd>)> {
        {
            let mut answers = self.0.answers();
            if !answers.is_empty() {
                let taken = answers.len().min(buffer.len());
                buffer[..taken].copy_from_slice(&answers[..taken]);
                answers.drain(..taken);
                return Ok((taken, Vec::new()));
            }
        }
        self.0.receive(buffer).await
    }

    fn can_pass_unix_fd(&self) -> bool {
        true
    }
}

#[async_trait::async_trait]
impl WriteHalf for Writer {
    async fn sendmsg(&mut self, buffer: &[u8], fds: &[BorrowedFd<'_>]) -> io::Result<usize> {
        if self.begun {
            return self.shared.send(buffer, fds).await;
        }
        let mut sent = 0;
        while let Some(end) = buffer[sent..].windows(2).position(|w| w == b"\r\n") {
            let line = &buffer[sent..sent + end + 2];
            let before = self.shared.lines();
            self.shared.send_all(line).await?;
            sent += line.len();
            if line.strip_prefix(b"\0").unwrap_or(line).starts_with(b"BEGIN") {
                self.begun = true;
                return Ok(sent);
            }
            if !self.shared.await_answer(before).await? {
                return Ok(buffer.len());
            }
        }
        if sent == buffer.len() {
            return Ok(sent);
        }
        Ok(sent + self.shared.send(&buffer[sent..], fds).await?)
    }

    async fn close(&mut self) -> io::Result<()> {
        Ok(rustix::net::shutdown(self.shared.socket.as_fd(), rustix::net::Shutdown::Both)?)
    }

    fn can_pass_unix_fd(&self) -> bool {
        true
    }
}

fn receive(socket: BorrowedFd<'_>, buffer: &mut [u8]) -> io::Result<(usize, Vec<OwnedFd>)> {
    let mut space = [MaybeUninit::uninit(); rustix::cmsg_space!(ScmRights(FDS_MAX))];
    let mut ancillary = RecvAncillaryBuffer::new(&mut space);
    let message = recvmsg(socket, &mut [IoSliceMut::new(buffer)], &mut ancillary, RecvFlags::CMSG_CLOEXEC)?;
    if message.bytes == 0 {
        return Err(io::Error::new(io::ErrorKind::BrokenPipe, "failed to read from socket"));
    }
    let mut fds = Vec::new();
    for item in ancillary.drain() {
        if let RecvAncillaryMessage::ScmRights(received) = item {
            fds.extend(received);
        }
    }
    Ok((message.bytes, fds))
}

fn send(socket: BorrowedFd<'_>, buffer: &[u8], fds: &[BorrowedFd<'_>]) -> io::Result<usize> {
    let mut space = [MaybeUninit::uninit(); rustix::cmsg_space!(ScmRights(FDS_MAX))];
    let mut ancillary = SendAncillaryBuffer::new(&mut space);
    if !fds.is_empty() && !ancillary.push(SendAncillaryMessage::ScmRights(fds)) {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "too many file descriptors"));
    }
    let sent = sendmsg(socket, &[IoSlice::new(buffer)], &mut ancillary, SendFlags::NOSIGNAL)?;
    if sent == 0 {
        return Err(io::Error::new(io::ErrorKind::WriteZero, "failed to write to socket"));
    }
    Ok(sent)
}
