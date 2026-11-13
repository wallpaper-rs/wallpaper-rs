use std::collections::BTreeMap;
use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};

const WEIGHTS: [&str; 3] = ["regular", "fill", "bold"];
const THEME: &str = "[Icon Theme]\nName=wprs\nComment=wallpaper-rs\nDirectories=scalable\n\n[scalable]\nSize=24\nMinSize=8\nMaxSize=512\nType=Scalable\n";

fn tone(look: &str, name: &str) -> String {
    let key = format!("property color {name}: \"");
    let at = look.find(&key).unwrap_or_else(|| panic!("Look.qml has no {name}")) + key.len();
    look[at..at + 7].to_owned()
}

fn glyphs(here: &Path) -> BTreeMap<String, PathBuf> {
    let mut found = BTreeMap::new();
    for weight in WEIGHTS {
        let dir = here.join("icons").join(weight);
        for entry in fs::read_dir(&dir).unwrap().map(|e| e.unwrap().path()) {
            let stem = entry.file_stem().unwrap().to_str().unwrap().to_owned();
            let name = if weight == "regular" { stem } else { format!("{stem}-{weight}") };
            found.insert(name, entry);
        }
    }
    found
}

fn file(qrc: &mut String, alias: &str, path: &Path) {
    writeln!(qrc, "<file alias=\"{alias}\">{}</file>", path.display()).unwrap();
}

pub fn write(here: &Path, out: &Path) -> PathBuf {
    println!("cargo::rerun-if-changed={}", here.display());
    let ink = tone(&fs::read_to_string(here.join("../qml/facade/Look.qml")).unwrap(), "escarcha");
    let glyphs = glyphs(here);
    let drawn = out.join("look-icons");
    fs::create_dir_all(&drawn).unwrap();
    let mut names: BTreeMap<String, PathBuf> = BTreeMap::new();
    for (name, source) in &glyphs {
        let target = drawn.join(format!("{name}.svg"));
        fs::write(&target, fs::read_to_string(source).unwrap().replace("currentColor", &ink)).unwrap();
        names.insert(name.clone(), target);
    }
    for line in fs::read_to_string(here.join("icons/names.txt")).unwrap().lines() {
        let (old, new) = line.split_once(' ').unwrap_or_else(|| panic!("bad icon line {line}"));
        let target = names
            .get(new)
            .unwrap_or_else(|| panic!("icon {old} maps to {new}, which is not bundled"))
            .clone();
        if let Some(had) = names.insert(old.to_owned(), target.clone()) {
            assert!(had == target, "icon {old} is also a bundled name with another drawing");
        }
    }
    let theme = out.join("look-index.theme");
    fs::write(&theme, THEME).unwrap();
    let mut qrc = String::from("<!DOCTYPE RCC><RCC version=\"1.0\">\n<qresource prefix=\"/icons/wprs\">\n");
    file(&mut qrc, "index.theme", &theme);
    names.iter().for_each(|(name, path)| file(&mut qrc, &format!("scalable/{name}.svg"), path));
    qrc.push_str("</qresource>\n<qresource prefix=\"/wprs/fonts\">\n");
    for font in ["Onest.ttf", "RedHatDisplay.ttf"] {
        file(&mut qrc, font, &here.join("fonts").join(font).canonicalize().unwrap());
    }
    qrc.push_str("</qresource>\n<qresource prefix=\"/wprs/pictures\">\n");
    file(&mut qrc, "example.jpg", &here.join("pictures/example.jpg").canonicalize().unwrap());
    qrc.push_str("</qresource>\n<qresource prefix=\"/qt/qml/io/github/wallpaper_rs/style\">\n");
    let mut style: Vec<_> = fs::read_dir(here.join("style")).unwrap().map(|e| e.unwrap().path()).collect();
    style.sort();
    for path in style {
        file(&mut qrc, path.file_name().unwrap().to_str().unwrap(), &path.canonicalize().unwrap());
    }
    qrc.push_str("</qresource>\n</RCC>\n");
    let path = out.join("look.qrc");
    fs::write(&path, qrc).unwrap();
    path
}
