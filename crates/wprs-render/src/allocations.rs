use crate::draw::TARGET_FORMAT;
use crate::{Layer, Renderer};
use std::alloc::{GlobalAlloc, Layout, System};
use std::backtrace::{Backtrace, BacktraceStatus};
use std::cell::Cell;
use wprs_core::Fit;

mod traced;

struct Counted;

thread_local! {
    static ALLOCATIONS: Cell<u64> = const { Cell::new(0) };
}

fn count() {
    if traced::counted() {
        let _ = ALLOCATIONS.try_with(|n| n.set(n.get() + 1));
        traced::record();
    }
}

unsafe impl GlobalAlloc for Counted {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        count();
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        count();
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        count();
        unsafe { System.realloc(ptr, layout, size) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static COUNTED: Counted = Counted;

pub(crate) fn allocations(work: impl FnOnce()) -> u64 {
    let before = ALLOCATIONS.with(Cell::get);
    work();
    ALLOCATIONS.with(Cell::get) - before
}

const STEADY: &str = "allocations::steady_frames_allocate_nothing_beyond_the_gpu_calls_they_make";

fn without_lock_traces(test: &str) {
    let out = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", test, "--test-threads", "1", "--nocapture"])
        .env("RUST_LIB_BACKTRACE", "0")
        .output()
        .unwrap();
    let (stdout, stderr) = (String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    assert!(out.status.success() && stdout.contains("1 passed"), "{stdout}{stderr}");
}

#[inline(never)]
fn deeper(n: u32, work: &mut dyn FnMut() -> u64) -> u64 {
    std::hint::black_box(if n == 0 { work() } else { deeper(n - 1, work) })
}

#[test]
fn steady_frames_allocate_nothing_beyond_the_gpu_calls_they_make() {
    if Backtrace::capture().status() == BacktraceStatus::Captured {
        return without_lock_traces(STEADY);
    }
    let r = match Renderer::shared() {
        Ok(r) => r,
        Err(e) => return assert!(std::env::var_os("WPRS_REQUIRE_GPU").is_none(), "GPU required: {e}"),
    };
    let red = r.upload_rgba(&[255, 0, 0, 255].repeat(4), 2, 2, 8).unwrap();
    let blue = r.upload_rgba(&[0, 0, 255, 255].repeat(4), 2, 2, 8).unwrap();
    let layer = |image, opacity| Layer {
        image: Some(image),
        fit: Fit::Fill,
        background: [0; 3],
        opacity,
        uv: None,
        relief: None,
        flip: [false; 2],
        grade: None,
    };
    let layers = [layer(&red, 1.0), layer(&blue, 0.5)];
    let mut target = r.create_target(32, 16).unwrap();
    for _ in 0..3 {
        r.draw_layers(&mut target, &layers).unwrap();
        r.finish().unwrap();
    }
    let [opaque, blended] = r.layers.get(&r.device, TARGET_FORMAT);
    let (view, groups) =
        (target.view.clone(), [target.slots[0].group.clone(), target.slots[1].group.clone()]);
    let bare = || {
        let memory = r.device.push_error_scope(wgpu::ErrorFilter::OutOfMemory);
        let validation = r.device.push_error_scope(wgpu::ErrorFilter::Validation);
        let mut encoder = r.device.create_command_encoder(&Default::default());
        let ops =
            wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), store: wgpu::StoreOp::Store };
        let attachment =
            wgpu::RenderPassColorAttachment { view: &view, depth_slice: None, resolve_target: None, ops };
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("wprs-render pass"),
            color_attachments: &[Some(attachment)],
            ..Default::default()
        });
        pass.set_pipeline(&opaque);
        pass.set_bind_group(0, &groups[0], &[]);
        pass.draw(0..3, 0..1);
        pass.set_pipeline(&blended);
        pass.set_blend_constant(wgpu::Color { r: 0.5, g: 0.5, b: 0.5, a: 0.5 });
        pass.set_bind_group(0, &groups[1], &[]);
        pass.draw(0..3, 0..1);
        drop(pass);
        r.queue.submit([encoder.finish()]);
        assert!(
            [validation.pop(), memory.pop()].into_iter().all(|scope| pollster::block_on(scope).is_none())
        );
    };
    let mut draw = || r.draw_layers(&mut target, &layers).unwrap();
    if let Some(dir) = std::env::var_os("WPRS_ALLOC_TRACE") {
        traced::trace(std::path::Path::new(&dir), bare, &mut draw, || r.finish().unwrap());
    }
    let (mut ours, mut theirs) = ([u64::MAX; 4], u64::MAX);
    for _ in 0..8 {
        theirs = theirs.min(allocations(bare));
        for (fewest, depth) in ours.iter_mut().zip([0, 4, 8, 16]) {
            r.finish().unwrap();
            *fewest = (*fewest).min(deeper(depth, &mut || allocations(&mut draw)));
        }
        r.finish().unwrap();
    }
    let ours = ours.into_iter().max().unwrap();
    assert!(ours <= theirs, "a steady frame made {ours} allocations, its GPU calls alone make {theirs}");
}
