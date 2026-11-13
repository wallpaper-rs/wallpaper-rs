use crate::link::Error;
use qrcodegen::{QrCode, QrCodeEcc};

const QUIET: i32 = 4;

pub struct Code(QrCode);

impl Code {
    pub fn new(text: &str) -> Result<Code, Error> {
        QrCode::encode_text(text, QrCodeEcc::Medium).map(Code).map_err(|_| Error::TooLong)
    }

    pub fn size(&self) -> i32 {
        self.0.size()
    }

    pub fn dark(&self, x: i32, y: i32) -> bool {
        self.0.get_module(x, y)
    }

    pub fn svg(&self) -> String {
        let (size, mut path) = (self.size(), String::new());
        for y in 0..size {
            let mut x = 0;
            while x < size {
                if !self.dark(x, y) {
                    x += 1;
                    continue;
                }
                let start = x;
                while x < size && self.dark(x, y) {
                    x += 1;
                }
                path.push_str(&format!("M{},{}h{}v1h-{}z", start + QUIET, y + QUIET, x - start, x - start));
            }
        }
        let side = size + 2 * QUIET;
        format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {side} {side}\" \
             shape-rendering=\"crispEdges\" role=\"img\"><path fill=\"currentColor\" d=\"{path}\"/></svg>"
        )
    }

    pub fn blocks(&self, light_terminal: bool) -> String {
        let size = self.size();
        let on = |x: i32, y: i32| self.dark(x, y) == light_terminal;
        let mut out = String::new();
        for y in (-QUIET..size + QUIET).step_by(2) {
            for x in -QUIET..size + QUIET {
                out.push(match (on(x, y), on(x, y + 1)) {
                    (true, true) => '█',
                    (true, false) => '▀',
                    (false, true) => '▄',
                    (false, false) => ' ',
                });
            }
            out.push('\n');
        }
        out
    }
}
