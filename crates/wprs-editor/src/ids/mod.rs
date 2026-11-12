mod clip;
mod gone;
mod refs;
mod rename;

pub use clip::{copy, paste};
pub use gone::forget;
pub use refs::{Hold, Ref, refs, swap, users};
pub use rename::rename;
