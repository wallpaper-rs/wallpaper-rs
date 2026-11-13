use std::process::Command;

const FP: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

fn run(args: &[&str]) -> (bool, String, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_wprs-ctl"))
        .arg("connect-code")
        .args(args)
        .env("WPRS_PRIVATE_BUS_ONLY", "1")
        .env_remove("DBUS_SESSION_BUS_ADDRESS")
        .output()
        .unwrap();
    (
        out.status.success(),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

#[test]
fn prints_the_text_and_draws_on_request() {
    let (ok, text, _) = run(&["https://Wall.example:8443", "--fingerprint", FP, "--invite", "abcdefgh"]);
    assert!(ok);
    assert_eq!(text, format!("wprs://connect/1?server=https://wall.example:8443&fp={FP}&invite=abcdefgh\n"));
    let (ok, plain, _) = run(&["https://wall.example"]);
    assert!(ok && plain == "wprs://connect/1?server=https://wall.example\n");
    let (ok, drawn, _) = run(&["https://wall.example", "--fingerprint", FP, "--draw"]);
    assert!(ok && drawn.lines().count() > 10 && drawn.contains('▀'));
}

#[test]
fn refuses_what_a_phone_could_not_read() {
    let (ok, text, error) = run(&["http://wall.example", "--fingerprint", FP]);
    assert!(!ok && text.is_empty() && error.contains("https"));
    let (ok, ..) = run(&["https://wall.example", "--fingerprint", "abc"]);
    assert!(!ok);
}
