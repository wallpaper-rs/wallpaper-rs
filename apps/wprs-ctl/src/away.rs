use crate::show::{short, table};
use crate::told::{fill, told, tr};
use wprs_core::model::{Away, AwayReply};

const WHERE: &str = told(
    "exists only on another device; publishing it to the server or to a repository makes it available here",
);

fn name(away: &Away) -> String {
    if away.title.is_empty() { short(&away.key) } else { away.title.clone() }
}

pub fn note(title: &str) -> String {
    format!("{title} {}", tr(WHERE))
}

const REPOSITORY: &str = told(
    "comes from the repository %1, which is not added on this device; adding the repository makes it available here",
);

fn line(away: &Away) -> String {
    match &away.repo {
        Some(repo) => format!("{} {}", name(away), fill(REPOSITORY, &[repo])),
        None => note(&name(away)),
    }
}

pub fn notes(away: &[Away]) -> String {
    away.iter().map(|a| format!("\n{}\n", line(a))).collect()
}

pub fn listing(reply: AwayReply) -> String {
    let join = |list: Vec<String>| if list.is_empty() { "-".to_owned() } else { list.join(",") };
    let rows: Vec<Vec<String>> = reply
        .items
        .iter()
        .map(|a| {
            let playlists = a.playlists.iter().map(|p| short(p)).collect();
            let screens = a.profiles.iter().map(|p| p.output.clone()).collect();
            let repo = a.repo.clone().unwrap_or_else(|| "-".to_owned());
            vec![short(&a.key), name(a), repo, join(playlists), join(screens)]
        })
        .collect();
    let empty = rows.is_empty();
    let text = table(&["RECORD", "TITLE", "REPOSITORY", "PLAYLISTS", "SCREENS"], rows);
    if empty { text } else { text + &fill(told("\nThese items %1.\n"), &[&tr(WHERE)]) }
}
