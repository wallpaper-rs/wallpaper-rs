#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[auto_cxx_name]
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        type ConnectCode = super::ConnectCodeRust;

        #[qinvokable]
        fn modules(self: &ConnectCode, server: &QString, fingerprint: &QString) -> QString;
    }
}

use cxx_qt_lib::QString;
use qobject::ConnectCode;
use wprs_qr::{Code, Link};

#[derive(Default)]
pub struct ConnectCodeRust;

pub fn modules(server: &str, fingerprint: &str) -> String {
    let print = Some(fingerprint.trim()).filter(|f| !f.is_empty());
    let Ok(code) = Link::new(server.trim(), print, None).and_then(|link| Code::new(&link.text())) else {
        return String::new();
    };
    let size = code.size();
    (0..size * size).map(|at| if code.dark(at % size, at / size) { '1' } else { '0' }).collect()
}

impl ConnectCode {
    fn modules(&self, server: &QString, fingerprint: &QString) -> QString {
        QString::from(modules(&server.to_string(), &fingerprint.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_server_with_its_fingerprint_gives_a_square_of_modules() {
        let bits = modules("https://home.example:8443", &"ab".repeat(32));
        let side = (bits.len() as f64).sqrt() as usize;
        assert!(side >= 21);
        assert_eq!(side * side, bits.len());
        assert!(bits.contains('1') && bits.contains('0'));
    }

    #[test]
    fn the_code_carries_nothing_but_the_address_and_the_fingerprint() {
        let link = Link::new("https://home.example", Some(&"cd".repeat(32)), None).unwrap();
        assert_eq!(
            link.text(),
            format!("wprs://connect/1?server=https://home.example&fp={}", "cd".repeat(32))
        );
        assert!(!modules("https://home.example", "").is_empty());
    }

    #[test]
    fn a_wrong_address_or_fingerprint_draws_nothing() {
        assert_eq!(modules("not an address", ""), "");
        assert_eq!(modules("https://home.example", "xyz"), "");
    }
}
