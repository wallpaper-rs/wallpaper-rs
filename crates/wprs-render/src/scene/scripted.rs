#[cfg(target_os = "android")]
#[path = "scripted/none.rs"]
mod imp;
#[cfg(not(target_os = "android"))]
#[path = "scripted/real.rs"]
mod imp;

pub use imp::*;
