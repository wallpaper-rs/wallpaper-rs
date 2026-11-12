use crate::author::Author;
use crate::capturing::Capture;
use anyhow::{Result, ensure};
use serde_json::{Value, json};
use std::fs;
use std::path::{Path, PathBuf};
use wprs_editor::actions::{
    Action, MaskStroke, ParticleEdit, ScriptFile, ShotEdit, SowEdit, TemplateEdit, UpgradeEdit,
};
use wprs_editor::{Op, Project, references};

pub(crate) const NEW: &str = "Create a package folder from a starting point: the scenes empty, particles, parallax or clock, web for a web page or preset for the values of another package";
pub(crate) const OPEN: &str = "Describe a scene folder, or first extract a .wpak scene package into it as an unsigned copy with the id <id>.copy";
pub(crate) const SCRIPT: &str = "Copy a JavaScript module into the scripts, or make it from a template, and declare the events and parts it uses";
pub(crate) const TEMPLATE: &str = "Make the module from a template instead of copying FILE: empty, media (handles every media player event and exports nowPlaying, accentColor, shown and progress for values to follow), touch (its update returns 1 while the pointer is over the output and exports position and pressed), timer (counts seconds with setInterval and exports seconds), audio (exports level, bass and beat), spawn (draws copies of a template of the scene, with settings), wave (exports a swinging value, with settings), daytime (exports daylight and hour), setting (exports the value of a user setting) or launches (counts how many times the scene started)";

pub(crate) fn value(text: &str) -> Value {
    serde_json::from_str(text).unwrap_or_else(|_| Value::String(text.into()))
}

pub(crate) fn edit(dir: &Path, action: Action) -> Result<()> {
    edit_all(dir, vec![action], false)
}

pub(crate) fn edit_all(dir: &Path, actions: Vec<Action>, json: bool) -> Result<()> {
    let mut project = Project::open(dir)?;
    let single = actions.len() == 1;
    let action = actions.first().cloned().unwrap_or(Action::Undo {});
    let undoing = single && matches!(action, Action::Undo {} | Action::Redo {});
    let named: Vec<(String, String)> = actions
        .iter()
        .flat_map(|action| match action {
            Action::RenameProperty { id, .. }
            | Action::RemoveProperty { id }
            | Action::Template(TemplateEdit::Rename { name: id, .. } | TemplateEdit::Remove { name: id })
            | Action::Particles(ParticleEdit::Sow(SowEdit::Rename { system: id, .. })) => {
                references::scripts(&project.dir, &project.doc, id)
                    .into_iter()
                    .map(|p| (p, id.clone()))
                    .collect()
            }
            Action::Rename(r) => references::scripts(&project.dir, &project.doc, &r.id)
                .into_iter()
                .map(|p| (p, r.id.clone()))
                .collect(),
            _ => Vec::new(),
        })
        .collect();
    let scripts = project.doc.pointer("/scene/scripts").cloned().unwrap_or_default();
    let ops = if single { project.act(&action)? } else { project.act_all(&actions)? };
    ensure!(!(undoing && ops.is_empty()), "there is nothing to {}", action.name());
    project.save()?;
    named.iter().for_each(|(path, id)| eprintln!("warning: {path} names {id:?}; scripts are not rewritten"));
    if let (Action::Particles(ParticleEdit::Sow(SowEdit::Rename { system, to })), false) =
        (&action, named.is_empty())
    {
        eprintln!(
            "to rewrite the exact names: scene act DIR particles '{}'",
            json!({ "scripts": { "from": system, "to": to } })
        );
    }
    if matches!(action, Action::ImportScript(_)) {
        let now = project.doc.pointer("/scene/scripts").cloned().unwrap_or_default();
        wprs_editor::scripts::changes(&scripts, &now).iter().for_each(|m| eprintln!("{}", m.local()));
    }
    if json {
        println!("{}", serde_json::to_string_pretty(&json!({ "ops": ops }))?);
        return Ok(());
    }
    let added = ops.iter().filter_map(|op| match op {
        Op::Add { path, value } => {
            Some(value["id"].as_str().unwrap_or(path.rsplit('/').next().unwrap_or_default()))
        }
        _ => None,
    });
    let added: Vec<&str> = added.collect();
    match action {
        _ if !single => ops
            .iter()
            .filter_map(|op| match op {
                Op::Add { value, .. } => value["id"].as_str(),
                _ => None,
            })
            .for_each(|id| println!("{id}")),
        Action::Import { .. } => added.last().iter().for_each(|id| println!("{id}")),
        Action::AddPreset { .. } => {
            let last = project.doc["manifest"]["presets"].as_array().and_then(|l| l.last());
            last.and_then(|p| p["id"].as_str()).iter().for_each(|id| println!("{id}"))
        }
        Action::AddProperty { .. } => ops
            .iter()
            .filter_map(|op| match op {
                Op::Add { path, .. } => {
                    path.strip_prefix("/manifest/properties/").filter(|p| !p.contains('/'))
                }
                _ => None,
            })
            .for_each(|id| println!("{id}")),
        Action::Thumbnail { .. } | Action::Shot(_) | Action::Asset(_) => ops
            .iter()
            .filter_map(|op| match op {
                Op::Add { value, .. } | Op::Replace { value, .. } => value.as_str(),
                _ => None,
            })
            .for_each(|path| println!("{path}")),
        _ => ops
            .iter()
            .filter_map(|op| match op {
                Op::Add { value, .. } => value["id"].as_str(),
                _ => None,
            })
            .for_each(|id| println!("{id}")),
    }
    Ok(())
}

