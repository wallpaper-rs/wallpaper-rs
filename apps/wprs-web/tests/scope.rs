use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use wprs_web::{Isolation, scope, spawn, unscoped};

const FAKE: &str = "#!/bin/sh\nprintf '%s\\n' \"$@\" > \"$(dirname \"$0\")/seen\"\nwhile [ \"$1\" != -- ]; do shift; done\nshift\nexec \"$@\"\n";

fn run(limits: bool) -> std::process::ExitStatus {
    let isolation = Isolation { limits, ..Isolation::default() };
    let (_link, mut child) = spawn(Path::new("/bin/sh"), isolation, |c| {
        c.args(["-c", "exit 0"]);
    })
    .unwrap();
    child.wait().unwrap()
}

#[test]
fn limits_wrap_the_helper_in_a_user_scope_only_where_a_user_manager_runs() {
    let dir = tempfile::tempdir().unwrap();
    let bin = dir.path().join("bin");
    let runtime = dir.path().join("run");
    std::fs::create_dir_all(&bin).unwrap();
    std::fs::create_dir_all(runtime.join("systemd")).unwrap();
    let fake = bin.join("systemd-run");
    std::fs::write(&fake, FAKE).unwrap();
    std::fs::set_permissions(&fake, std::fs::Permissions::from_mode(0o755)).unwrap();
    unsafe {
        std::env::set_var("PATH", format!("{}:/usr/bin:/bin", bin.display()));
        std::env::set_var("XDG_RUNTIME_DIR", &runtime);
    }
    assert_eq!(scope(), None, "no user manager socket yet");
    std::fs::write(runtime.join("systemd/private"), "").unwrap();
    assert_eq!(scope(), Some(fake.clone()));

    assert!(run(true).success());
    let seen = std::fs::read_to_string(bin.join("seen")).unwrap();
    let args: Vec<&str> = seen.lines().collect();
    assert_eq!(
        args,
        [
            "--user",
            "--scope",
            "--quiet",
            "--collect",
            "-p",
            "CPUQuota=100%",
            "-p",
            "CPUWeight=20",
            "-p",
            "MemoryMax=1536M",
            "-p",
            "MemorySwapMax=0",
            "-p",
            "TasksMax=512",
            "--",
            "/bin/sh",
            "-c",
            "exit 0"
        ]
    );

    std::fs::remove_file(bin.join("seen")).unwrap();
    assert!(run(false).success());
    assert!(!bin.join("seen").exists(), "no limits asked, no scope");

    unscoped();
    assert_eq!(scope(), None);
    assert!(run(true).success());
    assert!(!bin.join("seen").exists(), "a page told to stay unscoped runs plain");
}
