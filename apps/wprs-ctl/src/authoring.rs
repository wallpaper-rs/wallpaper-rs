use crate::author::Author;
use crate::steps::{action, edit};
use crate::told::{fill, told, tr};
use crate::usage;
use anyhow::{Context, Result, bail};
use std::fs;
use std::path::Path;
use wprs_editor::starters::{default_id, starter};
use wprs_editor::{Project, inspect, package};

fn describe(project: &Project, json: bool) -> Result<()> {
    let problems = package::problems(&project.dir, &project.doc);
    let errors = problems.iter().filter(|p| !p.warning).count();
    let m = &project.doc["manifest"];
    let layers = wprs_editor::layers::walk(&project.doc).len();
    let pending = project.pending.is_some();
    if json {
        let files = wprs_editor::check::check_files(&project.dir, &project.doc);
        println!("{}", wprs_editor::outline::outline(project, 0.0, &files));
        return Ok(());
    }
    let text = |k: &str| m[k].as_str().unwrap_or("?").to_owned();
    let title = wprs_editor::meta::title(&project.doc, &wprs_core::locale::languages());
    let (layers, errors) = (layers_of(layers), problems_of(errors));
    if !wprs_editor::pages::scened(&project.doc) {
        let kind = wprs_editor::pages::kind(&project.doc);
        println!("{title} ({} {}, {kind}): {errors}", text("id"), text("version"));
        for file in wprs_editor::pages::files(&project.dir, &project.doc) {
            let (path, role) = (file["path"].as_str().unwrap_or(""), file["role"].as_str().unwrap_or(""));
            println!(
                "{}",
                fill(
                    told("  %1: %2, %3, %4 bytes"),
                    &[&path, &role, &file["kind"].as_str().unwrap_or(""), &file["size"]]
                )
            );
        }
    } else {
        println!("{title} ({} {}): {layers}, {errors}", text("id"), text("version"));
    }
    if pending {
        println!(
            "{}",
            tr(told("the app kept unsaved changes; wprs-ctl scene recover restores or drops them"))
        );
    }
    Ok(())
}

pub(crate) fn layers_of(n: usize) -> String {
    match n {
        1 => tr(told("1 layer")).to_owned(),
        n => fill(told("%1 layers"), &[&n]),
    }
}

pub(crate) fn problems_of(n: usize) -> String {
    match n {
        1 => tr(told("1 problem")).to_owned(),
        n => fill(told("%1 problems"), &[&n]),
    }
}

pub fn run(command: Author) -> Result<()> {
    let command = match action(command) {
        Ok((dir, action)) => return edit(&dir, action),
        Err(command) => *command,
    };
    match command {
        Author::New { dir, from, id, title, locale } => {
            let name = fs::canonicalize(
                dir.parent().filter(|p| !p.as_os_str().is_empty()).unwrap_or(Path::new(".")),
            )?
            .join(dir.file_name().context(tr(told("name the new folder")))?);
            let folder = name.file_name().unwrap_or_default().to_string_lossy().into_owned();
            let id = id.unwrap_or_else(|| default_id(&folder));
            let locale = locale.unwrap_or_else(wprs_editor::starters::locale);
            let doc = starter(&from, &id, &title.unwrap_or(folder), &locale)
                .map_err(|e| usage(wprs_editor::told(&e)))?;
            Project::create(&dir, doc)?;
        }
        Author::Open { dir, from, copying, json } => {
            if let Some(file) = from {
                if copying.facts {
                    println!("{}", wprs_editor::copying::facts(&file)?);
                    return Ok(());
                }
                wprs_editor::copying::extract(&file, &dir, copying.id.as_deref(), copying.mine)?;
            }
            describe(&Project::open(&dir)?, json)?;
        }
        Author::Show { dir, pointer } => {
            let project = Project::open(&dir)?;
            let shown = project
                .doc
                .pointer(pointer.as_deref().unwrap_or(""))
                .context(tr(told("nothing is at that pointer")))?;
            println!("{}", serde_json::to_string_pretty(shown)?);
        }
        Author::Fields { dir, pointer } => {
            println!(
                "{}",
                serde_json::to_string_pretty(&{
                    let project = Project::open(&dir)?;
                    inspect::fields_in(Some(&project.dir), &project.doc, &pointer, &project.effects())
                })?
            );
        }
        Author::Folder(command) => crate::folder::run(command)?,
        Author::Rigging(command) => crate::rigging::run(command)?,
        Author::Clipping(command) => crate::clipping::run(command)?,
        Author::Swarming(command) => crate::swarmed::run(command)?,
        Author::Lamping(command) => crate::lamped::run(command)?,
        Author::Modeling(command) => crate::modeled::run(command)?,
        Author::Flying(command) => crate::animated::run(command)?,
        Author::Texturing(command) => crate::texturing::run(command)?,
        Author::Triggering(command) => crate::triggering::run(command)?,
        Author::Acting(command) => crate::acting::run(command)?,
        Author::Naming(command) => crate::naming::run(command)?,
        Author::Rendering(command) => crate::rendering::run(command)?,
        other => bail!("{}", fill(told("%1 is not a scene command"), &[&format!("{other:?}")])),
    }
    Ok(())
}
