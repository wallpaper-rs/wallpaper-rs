use super::serials::Serials;
use rustix::net::{
    RecvAncillaryBuffer, RecvAncillaryMessage, RecvFlags, SendAncillaryBuffer, SendAncillaryMessage,
    SendFlags, recvmsg, sendmsg,
};
use std::io::{self, IoSlice, IoSliceMut};
use std::mem::MaybeUninit;
use std::os::fd::{AsFd, BorrowedFd, OwnedFd};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, MutexGuard, PoisonError};
use tokio::io::Interest;

const FDS_MAX: usize = 1024;

#[derive(Debug)]
pub struct Shared {
    pub socket: tokio::net::UnixStream,
    answers: Mutex<Vec<u8>>,
    serials: Mutex<Serials>,
    begun: AtomicBool,
}

impl Shared {
    pub fn new(socket: tokio::net::UnixStream) -> Self {
        Self { socket, answers: Mutex::default(), serials: Mutex::default(), begun: AtomicBool::new(false) }
    }

    pub fn answers(&self) -> MutexGuard<'_, Vec<u8>> {
        self.answers.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub fn serials(&self) -> MutexGuard<'_, Serials> {
        self.serials.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub fn begun(&self) -> bool {
        self.begun.load(Ordering::Acquire)
    }

    pub fn begin(&self) {
        self.begun.store(true, Ordering::Release);
    }

    pub async fn receive(&self, buffer: &mut [u8]) -> io::Result<(usize, Vec<OwnedFd>)> {
        loop {
            self.socket.readable().await?;
            match self.socket.try_io(Interest::READABLE, || receive(self.socket.as_fd(), buffer)) {
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                result => return result,
            }
        }
    }

    pub async fn send(&self, buffer: &[IoSlice<'_>], fds: &[BorrowedFd<'_>]) -> io::Result<usize> {
        loop {
            self.socket.writable().await?;
            match self.socket.try_io(Interest::WRITABLE, || send(self.socket.as_fd(), buffer, fds)) {
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                result => return result,
            }
        }
    }

    pub async fn send_all(&self, mut buffer: &[u8]) -> io::Result<()> {
        while !buffer.is_empty() {
            buffer = &buffer[self.send(&[IoSlice::new(buffer)], &[]).await?..];
        }
        Ok(())
    }

    pub fn lines(&self) -> usize {
        self.answers().windows(2).filter(|w| w == b"\r\n").count()
    }

    pub async fn await_answer(&self, before: usize) -> io::Result<bool> {
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

fn send(socket: BorrowedFd<'_>, buffer: &[IoSlice<'_>], fds: &[BorrowedFd<'_>]) -> io::Result<usize> {
    let mut space = [MaybeUninit::uninit(); rustix::cmsg_space!(ScmRights(FDS_MAX))];
    let mut ancillary = SendAncillaryBuffer::new(&mut space);
    if !fds.is_empty() && !ancillary.push(SendAncillaryMessage::ScmRights(fds)) {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "too many file descriptors"));
    }
    let sent = sendmsg(socket, buffer, &mut ancillary, SendFlags::NOSIGNAL)?;
    if sent == 0 {
        return Err(io::Error::new(io::ErrorKind::WriteZero, "failed to write to socket"));
    }
    Ok(sent)
}
