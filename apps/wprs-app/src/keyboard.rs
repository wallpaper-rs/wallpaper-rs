#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("wprs-app/keys.h");
        #[rust_name = "install_keys"]
        pub fn installKeys();
        #[rust_name = "send_key"]
        pub fn sendKey(key: i32, modifiers: i32);
        #[rust_name = "send_wheel"]
        pub fn sendWheel(x: f64, y: f64, delta: i32);
    }
}

use crate::backend::qobject::Backend;

pub use qobject::install_keys;

impl Backend {
    pub fn press(&self, key: i32, modifiers: i32) {
        if crate::bus::checking() {
            qobject::send_key(key, modifiers);
        }
    }

    pub fn wheel(&self, x: f64, y: f64, delta: i32) {
        if crate::bus::checking() {
            qobject::send_wheel(x, y, delta);
        }
    }
}
