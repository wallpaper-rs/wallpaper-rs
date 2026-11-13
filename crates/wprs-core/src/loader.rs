//! Serialises the first-time loading of foreign graphics and media libraries.
//!
//! The graphics driver takes its own dispatch lock and then asks the dynamic loader for its lock
//! (it opens another library); a media codec plug-in is opened while the loader holds its lock and
//! then asks for the graphics lock. Two threads doing one each can wait for each other for ever.
//! Every one-time path that may open such a library runs inside [`serialised`], so the two orders
//! never interleave. The lock is taken on one-time paths only, never per frame.

use std::cell::Cell;
use std::sync::{Mutex, PoisonError};

static LOADER: Mutex<()> = Mutex::new(());

thread_local! {
    static DEPTH: Cell<u32> = const { Cell::new(0) };
}

struct Leave;

impl Drop for Leave {
    fn drop(&mut self) {
        DEPTH.with(|d| d.set(d.get() - 1));
    }
}

/// Runs `work` while holding the process-wide loader lock. Calls nested on one thread are fine.
pub fn serialised<T>(work: impl FnOnce() -> T) -> T {
    let nested = DEPTH.with(|d| d.replace(d.get() + 1)) > 0;
    let _leave = Leave;
    let _held = (!nested).then(|| LOADER.lock().unwrap_or_else(PoisonError::into_inner));
    work()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nested_calls_on_one_thread_do_not_deadlock() {
        assert_eq!(serialised(|| serialised(|| serialised(|| 7))), 7);
    }

    #[test]
    fn a_panic_releases_the_lock() {
        let _ = std::panic::catch_unwind(|| serialised(|| panic!("boom")));
        assert_eq!(serialised(|| 1), 1);
    }

    #[test]
    fn threads_never_overlap() {
        let inside = std::sync::atomic::AtomicU32::new(0);
        std::thread::scope(|s| {
            for _ in 0..8 {
                s.spawn(|| {
                    for _ in 0..50 {
                        serialised(|| {
                            assert_eq!(inside.fetch_add(1, std::sync::atomic::Ordering::SeqCst), 0);
                            inside.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                        });
                    }
                });
            }
        });
    }
}
