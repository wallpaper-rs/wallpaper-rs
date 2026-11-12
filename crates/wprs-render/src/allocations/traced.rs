use std::backtrace::Backtrace;
use std::cell::{Cell, RefCell};
use std::path::Path;

thread_local! {
    static TRACING: Cell<bool> = const { Cell::new(false) };
    static INSIDE: Cell<bool> = const { Cell::new(false) };
    static STACKS: RefCell<Vec<Backtrace>> = const { RefCell::new(Vec::new()) };
}

pub(super) fn counted() -> bool {
    !INSIDE.try_with(Cell::get).unwrap_or(true)
}

pub(super) fn record() {
    if !TRACING.try_with(Cell::get).unwrap_or(false) {
        return;
    }
    INSIDE.set(true);
    let stack = Backtrace::force_capture();
    STACKS.with_borrow_mut(|s| s.push(stack));
    INSIDE.set(false);
}

pub(super) fn traced(work: impl FnOnce()) -> (u64, String) {
    TRACING.set(true);
    let n = super::allocations(work);
    TRACING.set(false);
    INSIDE.set(true);
    let stacks = STACKS.with_borrow_mut(std::mem::take);
    let text = stacks.iter().map(|s| format!("=== allocation\n{s}\n")).collect();
    INSIDE.set(false);
    (n, text)
}

pub(super) fn trace(dir: &Path, mut bare: impl FnMut(), mut ours: impl FnMut(), mut settle: impl FnMut()) {
    let mut fewest = [(u64::MAX, String::new()), (u64::MAX, String::new())];
    for _ in 0..8 {
        let runs = [traced(&mut bare), {
            settle();
            traced(&mut ours)
        }];
        settle();
        for (best, run) in fewest.iter_mut().zip(runs) {
            if run.0 < best.0 {
                *best = run;
            }
        }
    }
    std::fs::create_dir_all(dir).unwrap();
    for (name, (n, text)) in ["bare", "ours"].into_iter().zip(fewest) {
        std::fs::write(dir.join(format!("{name}-{n}.txt")), text).unwrap();
    }
}
