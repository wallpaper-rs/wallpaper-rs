use crate::cli::Command;
use crate::inputs::Inputs;
use crate::library::Lib;
use crate::playlists::Playlist;
use crate::profiles::Profile;
use crate::told::{fill, told, tr};

fn on(output: &str) -> String {
    match output.is_empty() {
        true => tr(told("every screen")),
        false => output.to_owned(),
    }
}

fn count(items: &[String]) -> usize {
    items.len()
}

fn items(command: &Lib) -> Option<String> {
    Some(match command {
        Lib::Title { title, .. } => fill(told("title set to %1"), &[&title]),
        Lib::Rate { rating, items } => fill(told("rating %1 given to %2 items"), &[rating, &count(items)]),
        Lib::Fav { items, off: false } => fill(told("%1 items marked as favorites"), &[&count(items)]),
        Lib::Fav { items, off: true } => fill(told("%1 items are no longer favorites"), &[&count(items)]),
        Lib::Tag { items, add, remove } => fill(
            told("tags changed on %1 items: %2 added, %3 removed"),
            &[&count(items), &add.len(), &remove.len()],
        ),
        Lib::Retag { from, to } => fill(told("tag %1 renamed to %2"), &[from, to]),
        Lib::Rm { items, .. } => fill(told("%1 items removed from the library"), &[&count(items)]),
        Lib::Confirm { what, items } => fill(told("%1 confirmed for %2 items"), &[what, &count(items)]),
        Lib::Unconfirm { what, items } => {
            fill(told("%1 no longer confirmed for %2 items"), &[what, &count(items)])
        }
        Lib::ClearData { items } => fill(told("the data of %1 items was cleared"), &[&count(items)]),
        _ => return None,
    })
}

fn lists(command: &Playlist) -> Option<String> {
    Some(match command {
        Playlist::Rename { playlist, name } => fill(told("playlist %1 renamed to %2"), &[playlist, name]),
        Playlist::Query { playlist, query } => {
            fill(told("playlist %1 now follows the query %2"), &[playlist, query])
        }
        Playlist::Rm { playlist } => fill(told("playlist %1 deleted"), &[playlist]),
        Playlist::Add { playlist, items, folder: None, .. } => {
            fill(told("%1 items added to playlist %2"), &[&count(items), playlist])
        }
        Playlist::Add { playlist, folder: Some(folder), .. } => {
            fill(told("folder %1 added to playlist %2"), &[folder, playlist])
        }
        Playlist::Remove { playlist, items } => {
            fill(told("%1 items removed from playlist %2"), &[&count(items), playlist])
        }
        Playlist::RemoveAt { playlist, indexes } => {
            fill(told("%1 entries removed from playlist %2"), &[&indexes.len(), playlist])
        }
        Playlist::Move { playlist, from, to } => {
            fill(told("entry %1 of playlist %2 moved to %3"), &[from, playlist, to])
        }
        Playlist::Rule { playlist, key: Some(key), value: Some(value), .. } => {
            fill(told("rule %1 of playlist %2 set to %3"), &[key, playlist, value])
        }
        Playlist::Stop(out) => fill(told("playlist stopped on %1"), &[&on(&out.output)]),
        _ => return None,
    })
}

fn profiles(command: &Profile) -> Option<String> {
    Some(match command {
        Profile::Use { profile } => fill(told("profile %1 is active"), &[profile]),
        Profile::Off => tr(told("no profile is active")),
        Profile::Rename { profile, name } => fill(told("profile %1 renamed to %2"), &[profile, name]),
        Profile::Rm { profile } => fill(told("profile %1 deleted"), &[profile]),
        Profile::Auto { profile, key: Some(key), value: Some(value), .. } => {
            fill(told("rule %1 of profile %2 set to %3"), &[key, profile, value])
        }
        _ => return None,
    })
}

fn inputs(command: &Inputs) -> Option<String> {
    Some(match command {
        Inputs::Revoke { scene, input } => fill(told("%1 can no longer use %2"), &[scene, &input.name()]),
        Inputs::Clicks { state, output } => {
            let state = tr(if matches!(state, crate::inputs::Switch::On) { told("on") } else { told("off") });
            fill(told("clicks on %1: %2"), &[&on(output.as_deref().unwrap_or_default()), &state])
        }
        Inputs::Pointer { x, y } => fill(told("pointer reported at %1,%2"), &[x, y]),
        _ => return None,
    })
}

pub fn said(command: &Command) -> Option<String> {
    Some(match command {
        Command::Set { path, output, .. } => fill(told("wallpaper set: %1 on %2"), &[path, &on(output)]),
        Command::Clear(out) => fill(told("wallpaper cleared on %1"), &[&on(&out.output)]),
        Command::Restore(out) => fill(told("previous wallpaper restored on %1"), &[&on(&out.output)]),
        Command::Fit { fit, output } => fill(told("fit %1 on %2"), &[&fit.as_str(), &on(output)]),
        Command::Background { color, output } => fill(told("background %1 on %2"), &[color, &on(output)]),
        Command::Config { key: Some(key), value: Some(value), .. } => {
            fill(told("%1 set to %2"), &[key, value])
        }
        Command::Reload => tr(told("settings reloaded")),
        Command::RetryEffects => tr(told("effects tried again")),
        Command::Pause => tr(told("paused")),
        Command::Resume => tr(told("playing again")),
        Command::Volume { percent } => fill(told("volume %1 %"), &[percent]),
        Command::Mute => tr(told("muted")),
        Command::Unmute => tr(told("sound on")),
        Command::Covered(cover) => {
            let state = tr(if cover.covered { told("covered") } else { told("not covered") });
            fill(told("%1 marked as %2"), &[&cover.output, &state])
        }
        Command::Release { output } => fill(told("stream released on %1"), &[&on(output)]),
        Command::Cancel { id } => fill(told("job %1 cancelled"), &[id]),
        Command::Lib(command) => return items(command),
        Command::Playlist(command) => return lists(command),
        Command::Profile(command) => return profiles(command),
        Command::Input(command) => return inputs(command),
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::said;
    use crate::Cli;
    use clap::Parser;

    fn line(args: &[&str]) -> Option<String> {
        let argv = std::iter::once("wprs-ctl").chain(args.iter().copied());
        said(&Cli::try_parse_from(argv).unwrap().command)
    }

    #[test]
    fn commands_that_act_say_what_they_did() {
        assert_eq!(line(&["mute"]).as_deref(), Some("muted"));
        assert_eq!(line(&["covered", "DP-1", "true"]).as_deref(), Some("DP-1 marked as covered"));
        assert_eq!(line(&["set", "/a.png", "-o", "DP-1"]).as_deref(), Some("wallpaper set: /a.png on DP-1"));
        assert_eq!(
            line(&["config", "playback.volume", "0.5"]).as_deref(),
            Some("playback.volume set to 0.5")
        );
        assert_eq!(
            line(&["input", "revoke", "org.example.a", "audio"]).as_deref(),
            Some("org.example.a can no longer use audio")
        );
        assert_eq!(line(&["lib", "rate", "3", "abcdef"]).as_deref(), Some("rating 3 given to 1 items"));
        assert_eq!(line(&["playlist", "rm", "Sea"]).as_deref(), Some("playlist Sea deleted"));
        assert_eq!(line(&["profile", "off"]).as_deref(), Some("no profile is active"));
    }

    #[test]
    fn commands_that_print_their_own_result_or_read_stay_as_they_are() {
        assert_eq!(line(&["state"]), None);
        assert_eq!(line(&["lib", "sources"]), None);
        assert_eq!(line(&["config"]), None);
    }
}
