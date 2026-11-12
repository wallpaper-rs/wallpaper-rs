use super::launch::{entry, valid_id, words};
use super::{Target, target};
use serde_json::json;

#[test]
fn shortcut_values_name_https_addresses_absolute_files_and_folders_or_application_ids() {
    let cases = [
        (json!({ "uri": "https://example.org/a" }), Some(Target::Uri("https://example.org/a".into()))),
        (json!({ "file": "/srv/notes.txt" }), Some(Target::File("/srv/notes.txt".into()))),
        (json!({ "folder": "/srv/photos" }), Some(Target::Folder("/srv/photos".into()))),
        (json!({ "app": "org.kde.dolphin" }), Some(Target::App("org.kde.dolphin".into()))),
        (json!({ "uri": "http://example.org" }), None),
        (json!({ "uri": "file:///etc/passwd" }), None),
        (json!({ "file": "notes.txt" }), None),
        (json!({ "folder": "/srv/../etc" }), None),
        (json!({ "app": "../evil" }), None),
        (json!({ "app": "a b" }), None),
        (json!({ "app": "x", "uri": "https://example.org" }), None),
        (json!("https://example.org"), None),
        (json!(null), None),
    ];
    for (value, want) in cases {
        assert_eq!(target(&value), want, "{value}");
    }
    assert!(
        valid_id("org.gnome.Nautilus") && !valid_id("") && !valid_id("-x") && !valid_id(&"a".repeat(256))
    );
}

#[test]
fn desktop_entries_give_their_program_without_field_codes_or_a_shell() {
    let text = "[Desktop Entry]\nType=Application\nExec=\"/opt/My App/run\" --new-window %U\nName=App\n[Desktop Action x]\nExec=other\n";
    let found = entry(text);
    assert_eq!((found["Type"], found["Name"]), ("Application", "App"));
    assert_eq!(words(found["Exec"]), ["/opt/My App/run", "--new-window"]);
    assert_eq!(words("sh -c \"echo \\\"hi\\\"\" 100%%"), ["sh", "-c", "echo \"hi\"", "100%"]);
    assert_eq!(words("app %x"), Vec::<String>::new(), "an unknown field code makes the entry unusable");
    assert_eq!(words("a  b\t\"\""), ["a", "b", ""]);
}

#[test]
fn a_target_is_checked_as_it_is_typed() {
    use super::apps::check;
    let dir = std::env::temp_dir();
    let folder = dir.to_string_lossy().into_owned();
    let accepted = [json!(null), json!({ "uri": "https://example.org/a?b=1" }), json!({ "folder": folder })];
    for value in accepted {
        assert_eq!(check(&value), Ok(()), "{value}");
    }
    let refused = [
        json!({ "uri": "http://example.org" }),
        json!({ "uri": "https://" }),
        json!({ "uri": "https:///path" }),
        json!({ "uri": "https://exa mple.org" }),
        json!({ "uri": "https://example.org/\u{7}" }),
        json!({ "file": "/no/such/file.txt" }),
        json!({ "folder": "/no/such/folder" }),
        json!({ "file": "relative.txt" }),
        json!({ "app": "no.such.Application.Anywhere" }),
        json!({ "app": "a b" }),
        json!("https://example.org"),
        json!({ "uri": "https://a.org", "file": "/x" }),
    ];
    for value in refused {
        assert!(check(&value).is_err(), "{value} was accepted");
    }
    assert!(check(&json!({ "uri": "http://example.org" })).unwrap_err().contains("https://"));
}

#[test]
fn installed_applications_come_from_the_data_folders_without_hidden_or_terminal_ones() {
    let root = tempfile::tempdir().unwrap();
    let apps = root.path().join("applications");
    std::fs::create_dir_all(&apps).unwrap();
    let write = |name: &str, text: &str| std::fs::write(apps.join(name), text).unwrap();
    write("org.example.Zed.desktop", "[Desktop Entry]\nType=Application\nName=Zed\nIcon=zed\nExec=zed\n");
    write("org.example.Alpha.desktop", "[Desktop Entry]\nType=Application\nName=alpha\nExec=alpha\n");
    write("org.example.Hidden.desktop", "[Desktop Entry]\nType=Application\nHidden=true\nExec=x\n");
    write("org.example.Quiet.desktop", "[Desktop Entry]\nType=Application\nNoDisplay=true\nExec=x\n");
    write("org.example.Shell.desktop", "[Desktop Entry]\nType=Application\nTerminal=true\nExec=x\n");
    write("org.example.Link.desktop", "[Desktop Entry]\nType=Link\nURL=https://example.org\n");
    write("not an id.desktop", "[Desktop Entry]\nType=Application\nName=Spaced\nExec=x\n");
    write("README.txt", "x");
    let found = super::apps::listed_in(&[apps]);
    let ids: Vec<&str> = found.iter().map(|a| a.id.as_str()).collect();
    assert_eq!(ids, ["org.example.Alpha", "org.example.Zed"]);
    assert_eq!((found[1].name.as_str(), found[1].icon.as_str()), ("Zed", "zed"));
    assert_eq!(found[0].icon, "");
}
