use clap::Parser;
use wprs_ctl::Cli;

fn parsed(line: &str) -> String {
    let args: Vec<&str> = line.split(' ').collect();
    let cli = Cli::try_parse_from(["wprs-ctl"].iter().chain(&args)).unwrap_or_else(|e| panic!("{line}: {e}"));
    format!("{:?}", cli.command)
}

#[test]
fn the_sound_of_a_screen_and_the_folders_and_rotations_of_playlists_parse() {
    let cases = [
        ("sound", r#"Sound(Hearing { state: None, volume: None, output: "" })"#),
        ("sound off -o DP-1", r#"Sound(Hearing { state: Some("off"), volume: None, output: "DP-1" })"#),
        ("sound --volume 30", r#"Sound(Hearing { state: None, volume: Some(30), output: "" })"#),
        (
            "lib apply R1AB2C -o DP-1 --sound off --volume 30",
            r#"Lib(Apply(Applying { item: "R1AB2C", output: "DP-1", fit: None, span: false, preset: "", sound: Some("off"), volume: Some(30) }))"#,
        ),
        (
            "playlist add Sea --folder /pics/sea --follow",
            r#"Playlist(Add { playlist: "Sea", items: [], folder: Some("/pics/sea"), follow: true })"#,
        ),
        (
            "playlist add Sea R1AB2C X9Y8Z7",
            r#"Playlist(Add { playlist: "Sea", items: ["R1AB2C", "X9Y8Z7"], folder: None, follow: false })"#,
        ),
        (
            "playlist rotate R1AB2C X9Y8Z7 --every 10m --name Spin -o DP-1",
            r#"Playlist(Rotate { items: ["R1AB2C", "X9Y8Z7"], folder: None, follow: false, every: "10m", name: Some("Spin"), out: Out { output: "DP-1" } })"#,
        ),
        (
            "playlist rotate --folder /pics --follow --every 1h",
            r#"Playlist(Rotate { items: [], folder: Some("/pics"), follow: true, every: "1h", name: None, out: Out { output: "" } })"#,
        ),
    ];
    for (line, debug) in cases {
        assert_eq!(parsed(line), debug, "{line}");
    }
}

#[test]
fn bad_sound_words_and_a_follow_without_a_folder_are_refused() {
    let refused = |line: &str| {
        let args: Vec<&str> = line.split(' ').collect();
        Cli::try_parse_from(["wprs-ctl"].iter().chain(&args)).is_err()
    };
    assert!(refused("sound loud"), "a sound word that is not on, off or follow was taken");
    assert!(refused("sound --volume 101"));
    assert!(refused("playlist add Sea R1AB2C --follow"));
    assert!(refused("playlist add Sea"), "an empty add was taken");
    assert!(refused("playlist rotate R1AB2C"), "a rotation without time was taken");
}
