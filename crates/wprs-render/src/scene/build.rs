use super::chains::{Shelf, lock};
use super::flight::building;
use super::make::Recipe;
use super::pipes::{Key, Shader};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::sync::{Arc, Mutex, PoisonError, mpsc};
use wgpu::{BindGroupLayout, RenderPipeline, ShaderModule};
use wprs_scene::Descriptor;

const ROOM: usize = 64 << 20;
const WORKERS: usize = 4;

#[derive(Clone)]
pub(crate) struct Job {
    pub(crate) d: Arc<Descriptor>,
    pub(crate) wanted: Vec<Key>,
    pub(crate) variants: Vec<u32>,
}

pub(crate) struct Kit {
    pub(crate) device: wgpu::Device,
    pub(crate) groups: [BindGroupLayout; 2],
    pub(crate) shelf: Arc<Mutex<Shelf>>,
    pub(crate) probe: Option<super::probe::Probe>,
}

pub(crate) type Built = (u64, Result<(), String>);

impl Kit {
    pub(crate) fn start(
        self: &Arc<Self>,
        jobs: Vec<Job>,
        stop: Arc<AtomicBool>,
    ) -> Result<mpsc::Receiver<Built>, String> {
        let (tx, rx) = mpsc::channel();
        let workers = jobs.len().min(WORKERS);
        let (kit, jobs) = (self.clone(), Arc::new(Mutex::new(VecDeque::from(jobs))));
        for _ in 0..workers {
            let (kit, jobs, halt, tx) = (kit.clone(), jobs.clone(), stop.clone(), tx.clone());
            let work = move || {
                let mut trial = None;
                loop {
                    let next = jobs.lock().unwrap_or_else(PoisonError::into_inner).pop_front();
                    let Some(job) = next.filter(|_| !halt.load(Relaxed)) else { break };
                    let key = job.d.key;
                    building(key, true);
                    drop(wprs_scene::trying(key));
                    let built = match &kit.probe {
                        Some(probe) => {
                            trial.get_or_insert_with(|| probe.open()).as_ref().map_or(Ok(()), |device| {
                                make(device, &super::pipes::package_layouts(device), &job, None).map(drop)
                            })
                        }
                        None => kit.build(&job),
                    };
                    building(key, false);
                    if tx.send((key, built)).is_err() {
                        break;
                    }
                }
            };
            let thread = std::thread::Builder::new().name("wprs-build".into()).stack_size(ROOM);
            let thread = thread.spawn(work).map_err(|e| format!("cannot start building it: {e}"))?;
            super::builders::keep(thread, stop.clone());
        }
        Ok(rx)
    }

    pub(crate) fn build(&self, job: &Job) -> Result<(), String> {
        let key = job.d.key;
        let have = lock(&self.shelf).modules.get(&key).cloned();
        let (module, made) = make(&self.device, &self.groups, job, have)?;
        let mut shelf = lock(&self.shelf);
        if shelf.held.contains_key(&key) {
            shelf.modules.entry(key).or_insert(module);
            shelf.owned.entry(key).or_insert_with(|| job.d.clone());
            for (k, p) in made {
                shelf.cache.entry(k).or_insert(p);
            }
        }
        Ok(())
    }
}

type Made = (ShaderModule, Vec<(Key, RenderPipeline)>);

fn make(
    device: &wgpu::Device,
    groups: &[BindGroupLayout; 2],
    job: &Job,
    have: Option<ShaderModule>,
) -> Result<Made, String> {
    let parsed = match have {
        Some(_) => None,
        None => Some(wprs_scene::guarded(&job.d)?),
    };
    let (made, failure) = crate::lost::caught(device, || {
        let module = have.unwrap_or_else(|| {
            let source = wgpu::ShaderSource::Naga(std::borrow::Cow::Owned(parsed.expect("parsed above")));
            let label = Some("wprs-scene package shader");
            device.create_shader_module(wgpu::ShaderModuleDescriptor { label, source })
        });
        let made: Vec<(Key, RenderPipeline)> = (job.wanted.iter())
            .map(|&(shader, mode, format)| {
                let recipe = Recipe::own(&job.d, module.clone(), groups.to_vec(), shader);
                ((shader, mode, format), recipe.make(device, mode, format))
            })
            .collect();
        (module, made)
    });
    failure.map_or(Ok(made), |e| Err(e.to_string()))
}

pub(crate) fn own(key: u64, pass: usize, variant: u32) -> Shader {
    Shader::Own(key, u8::try_from(pass).unwrap_or(u8::MAX), variant)
}
