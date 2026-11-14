use crate::Renderer;
use crate::scene::stops::{UNBUILT, stopped};
use crate::scene::{builders, clock, flight, packages};
use std::sync::{Arc, atomic::AtomicBool, mpsc};
use std::time::Instant;

const ROOM: usize = 64 << 20;

pub(crate) type Trial = Box<dyn FnOnce(&wgpu::Device) + Send>;

pub(crate) fn left(key: u64) -> Option<String> {
    if stopped(key) {
        return Some(UNBUILT.into());
    }
    clock::slow(key)
}

fn attempt<T>(device: &wgpu::Device, work: impl FnOnce(&wgpu::Device) -> T) -> Result<T, String> {
    let (made, failure) = crate::lost::caught(device, || work(device));
    failure.map_or(Ok(made), |e| Err(e.to_string()))
}

pub(crate) fn within<T: Send + 'static>(
    (key, r): (u64, &Renderer),
    trial: Option<Trial>,
    work: impl FnOnce(&wgpu::Device) -> T + Send + 'static,
) -> Result<T, String> {
    let Some(probe) = r.probe() else { return elsewhere(key, &r.device, work) };
    let Some(trial) = trial else { return attempt(&r.device, work) };
    let tried =
        elsewhere(key, &r.device, move |_| probe.open().map_or(Ok(()), |device| attempt(&device, trial)));
    tried.and_then(|t| t)?;
    flight::building(key, true);
    let made = wprs_scene::trying(key).and_then(|()| attempt(&r.device, work));
    flight::building(key, false);
    wprs_scene::tried(key);
    made
}

fn elsewhere<T: Send + 'static>(
    key: u64,
    device: &wgpu::Device,
    work: impl FnOnce(&wgpu::Device) -> T + Send + 'static,
) -> Result<T, String> {
    later(key, device, Arc::new(|| {}), work)?.wait()
}

pub(crate) type Waker = Arc<dyn Fn() + Send + Sync>;

pub(crate) struct Later<T> {
    key: u64,
    rx: mpsc::Receiver<Result<T, String>>,
    until: Instant,
}

pub(crate) fn later<T: Send + 'static>(
    key: u64,
    device: &wgpu::Device,
    wake: Waker,
    work: impl FnOnce(&wgpu::Device) -> T + Send + 'static,
) -> Result<Later<T>, String> {
    let (tx, rx) = mpsc::channel();
    let device = device.clone();
    let job = move || {
        flight::building(key, true);
        let made = wprs_scene::trying(key).and_then(|()| attempt(&device, work));
        flight::building(key, false);
        wprs_scene::tried(key);
        drop(tx.send(made));
        wake();
    };
    let thread = std::thread::Builder::new().name("wprs-build".into()).stack_size(ROOM).spawn(job);
    let thread = thread.map_err(|e| format!("cannot start building it: {e}"))?;
    builders::keep(thread, Arc::new(AtomicBool::new(false)));
    Ok(Later { key, rx, until: Instant::now() + packages::build_timeout() })
}

impl<T> Later<T> {
    pub(crate) fn until(&self) -> Instant {
        self.until
    }

    pub(crate) fn poll(&self, now: Instant) -> Option<Result<T, String>> {
        match self.rx.try_recv() {
            Ok(made) => Some(made),
            Err(mpsc::TryRecvError::Disconnected) => Some(Err("its build stopped".into())),
            Err(mpsc::TryRecvError::Empty) if now >= self.until => Some(self.late()),
            Err(mpsc::TryRecvError::Empty) => None,
        }
    }

    pub(crate) fn wait(self) -> Result<T, String> {
        let left = self.until.saturating_duration_since(Instant::now());
        self.rx.recv_timeout(left).unwrap_or_else(|_| self.late())
    }

    fn late(&self) -> Result<T, String> {
        if !flight::built(self.key) && wprs_scene::strike(&[self.key]).is_err() {
            tracing::warn!("cannot remember that a shader took too long to build");
        }
        let limit = packages::build_timeout();
        Err(format!("its shader took more than {} s to build", limit.as_secs_f32()))
    }
}