pub(crate) fn action(command: Author) -> Result<(PathBuf, Action), Box<Author>> {
    Ok(match command {
        Author::Set { dir, pointer, value: v } => (dir, Action::Set { pointer, value: value(&v) }),
        Author::Add { dir, pointer, value: v } => (dir, Action::Add { pointer, value: value(&v) }),
        Author::Remove { dir, pointer } => (dir, Action::Remove { pointer }),
        Author::Move { dir, from, to } => (dir, Action::Move { from, to }),
        Author::Layers(command) => crate::layering::action(command),
        Author::Keying(command) => crate::keying::action(command),
        Author::Shaping(command) => crate::shaping::action(command),
        Author::Shading(command) => crate::shading::action(command),
        Author::ImportScript { dir, file, template } => {
            let file = file.unwrap_or_default();
            let file = if template.is_some() { file } else { fs::canonicalize(&file).unwrap_or(file) };
            (dir, Action::ImportScript(ScriptFile { file, template }))
        }
        Author::PaintMask { dir, pointer, points, channel, radius, hardness, value } => {
            let pair = |p: &str| {
                p.split_once(',').and_then(|(x, y)| Some([x.trim().parse().ok()?, y.trim().parse().ok()?]))
            };
            let points =
                points.split(';').filter(|p| !p.trim().is_empty()).map(pair).collect::<Option<Vec<_>>>();
            let points = points.unwrap_or_default();
            let channel = channel.and_then(|c| ["r", "g", "b", "a"].iter().position(|n| *n == c));
            (
                dir,
                Action::PaintMask(MaskStroke {
                    pointer,
                    channel,
                    radius,
                    hardness,
                    value,
                    points,
                    direction: false,
                }),
            )
        }
        Author::Translate { dir, pointer, language, text, add, .. } => {
            (dir, Action::Translate(wprs_editor::actions::TranslateEdit { pointer, language, text, add }))
        }
        Author::Thumbnail { dir, animated, shot: Capture { file: Some(file), .. } } => {
            (dir, Action::Thumbnail { file: fs::canonicalize(&file).unwrap_or(file), animated })
        }
        Author::Thumbnail { dir, animated: false, shot, .. } => {
            (dir, Action::Shot(ShotEdit::Still { time: shot.time }))
        }
        Author::Thumbnail { dir, shot, .. } => {
            let (from, seconds, fps) = (shot.from, shot.seconds, shot.fps);
            (dir, Action::Shot(ShotEdit::Recording { from, seconds, fps }))
        }
        Author::Upgrade { dir, rating, reasons, photosensitive, derived_from } => {
            let rating = rating.and_then(|r| serde_json::from_value(json!(r)).ok());
            let reasons = reasons.iter().filter_map(|r| serde_json::from_value(json!(r)).ok()).collect();
            (dir, Action::Upgrade(UpgradeEdit { rating, reasons, photosensitive, derived_from }))
        }
        Author::Properties(command) => crate::userprops::action(command),
        Author::Rigging(command) => return crate::rigging::action(command),
        Author::Clipping(command) => return crate::clipping::action(command),
        Author::Undoing(crate::undoing::Undoing::Undo { dir }) => (dir, Action::Undo {}),
        Author::Undoing(crate::undoing::Undoing::Redo { dir }) => (dir, Action::Redo {}),
        Author::Swarming(command) => return crate::swarmed::action(command),
        Author::Lamping(command) => return crate::lamped::action(command),
        Author::Modeling(command) => return crate::modeled::action(command),
        Author::Flying(command) => return crate::animated::action(command),
        Author::Naming(command) => return crate::naming::action(command),
        other => return Err(Box::new(other)),
    })
}
