use crate::author::DIR;
use crate::steps::edit_all;
use crate::usage;
use anyhow::Result;
use clap::Subcommand;
use serde_json::{Value, json};
use std::io::Read;
use std::path::PathBuf;
use wprs_editor::Project;
use wprs_editor::actions::Action;
use wprs_editor::manual;

const ARGS: &str =
    "Arguments of the action as a JSON object; - reads them from the standard input; {} when omitted";
const FILE: &str = "Read the JSON from this file instead of the argument";

#[derive(Subcommand, Debug)]
pub enum Acting {
    #[command(
        about = "Apply any editor action to a scene folder, as the app does; the name batch takes a JSON list of {action: arguments} and applies them as one undo step",
        after_long_help = "Every action of the editor is reachable here; wprs-ctl scene actions lists them with their arguments.\nExample: wprs-ctl scene act DIR hide '{\"id\": \"sky\", \"hidden\": true}'"
    )]
    Act {
        #[arg(help = DIR)]
        dir: PathBuf,
        #[arg(help = "Action name, such as add_layer or set, or batch for a list of actions")]
        action: String,
        #[arg(value_name = "JSON", allow_hyphen_values = true, help = ARGS)]
        args: Option<String>,
        #[arg(long, value_name = "PATH", conflicts_with = "args", help = FILE)]
        file: Option<PathBuf>,
        #[arg(long, help = "Print the operations that changed the document as JSON")]
        json: bool,
    },
    #[command(
        about = "List the editor actions with a line each, or show the arguments and an example of one; with --queries, the same for the questions scene ask answers"
    )]
    Actions {
        #[arg(help = "Action (or with --queries, query) to describe; every one is listed when omitted")]
        name: Option<String>,
        #[arg(long, help = "Describe the queries of scene ask instead of the actions")]
        queries: bool,
        #[arg(long, help = "Print the list, or the JSON schema of the arguments, as JSON")]
        json: bool,
    },
    #[command(
        about = "Ask the editor a question about a scene folder, as the app does, and print the reply as JSON"
    )]
    Ask {
        #[arg(help = DIR)]
        dir: PathBuf,
        #[arg(help = "Query name; wprs-ctl scene actions --queries lists them")]
        query: String,
        #[arg(
            value_name = "JSON",
            allow_hyphen_values = true,
            help = "Arguments of the query as JSON; - reads them from the standard input"
        )]
        args: Option<String>,
        #[arg(long, value_name = "PATH", conflicts_with = "args", help = FILE)]
        file: Option<PathBuf>,
        #[arg(
            long,
            default_value = "",
            help = "Layer the query is about when its arguments do not name one"
        )]
        layer: String,
    },
}

fn given(args: Option<String>, file: Option<PathBuf>) -> Result<Value> {
    let text = match (args.as_deref(), file) {
        (_, Some(path)) => std::fs::read_to_string(&path)?,
        (Some("-"), _) => std::io::read_to_string(std::io::stdin().lock().by_ref())?,
        (Some(text), _) => text.to_owned(),
        (None, None) => return Ok(json!({})),
    };
    serde_json::from_str(&text).map_err(|e| usage(format!("the arguments are not JSON: {e}")))
}

fn actions(name: &str, args: Value) -> Result<Vec<Action>> {
    let each = |value: Value| serde_json::from_value::<Action>(value).map_err(|e| usage(e.to_string()));
    if name != "batch" {
        return Ok(vec![each(json!({ name: args }))?]);
    }
    let Value::Array(list) = args else {
        return Err(usage("batch takes a JSON list of {action: arguments}"));
    };
    let all = list.into_iter().map(each).collect::<Result<Vec<_>>>()?;
    anyhow::ensure!(!all.is_empty(), usage("batch takes at least one action"));
    Ok(all)
}

fn listed(what: &[(&str, &str)], json: bool) -> Result<()> {
    if json {
        let all: Vec<Value> = what.iter().map(|(n, a)| json!({ "name": n, "about": a })).collect();
        println!("{}", serde_json::to_string_pretty(&all)?);
        return Ok(());
    }
    let width = what.iter().map(|(n, _)| n.len()).max().unwrap_or(0);
    what.iter().for_each(|(n, a)| println!("{n:width$}  {a}"));
    Ok(())
}

fn example_text(kind: &str, name: &str) -> String {
    let lines = manual::example_lines(kind, name).unwrap_or_default();
    let head = match lines.len() > 1 {
        true => "Example (the lines before the last one prepare the scene):",
        false => "Example:",
    };
    format!("\n{head}\n  {}", lines.join("\n  "))
}

fn described(name: &str, json: bool) -> Result<()> {
    let Some(schema) = manual::schema(name) else {
        return Err(usage(format!("there is no action {name:?}; wprs-ctl scene actions lists them")));
    };
    if json {
        println!("{}", serde_json::to_string_pretty(&schema)?);
        return Ok(());
    }
    println!("{}", manual::describe(name).unwrap_or_default());
    println!("{}", example_text("act", name));
    Ok(())
}

fn asked(name: &str, json: bool) -> Result<()> {
    let Some(about) = manual::ask_about(name) else {
        return Err(usage(format!(
            "there is no query {name:?}; wprs-ctl scene actions --queries lists them"
        )));
    };
    let lines = manual::example_lines("ask", name).unwrap_or_default();
    if json {
        println!("{}", json!({ "name": name, "about": about, "example": lines }));
        return Ok(());
    }
    println!("{name}: {about}\n{}", example_text("ask", name));
    Ok(())
}

pub fn run(command: Acting) -> Result<()> {
    match command {
        Acting::Act { dir, action, args, file, json } => {
            edit_all(&dir, actions(&action, given(args, file)?)?, json)
        }
        Acting::Actions { name: None, queries: false, json } => listed(&manual::ABOUT, json),
        Acting::Actions { name: None, queries: true, json } => listed(&manual::ASKS, json),
        Acting::Actions { name: Some(name), queries: false, json } => described(&name, json),
        Acting::Actions { name: Some(name), queries: true, json } => asked(&name, json),
        Acting::Ask { dir, query, args, file, layer } => {
            let args = given(args, file)?;
            anyhow::ensure!(
                manual::ask_about(&query).is_some(),
                usage(format!("there is no query {query:?}; wprs-ctl scene actions --queries lists them"))
            );
            let project = Project::open(&dir)?;
            let found = wprs_editor::rigs::query(
                &project.dir,
                &project.holding(&layer),
                &layer,
                &manual::question(&query, args),
            );
            println!("{}", serde_json::to_string_pretty(&found)?);
            Ok(())
        }
    }
}
