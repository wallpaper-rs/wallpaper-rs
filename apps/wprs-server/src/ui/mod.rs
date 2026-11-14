mod blocks;
mod case;
mod forms;
mod rows;
mod size;
mod steps;

pub use blocks::{copyable, copyable_with, empty, fix_fold, fold, notice};
pub use case::{back_link, case_head, confirm, subheading};
pub use forms::{
    Kind, button, button_named, checkbox, disabled_button, field, file_picker, form_actions, link_button,
};
pub use rows::{Row, pairs, quiet_line, rows, status, status_line};
pub use size::size;
pub use steps::{Choice, choices, key_box, qr, steps};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tone {
    Good,
    Warn,
    Bad,
}

impl Tone {
    pub fn class(self) -> &'static str {
        match self {
            Tone::Good => "bien",
            Tone::Warn => "aviso",
            Tone::Bad => "mal",
        }
    }

    pub fn icon(self) -> &'static str {
        match self {
            Tone::Good => "check-circle-fill",
            Tone::Warn => "warning-fill",
            Tone::Bad => "warning-octagon-fill",
        }
    }
}
