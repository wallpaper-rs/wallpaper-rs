use std::time::{Duration, Instant};

extern "C" fn hang() {
    std::thread::sleep(Duration::from_secs(600));
}

#[test]
fn a_fatal_error_exits_at_once_even_when_exit_handlers_hang() {
    if std::env::var_os("WPRS_FATAL_CHILD").is_some() {
        unsafe { libc::atexit(hang) };
        super::fatal("a test failure");
    }
    let name = "tests::a_fatal_error_exits_at_once_even_when_exit_handlers_hang";
    let mut child = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", name, "--nocapture"])
        .env("WPRS_FATAL_CHILD", "1")
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(20);
    let status = loop {
        match child.try_wait().unwrap() {
            Some(status) => break status,
            None if Instant::now() > deadline => {
                child.kill().unwrap();
                panic!("the daemon hung while exiting");
            }
            None => std::thread::sleep(Duration::from_millis(20)),
        }
    };
    assert_eq!(status.code(), Some(70));
}
