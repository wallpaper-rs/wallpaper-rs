mod app;
#[macro_use]
mod args;
mod backend;
mod bus;
#[macro_use]
mod callmacro;
mod calls;
mod checkrun;
mod choices;
mod depth;
mod depthproc;
mod depthview;
mod depthwork;
mod diff;
mod editing;
mod editor;
mod editsteps;
mod events;
mod fetch;
mod grid;
mod items;
mod keyboard;
mod landing;
mod library;
mod packer;
mod picture;
mod relief;
mod reliefjobs;
mod reliefshow;
mod reliefwork;
mod rows;
mod scene;
mod scenecode;
mod scenecopy;
mod scenedoc;
mod scenefile;
mod scenehear;
mod scenelive;
mod scenemedia;
mod scenerender;
mod scenesettle;
mod sceneshow;
mod scenesweep;
mod scenewave;
mod scenework;
mod secret;
mod shadercheck;
mod shards;
mod shelf;
mod signing;
mod stall;
mod stallcheck;
mod stallwork;
mod strips;
#[cfg(test)]
mod tests;
mod tr;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QQuickStyle, QString, QUrl};
use std::path::Path;
use std::process::ExitCode;
use wprs_core::dbus::APP_ID;

const MAIN: &str = "qrc:/qt/qml/io/github/wallpaper_rs/app/qml/Main.qml";
const STYLE: &str = "org.kde.desktop";

fn main() -> ExitCode {
    wprs_scene::serve_check();
    wprs_scene::check_out_of_process(true);
    wprs_core::stopped_effects().inspect(|p| wprs_scene::keep_strikes(p));
    let code = start();
    bus::part(std::time::Duration::from_secs(2));
    wprs_scene::end_trials();
    wprs_render::finish_builds(if code == ExitCode::SUCCESS { 0 } else { 1 });
    code
}

fn start() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("--version" | "-V") => println!("wprs-app {}", wprs_core::VERSION),
        Some("--help" | "-h") => {
            println!("usage: wprs-app [--check | --version | --page PAGE] [FILE | URI]...");
            println!("pages: {}", app::PAGES.join(", "));
        }
        Some("--check") if !bus::checking() => return checkrun::run(),
        Some("--depth") => return depthproc::child(&args[1..]),
        Some("--page") => match app::page(args.get(1).map(String::as_str)) {
            Some(page) => return open(&args[2..], Some(page)),
            None => {
                eprintln!("wprs-app: --page takes one of {}", app::PAGES.join(", "));
                return ExitCode::from(2);
            }
        },
        _ => return open(&args, None),
    }
    ExitCode::SUCCESS
}

fn open(args: &[String], page: Option<&str>) -> ExitCode {
    app::start(args.iter().filter_map(|a| file(a)).collect(), page).unwrap_or_else(run)
}

fn file(arg: &str) -> Option<String> {
    if arg.contains("://") {
        return Some(arg.to_owned());
    }
    let path = std::path::absolute(arg).ok().filter(|p| p.is_file())?;
    Some(bus::url(path.to_str()?))
}

fn run() -> ExitCode {
    let mut app = QGuiApplication::new();
    QGuiApplication::set_desktop_file_name(&QString::from(APP_ID));
    let mut engine = QQmlApplicationEngine::new();
    let (Some(mut app), Some(mut engine)) = (app.as_mut(), engine.as_mut()) else { return ExitCode::FAILURE };
    app.as_mut().set_application_name(&QString::from("wallpaper-rs"));
    engine.as_mut().add_import_path(&QString::from("qrc:/qt/qml"));
    let paths = engine.import_path_list();
    let style = paths.iter().any(|p| Path::new(&p.to_string()).join(STYLE.replace('.', "/")).is_dir());
    if style && std::env::var_os("QT_QUICK_CONTROLS_STYLE").is_none() {
        QQuickStyle::set_style(&QString::from(STYLE));
    }
    if bus::checking() {
        engine.as_mut().on_object_creation_failed(|_, _| std::process::exit(1)).release();
    }
    engine.load(&QUrl::from(MAIN));
    ExitCode::from(app.exec() as u8)
}
