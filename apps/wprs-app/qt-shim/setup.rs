use std::os::unix::fs::{PermissionsExt, symlink};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

fn query(qmake: &Path, name: &str) -> Option<PathBuf> {
    let out = Command::new(qmake).args(["-query", name]).env_clear().output().ok()?;
    let text = String::from_utf8(out.stdout).ok()?;
    (!text.trim().is_empty()).then(|| PathBuf::from(text.trim()))
}

fn find_qmake() -> Option<PathBuf> {
    if let Some(qmake) = env::var_os("QMAKE") {
        return Some(PathBuf::from(qmake));
    }
    let dirs: Vec<_> = env::split_paths(&env::var_os("PATH")?).collect();
    ["qmake6", "qmake-qt6", "qmake"]
        .iter()
        .flat_map(|n| dirs.iter().map(move |d| d.join(n)))
        .find(|p| p.is_file())
}

fn install_script(source: &str, target: &Path) -> Option<()> {
    fs::write(target, source).ok()?;
    fs::set_permissions(target, fs::Permissions::from_mode(0o755)).ok()
}

fn prepare(dir: &Path, aot: bool) -> Option<PathBuf> {
    let qmake = find_qmake()?;
    let libexecs = query(&qmake, "QT_HOST_LIBEXECS").or_else(|| query(&qmake, "QT_INSTALL_LIBEXECS"))?;
    let bins = query(&qmake, "QT_HOST_BINS").or_else(|| query(&qmake, "QT_INSTALL_BINS"));
    let real_cache = [Some(libexecs.clone()), bins]
        .into_iter()
        .flatten()
        .map(|d| d.join("qmlcachegen"))
        .find(|p| p.is_file())?;
    let _ = fs::remove_dir_all(dir);
    let libexec = dir.join("libexec");
    fs::create_dir_all(&libexec).ok()?;
    fs::create_dir_all(dir.join("run")).ok()?;
    for entry in fs::read_dir(&libexecs).ok()?.flatten() {
        if entry.file_name() != "qmlcachegen" {
            let _ = symlink(entry.path(), libexec.join(entry.file_name()));
        }
    }
    install_script(include_str!("qmlcachegen.sh"), &libexec.join("qmlcachegen"))?;
    let shim = dir.join("qmake");
    install_script(include_str!("qmake.sh"), &shim)?;
    fs::write(dir.join("real-qmake"), qmake.to_string_lossy().as_bytes()).ok()?;
    fs::write(dir.join("real-qmlcachegen"), real_cache.to_string_lossy().as_bytes()).ok()?;
    fs::write(dir.join("mode"), if aot { "aot" } else { "check" }).ok()?;
    Some(shim)
}

pub fn install() {
    println!("cargo::rerun-if-env-changed=QMAKE");
    println!("cargo::rerun-if-env-changed=WPRS_QML_AOT");
    println!("cargo::rerun-if-changed=qt-shim");
    let aot = match env::var("WPRS_QML_AOT").as_deref() {
        Ok("1") => true,
        Ok("0") => false,
        _ => env::var("PROFILE").is_ok_and(|p| p == "release"),
    };
    let dir = PathBuf::from(env::var("OUT_DIR").unwrap()).join("qt-shim");
    if let Some(shim) = prepare(&dir, aot) {
        unsafe { env::set_var("QMAKE", shim) };
    }
}
