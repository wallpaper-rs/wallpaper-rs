use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static SKIPPED: AtomicU64 = AtomicU64::new(0);

pub fn now() -> u64 {
    let real = SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_millis() as u64);
    real + SKIPPED.load(Ordering::Relaxed)
}

pub fn skip_clock(ms: u64) {
    SKIPPED.fetch_add(ms, Ordering::Relaxed);
}
