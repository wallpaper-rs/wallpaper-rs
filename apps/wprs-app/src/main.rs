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
mod connectcode;
mod covers;
mod depth;
mod depthproc;
mod depthview;
mod depthwork;
mod desknote;
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
mod looks;
mod packer;
mod picture;
mod preview;
mod relief;
mod reliefjobs;
mod reliefland;
mod reliefshow;
mod reliefwork;
mod rows;
mod scene;
mod scenecode;
mod scenecopy;
mod scenecross;
mod scenedoc;
mod scenefile;
mod scenehear;
mod scenelive;
mod scenemedia;
mod scenerecent;
mod scenerender;
mod scenesettle;
mod sceneshow;
mod scenesplit;
mod scenesweep;
mod scenewave;
mod scenework;
mod secret;
mod service;
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
mod tones;
mod tr;

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QString, QUrl};
use std::process::ExitCode;
use wprs_core::dbus::launcher;

const MAIN: &str = "qrc:/qt/qml/io/github/wallpaper_rs/app/qml/Main.qml";
const PIECES: &str = "qrc:/qt/qml/io/github/wallpaper_rs/app/qml/Pieces.qml";

fn main() -> ExitCode {
    wprs_render::serve_script_host();
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
            println!("usage: wprs-app [--check | --version | --page PAGE | --browse OUTPUT] [FILE | URI]...");
            println!("pages: {}", app::PAGES.join(", "));
        }
        Some("--check") if !bus::checking() => return checkrun::run(),
        Some("--pieces") => return show(PIECES),
        Some("--depth") => return depthproc::child(&args[1..]),
        Some("--browse") => return preview::start(),
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
    show(MAIN)
}

fn show(url: &str) -> ExitCode {
    ExitCode::from(engine(url))
}

fn engine(url: &str) -> u8 {
    let mut app = QGuiApplication::new();
    QGuiApplication::set_desktop_file_name(&QString::from(
        launcher(std::env::var("FLATPAK_ID").ok()).as_str(),
    ));
    let mut engine = QQmlApplicationEngine::new();
    let (Some(mut app), Some(mut engine)) = (app.as_mut(), engine.as_mut()) else { return 1 };
    app.as_mut().set_application_name(&QString::from("wallpaper-rs"));
    engine.as_mut().add_import_path(&QString::from("qrc:/qt/qml"));
    looks::install();
    if bus::checking() && shards::claim("compiled") {
        looks::qobject::compile_qml();
    }
    if bus::checking() || url == preview::PREVIEW {
        engine.as_mut().on_object_creation_failed(|_, _| std::process::exit(1)).release();
    }
    engine.load(&QUrl::from(url));
    app.exec() as u8
}
