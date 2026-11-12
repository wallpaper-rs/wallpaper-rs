use crate::show::{short, table};
use wprs_core::model::{Away, AwayReply};

const WHERE: &str =
    "exists only on another device; publishing it to the server or to a repository makes it available here";

fn name(away: &Away) -> String {
    if away.title.is_empty() { short(&away.key) } else { away.title.clone() }
}

pub fn note(title: &str) -> String {
    format!("{title} {WHERE}")
}

pub fn notes(away: &[Away]) -> String {
    away.iter().map(|a| format!("\n{}\n", note(&name(a)))).collect()
}

pub fn listing(reply: AwayReply) -> String {
    let join = |list: Vec<String>| if list.is_empty() { "-".to_owned() } else { list.join(",") };
    let rows: Vec<Vec<String>> = reply
        .items
        .iter()
        .map(|a| {
            let playlists = a.playlists.iter().map(|p| short(p)).collect();
            let screens = a.profiles.iter().map(|p| p.output.clone()).collect();
            vec![short(&a.key), name(a), join(playlists), join(screens)]
        })
        .collect();
    let empty = rows.is_empty();
    let text = table(&["RECORD", "TITLE", "PLAYLISTS", "SCREENS"], rows);
    if empty { text } else { text + &format!("\nThese items {WHERE}.\n") }
}
