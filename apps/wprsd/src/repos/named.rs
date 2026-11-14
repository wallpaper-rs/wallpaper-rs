use crate::error::{Error, failed, invalid, not_found};
use wprs_core::dbus::error::{EXISTS, NEWER, OFFLINE, STALE, UNTRUSTED};

impl From<wprs_format::Error> for Error {
    fn from(e: wprs_format::Error) -> Self {
        use wprs_format::Error as F;
        match e {
            F::NotFound(_) => not_found(e.to_string()),
            F::Exists(_) => Error::Named(EXISTS, e.to_string()),
            F::Invalid { reason, .. } => invalid(reason),
            F::UnsafePath(_) => invalid(e.to_string()),
            F::Stale(_) => Error::Named(STALE, e.to_string()),
            F::Newer(_) => Error::Named(NEWER, e.to_string()),
            F::Io(io) if io.kind() == std::io::ErrorKind::Interrupted => Error::Cancelled,
            F::Io(ref io) if unreachable(io.kind()) => Error::Named(OFFLINE, e.to_string()),
            F::Signature(_)
            | F::Key(_)
            | F::Mismatch(_)
            | F::SizeMismatch(_)
            | F::Rollback(_)
            | F::Equivocation(_)
            | F::Unanchored(_) => Error::Named(UNTRUSTED, e.to_string()),
            e => failed(e.to_string()),
        }
    }
}

fn unreachable(kind: std::io::ErrorKind) -> bool {
    use std::io::ErrorKind as K;
    matches!(
        kind,
        K::TimedOut | K::NotConnected | K::ConnectionRefused | K::ConnectionReset | K::HostUnreachable | K::NetworkUnreachable
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    fn name(e: wprs_format::Error) -> String {
        Error::from(e).parts().0
    }

    #[test]
    fn a_failed_download_or_check_is_named_by_its_kind() {
        let gone = |kind| wprs_format::Error::Io(io::Error::new(kind, "x"));
        assert_eq!(name(gone(io::ErrorKind::NotConnected)), OFFLINE);
        assert_eq!(name(gone(io::ErrorKind::TimedOut)), OFFLINE);
        assert_eq!(name(wprs_format::Error::Signature("bad".into())), UNTRUSTED);
        assert_eq!(name(wprs_format::Error::SizeMismatch("a.wpak".into())), UNTRUSTED);
        assert_eq!(name(wprs_format::Error::Rollback("index".into())), UNTRUSTED);
        assert_eq!(name(gone(io::ErrorKind::Other)), "org.freedesktop.DBus.Error.Failed");
        assert!(matches!(Error::from(gone(io::ErrorKind::Interrupted)), Error::Cancelled));
    }
}
