use crate::Source;
use crate::modules::Modules;
use rquickjs::{CatchResultExt, CaughtError, Context, Module, Runtime};

const MEMORY: usize = 32 << 20;
const STACK: usize = 256 << 10;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fault {
    pub line: Option<u32>,
    pub message: String,
}

pub fn syntax(sources: &[Source], first: &[String]) -> Vec<(usize, Fault)> {
    let parts = Runtime::new().and_then(|runtime| {
        runtime.set_memory_limit(MEMORY);
        runtime.set_max_stack_size(STACK);
        let modules =
            Modules { sources: sources.iter().cloned().collect(), first: first.iter().cloned().collect() };
        runtime.set_loader(modules.clone(), modules);
        Context::full(&runtime).map(|context| (runtime, context))
    });
    let Ok((_runtime, context)) = parts else { return Vec::new() };
    sources
        .iter()
        .enumerate()
        .filter_map(|(i, (name, source))| {
            let found = context.with(|ctx| {
                Module::declare(ctx.clone(), name.as_str(), source.as_str())
                    .catch(&ctx)
                    .err()
                    .map(|caught| fault(&caught))
            });
            found.map(|fault| (i, fault))
        })
        .collect()
}

fn fault(caught: &CaughtError<'_>) -> Fault {
    match caught {
        CaughtError::Exception(e) => {
            Fault { line: e.stack().as_deref().and_then(line_of), message: e.message().unwrap_or_default() }
        }
        other => Fault { line: None, message: other.to_string() },
    }
}

fn line_of(stack: &str) -> Option<u32> {
    let place = stack.lines().find_map(|l| l.trim().strip_prefix("at "))?;
    let mut parts = place.rsplitn(3, ':');
    let (_column, line) = (parts.next()?, parts.next()?);
    line.parse().ok()
}
