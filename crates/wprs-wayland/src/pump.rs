use crate::Event;
use crate::control::Wake;
use rustix::event::{PollFd, PollFlags, Timespec, poll};
use smithay_client_toolkit::reexports::client::{DispatchError, EventQueue};
use std::time::Instant;
use wayland_backend::client::WaylandError;

pub(crate) fn dispatch<S>(
    queue: &mut EventQueue<S>,
    state: &mut S,
    until: Option<Instant>,
    wake: Option<&std::os::unix::net::UnixStream>,
) -> Result<usize, DispatchError> {
    let dispatched = queue.dispatch_pending(state)?;
    if dispatched > 0 {
        return Ok(dispatched);
    }
    queue.flush()?;
    if let Some(guard) = queue.prepare_read() {
        let wait = until
            .map(|at| Timespec::try_from(at.saturating_duration_since(Instant::now())).unwrap_or_default());
        let fd = guard.connection_fd();
        let mut fds = vec![PollFd::new(&fd, PollFlags::IN | PollFlags::ERR)];
        fds.extend(wake.map(|w| PollFd::new(w, PollFlags::IN)));
        let ready = poll(&mut fds, wait.as_ref());
        let woken = fds.get(1).is_some_and(|w| !w.revents().is_empty());
        let readable = fds[0].revents().intersects(PollFlags::IN | PollFlags::ERR | PollFlags::HUP);
        if woken && let Some(w) = wake {
            Wake::drain(w);
        }
        if ready.is_ok_and(|n| n > 0) && readable {
            match guard.read() {
                Err(WaylandError::Io(e)) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                read => drop(read?),
            }
        }
    }
    queue.dispatch_pending(state)
}

pub(crate) fn ended(error: DispatchError) -> Event {
    use std::io::ErrorKind::{BrokenPipe, ConnectionAborted, ConnectionReset, NotConnected, UnexpectedEof};
    match error {
        DispatchError::Backend(WaylandError::Io(e))
            if matches!(
                e.kind(),
                BrokenPipe | ConnectionReset | ConnectionAborted | NotConnected | UnexpectedEof
            ) =>
        {
            Event::Lost(e.to_string())
        }
        error => Event::Failed(error.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use smithay_client_toolkit::reexports::client::Connection;
    use wayland_backend::protocol::ProtocolError;

    #[test]
    fn protocol_errors_fail_and_a_closed_connection_ends() {
        let (code, object_id, object_interface) = (2, 9, "zwlr_layer_surface_v1".into());
        let error = ProtocolError { code, object_id, object_interface, message: "invalid size".into() };
        assert!(matches!(ended(DispatchError::Backend(WaylandError::Protocol(error))), Event::Failed(_)));
        let io = |kind| DispatchError::Backend(WaylandError::Io(std::io::Error::from(kind)));
        assert!(matches!(ended(io(std::io::ErrorKind::BrokenPipe)), Event::Lost(_)));
        assert!(matches!(ended(io(std::io::ErrorKind::InvalidData)), Event::Failed(_)));
    }

    #[test]
    fn a_pending_retry_wakes_the_session_without_any_event() {
        let (ours, _compositor) = std::os::unix::net::UnixStream::pair().unwrap();
        let mut queue = Connection::from_socket(ours).unwrap().new_event_queue::<()>();
        let at = Instant::now() + std::time::Duration::from_millis(50);
        assert_eq!(dispatch(&mut queue, &mut (), Some(at), None).unwrap(), 0);
        assert!(Instant::now() >= at, "the session woke before the retry was due");
        assert_eq!(dispatch(&mut queue, &mut (), Some(at), None).unwrap(), 0, "a due retry waited");
    }

    #[test]
    fn a_wake_ends_the_wait_before_the_retry_is_due() {
        let (ours, _compositor) = std::os::unix::net::UnixStream::pair().unwrap();
        let mut queue = Connection::from_socket(ours).unwrap().new_event_queue::<()>();
        let (wake, listener) = Wake::pair().unwrap();
        let later = Instant::now() + std::time::Duration::from_secs(30);
        let started = Instant::now();
        std::io::Write::write_all(&mut &wake.tx, &[1]).unwrap();
        assert_eq!(dispatch(&mut queue, &mut (), Some(later), Some(&listener)).unwrap(), 0);
        assert!(started.elapsed() < std::time::Duration::from_secs(5), "the wake did not end the wait");
        let mut rest = [0u8; 1];
        assert!(std::io::Read::read(&mut &listener, &mut rest).is_err(), "the wake was not drained");
    }
}
