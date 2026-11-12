use crate::runtime::environment;
use crate::{Map, normalize, tensor};
use anyhow::{Context, Result, anyhow, bail, ensure};
use image::RgbImage;
use ort::environment::Environment;
use ort::ep::ExecutionProvider;
use ort::session::{RunOptions, Session};
use ort::value::Tensor;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::time::Duration;
use wprs_core::message::said;

pub const TELEMETRY: &str = "ORT_DISABLE_TELEMETRY";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Loading,
    Estimating,
    Saving,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Device {
    #[default]
    Auto,
    Cpu,
    Cuda,
}

impl std::str::FromStr for Device {
    type Err = anyhow::Error;

    fn from_str(text: &str) -> Result<Self> {
        match text {
            "auto" => Ok(Device::Auto),
            "cpu" => Ok(Device::Cpu),
            "cuda" => Ok(Device::Cuda),
            _ => {
                bail!("{}", said("depth.is_not_auto_cpu_or", "%1 is not auto, cpu or cuda", &[&text]).local())
            }
        }
    }
}

impl Device {
    pub fn name(self) -> &'static str {
        ["auto", "cpu", "cuda"][self as usize]
    }
}

pub struct Estimator {
    session: Session,
    device: Device,
    _environment: Environment,
}

pub(crate) fn text(e: impl std::fmt::Display) -> anyhow::Error {
    anyhow!("{e}")
}

pub fn threads() -> usize {
    std::thread::available_parallelism().map_or(1, |n| n.get().saturating_sub(1).max(1))
}

pub fn background() {
    unsafe { libc::setpriority(libc::PRIO_PROCESS, 0, 19) };
}

pub(crate) struct Opened {
    pub(crate) session: Session,
    pub(crate) device: Device,
    pub(crate) environment: Environment,
}

pub(crate) fn open_session(
    model: &Path,
    libraries: &[PathBuf],
    threads: usize,
    device: Device,
) -> Result<Opened> {
    let environment = environment(libraries)?;
    let session = |gpu: bool| -> Result<Session> {
        let cpu = ort::ep::CPU::default().with_arena_allocator(false).build();
        let cuda = gpu.then(|| ort::ep::CUDA::default().build().error_on_failure());
        let providers: Vec<_> = cuda.into_iter().chain([cpu]).collect();
        Session::builder(&environment)
            .map_err(text)?
            .with_execution_providers(providers)
            .map_err(text)?
            .with_intra_threads(threads)
            .map_err(text)?
            .with_memory_pattern(false)
            .map_err(text)?
            .commit_from_file(model)
            .map_err(text)
    };
    let cuda = device != Device::Cpu && ort::ep::CUDA::is_available().unwrap_or(false);
    let (session, device) = match (device, cuda) {
        (Device::Cuda, false) => {
            bail!(
                "{}",
                said(
                    "depth.this_onnx_runtime_cannot_use",
                    "this ONNX Runtime cannot use CUDA; give one built with CUDA in ORT_DYLIB_PATH",
                    &[]
                )
                .local()
            )
        }
        (Device::Cuda, true) => (
            session(true).context(
                said("depth.cannot_run_the_model_with", "cannot run the model with CUDA", &[]).local(),
            )?,
            Device::Cuda,
        ),
        (_, true) => match session(true) {
            Ok(session) => (session, Device::Cuda),
            Err(e) => {
                eprintln!(
                    "{}",
                    said(
                        "depth.the_model_runs_on_the",
                        "the model runs on the processor: %1",
                        &[&format!("{e:#}")]
                    )
                    .local()
                );
                (session(false)?, Device::Cpu)
            }
        },
        _ => (session(false)?, Device::Cpu),
    };
    Ok(Opened { session, device, environment })
}

impl Estimator {
    pub fn open(model: &Path, libraries: &[PathBuf], threads: usize, device: Device) -> Result<Self> {
        let Opened { session, device, environment } = open_session(model, libraries, threads, device)?;
        Ok(Self { session, device, _environment: environment })
    }

    pub fn device(&self) -> Device {
        self.device
    }

    pub fn estimate(&mut self, image: &RgbImage, input: u32) -> Result<Map> {
        self.estimate_until(image, input, &AtomicBool::new(false))
    }

    pub fn estimate_until(&mut self, image: &RgbImage, input: u32, stop: &AtomicBool) -> Result<Map> {
        let (data, (w, h)) = tensor(image, input);
        let value = Tensor::from_array(([1usize, 3, h as usize, w as usize], data)).map_err(text)?;
        let options = RunOptions::new().map_err(text)?;
        let done = AtomicBool::new(false);
        let outputs = std::thread::scope(|scope| {
            scope.spawn(|| {
                while !done.load(Relaxed) {
                    if stop.load(Relaxed) {
                        return drop(options.terminate());
                    }
                    std::thread::sleep(Duration::from_millis(100));
                }
            });
            let ran = self.session.run_with_options(ort::inputs![value], &options);
            done.store(true, Relaxed);
            ran
        });
        ensure!(!stop.load(Relaxed), Stopped);
        let outputs = outputs.map_err(text)?;
        let (shape, values) = outputs[0].try_extract_tensor::<f32>().map_err(text)?;
        let &[.., oh, ow] = &shape[..] else {
            bail!("{}", said("depth.the_model_gave_no_depth", "the model gave no depth", &[]).local())
        };
        Ok(normalize(values, (ow as u32, oh as u32), image.dimensions(), input))
    }
}

#[derive(Debug, thiserror::Error)]
#[error("the depth map was cancelled")]
pub struct Stopped;
