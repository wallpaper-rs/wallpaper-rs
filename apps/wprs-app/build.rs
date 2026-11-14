use cxx_qt_build::{CxxQtBuilder, QmlFile, QmlModule};
use std::path::PathBuf;
use std::{env, fs};

#[path = "qt-shim/setup.rs"]
mod shim;

#[path = "look/resources.rs"]
mod look;

fn files(dir: &str) -> Vec<PathBuf> {
    let mut files: Vec<_> = fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|e| e == "qml"))
        .collect();
    files.sort();
    files
}

fn main() {
    let mut langs: Vec<_> = fs::read_dir("../../po").unwrap().map(|e| e.unwrap().path()).collect();
    langs.sort();
    let catalogs: String =
        (langs.iter().map(|dir| (dir, dir.join("wprs-app.po"))).filter(|(_, po)| po.is_file()))
            .map(|(dir, po)| {
                format!("({:?}, include_str!({:?})),", dir.file_name().unwrap(), po.canonicalize().unwrap())
            })
            .collect();
    fs::write(PathBuf::from(env::var("OUT_DIR").unwrap()).join("catalogs.rs"), format!("&[{catalogs}]"))
        .unwrap();
    println!("cargo::rerun-if-changed=../../po");
    println!("cargo::rerun-if-changed=qml");
    println!("cargo::rerun-if-changed=starters");
    shim::install();
    let look = look::write(
        &PathBuf::from("look").canonicalize().unwrap(),
        &PathBuf::from(env::var("OUT_DIR").unwrap()),
    );
    let pages = files("qml").into_iter().map(QmlFile::from);
    let singletons = files("qml/facade").into_iter().map(|p| QmlFile::from(p).singleton(true));
    CxxQtBuilder::new_qml_module(
        QmlModule::new("io.github.wallpaper_rs.app").qml_files(pages.chain(singletons)),
    )
    .qrc(look)
    .qrc("starters/starters.qrc")
    .files([
        "src/backend.rs",
        "src/connectcode.rs",
        "src/depth.rs",
        "src/desknote.rs",
        "src/editor.rs",
        "src/items.rs",
        "src/keyboard.rs",
        "src/looks.rs",
        "src/packer.rs",
        "src/relief.rs",
        "src/scene.rs",
        "src/scenerecent.rs",
        "src/shadercheck.rs",
        "src/shelf.rs",
        "src/signing.rs",
        "src/strips.rs",
        "src/stall.rs",
    ])
    .build();
}
