use std::sync::LazyLock;
use tokio::runtime::{Builder, Runtime};

static OUTSIDE: LazyLock<Runtime> = LazyLock::new(|| {
    let mut builder = Builder::new_multi_thread();
    builder.worker_threads(2).thread_name("wprsd-outside").enable_all();
    builder.build().unwrap_or_else(|e| crate::fatal(&format!("cannot start the network runtime: {e}")))
});

pub fn block_on<T>(work: impl Future<Output = T>) -> T {
    OUTSIDE.block_on(work)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering::SeqCst};
    use std::time::Duration;

    #[test]
    fn blocking_work_outlives_the_shutdown_of_the_runtime_that_started_it() {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let (started, finished) = (Arc::new(AtomicBool::new(false)), Arc::new(AtomicBool::new(false)));
        let (go, done) = (started.clone(), finished.clone());
        runtime.spawn_blocking(move || {
            go.store(true, SeqCst);
            std::thread::sleep(Duration::from_millis(300));
            super::block_on(async { tokio::time::sleep(Duration::from_millis(20)).await });
            done.store(true, SeqCst);
        });
        while !started.load(SeqCst) {
            std::thread::yield_now();
        }
        runtime.shutdown_timeout(Duration::from_secs(60));
        assert!(finished.load(SeqCst));
    }
}
