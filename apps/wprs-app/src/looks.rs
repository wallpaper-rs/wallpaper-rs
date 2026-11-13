#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "C++" {
        include!("wprs-app/look.h");
        #[rust_name = "install_look"]
        pub fn installLook(style: &QString, theme: &QString);
        #[rust_name = "dress_look"]
        fn dressLook(family: &QString, pixels: i32);
        #[rust_name = "paint_look"]
        fn paintLook(tones: &QString);
        #[rust_name = "look_style"]
        fn lookStyle() -> QString;
        #[rust_name = "look_family"]
        fn lookFamily() -> QString;
        #[rust_name = "look_has_family"]
        fn lookHasFamily(family: &QString) -> bool;
        #[rust_name = "look_has_icon"]
        fn lookHasIcon(name: &QString) -> bool;
    }

    #[auto_cxx_name]
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        type Looks = super::LooksRust;

        #[qinvokable]
        fn dress(self: &Looks, family: &QString, pixels: i32);
        #[qinvokable]
        fn paint(self: &Looks, tones: &QString);
        #[qinvokable]
        fn style(self: &Looks) -> QString;
        #[qinvokable]
        fn family(self: &Looks) -> QString;
        #[qinvokable]
        fn has_family(self: &Looks, family: &QString) -> bool;
        #[qinvokable]
        fn has_icon(self: &Looks, name: &QString) -> bool;
        #[qinvokable]
        fn icon_names(self: &Looks) -> QString;
    }
}

use cxx_qt_lib::QString;
use qobject::*;

pub const STYLE: &str = "io.github.wallpaper_rs.style";
pub const THEME: &str = "wprs";
pub const NAMES: &str = include_str!("../look/icons/names.txt");

#[derive(Default)]
pub struct LooksRust;

pub fn install() {
    install_look(&QString::from(STYLE), &QString::from(THEME));
}

pub fn names() -> impl Iterator<Item = (&'static str, &'static str)> {
    NAMES.lines().filter_map(|line| line.split_once(' '))
}

impl Looks {
    fn dress(&self, family: &QString, pixels: i32) {
        dress_look(family, pixels);
    }

    fn paint(&self, tones: &QString) {
        paint_look(tones);
    }

    fn style(&self) -> QString {
        look_style()
    }

    fn family(&self) -> QString {
        look_family()
    }

    fn has_family(&self, family: &QString) -> bool {
        look_has_family(family)
    }

    fn has_icon(&self, name: &QString) -> bool {
        look_has_icon(name)
    }

    fn icon_names(&self) -> QString {
        let all: Vec<&str> = names().flat_map(|(old, new)| [old, new]).collect();
        QString::from(all.join(",").as_str())
    }
}
