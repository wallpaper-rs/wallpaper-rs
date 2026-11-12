use crate::show::{self, list, text};
use crate::told::{fill, told, tr};
use anyhow::Result;
use serde_json::{Value, json};
use std::path::Path;
use wprs_editor::review;
use wprs_format::{Content, Error, Facts, Limits, Manifest, Origin, features, inspect, open, upgrade};

pub fn check(path: &Path, json: bool) -> Result<()> {
    let (manifest, facts) = match path.is_dir() {
        true => inspect(path)?,
        false => {
            let mut package = open(path, &Limits::default())?;
            package.look_closer()?;
            (package.manifest, package.facts)
        }
    };
    match features::check(&manifest, &facts) {
        Err(Error::Invalid { field, .. }) if field == "requires.features" => {}
        checked if manifest.v2() => checked?,
        _ => {}
    }
    let problems: Vec<String> =
        crate::scriptfaults::of(path, &manifest).iter().map(|f| f.said().local()).collect();
    let listed = manifest.features();
    let (needed, missing) = review::needs(&manifest, &facts);
    let platforms = review::platforms(&manifest, &facts);
    let report = json!({
        "path": path, "schema": manifest.schema, "id": manifest.id, "version": manifest.version,
        "kind": manifest.kind, "title": manifest.title_in(&wprs_core::locale::languages()),
        "rating": manifest.content.as_ref().map(|c| text(&c.rating)),
        "reasons": manifest.content.iter().flat_map(|c| &c.descriptors).map(text).collect::<Vec<_>>(),
        "photosensitive": manifest.photosensitive, "origin": manifest.origin.as_ref().map(|o| text(&o.kind)),
        "inputs": manifest.inputs.iter().map(|i| i.name()).collect::<Vec<_>>(),
        "features": needed, "declared": listed, "missing": missing,
        "facets": facts.facets, "languages": manifest.locales(), "platforms": platforms,
        "warnings": features::warnings(&manifest, &facts), "problems": problems,
    });
    match json {
        true => println!("{report}"),
        false => print!("{}", human(&manifest, &facts, &report)),
    }
    anyhow::ensure!(problems.is_empty(), "{}", crate::authoring::problems_of(problems.len()));
    Ok(())
}

fn human(manifest: &Manifest, facts: &Facts, report: &Value) -> String {
    let words = |key: &str| list(report[key].as_array().into_iter().flatten().filter_map(Value::as_str));
    let named =
        |f: &Value| format!("{} ({})", f["name"].as_str().unwrap_or(""), f["why"].as_str().unwrap_or(""));
    let needs = |key: &str| list(report[key].as_array().into_iter().flatten().map(named));
    let flag = |on: Option<bool>| on.map_or("unknown", |on| if on { "yes" } else { "no" }).to_string();
    let rating = report["rating"].as_str().unwrap_or("unrated").to_string();
    let plays = |on: &Value| match (on["refused"].as_str(), on["without"].as_array()) {
        (Some(reason), _) => format!("refused: {reason}"),
        (None, Some(without)) if !without.is_empty() => {
            fill(told("plays without %1"), &[&list(without.iter().filter_map(Value::as_str))])
        }
        _ => "plays".into(),
    };
    let reasons = Some(words("reasons")).filter(|r| !r.is_empty()).map(|r| format!(" ({r})"));
    let problems = report["problems"].as_array().into_iter().flatten().filter_map(Value::as_str);
    let problems: String = problems.map(|p| fill(told("problem: %1\n"), &[&p])).collect();
    let state = if problems.is_empty() { "ok" } else { "problems" };
    let head =
        format!("{state} {} {} {} ({})\n", manifest.kind, manifest.id, manifest.version, manifest.schema);
    let pairs = [
        ("title", report["title"].as_str().unwrap_or("").to_string()),
        ("rating", rating + &reasons.unwrap_or_default()),
        ("photosensitive", flag(manifest.photosensitive)),
        ("origin", report["origin"].as_str().unwrap_or("-").to_string()),
        ("inputs", words("inputs")),
        ("needs", needs("features")),
        (&tr(told("build adds")), needs("missing")),
        ("facets", list(&facts.facets)),
        ("desktop", plays(&report["platforms"]["desktop"])),
        ("android", plays(&report["platforms"]["android"])),
        ("languages", words("languages")),
    ];
    let shown = pairs.into_iter().filter(|(_, v)| !v.is_empty());
    let warnings = report["warnings"].as_array().into_iter().flatten().filter_map(Value::as_str);
    head + &problems
        + &show::lines(shown)
        + &warnings.map(|w| format!("{}\n", fill(told("warning: %1"), &[&w]))).collect::<String>()
}

pub fn upgraded(
    dir: &Path,
    rating: &str,
    reasons: &[String],
    photosensitive: bool,
    from: &[String],
) -> Result<()> {
    let content: Content = serde_json::from_value(json!({"rating": rating, "descriptors": reasons}))?;
    let (manifest, renames) = upgrade(dir, content, photosensitive, Origin::by(from))?;
    let inputs = list(manifest.inputs.iter().map(|i| i.name()));
    println!(
        "{}",
        fill(
            told("%1 %2 is now %3 (inputs: %4)"),
            &[&manifest.id, &manifest.version, &manifest.schema, &inputs]
        )
    );
    renames.iter().for_each(|(old, new)| println!("{}", fill(told("property %1 is now %2"), &[&old, &new])));
    Ok(())
}
