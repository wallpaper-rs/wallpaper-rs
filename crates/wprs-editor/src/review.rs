use crate::meta::facts_of;
use crate::package::problems;
use serde_json::{Value, json};
use std::fs;
use std::path::Path;
use wprs_format::features::{self, explain};
use wprs_format::{Error, Facts, MANIFEST, Manifest, Platform, SIGNATURE};

fn file_of(doc: &Value, pointer: &str) -> String {
    if let Some(path) = doc.pointer(pointer).and_then(Value::as_str).filter(|_| pointer.ends_with("/path")) {
        return path.to_owned();
    }
    match pointer {
        p if p.starts_with("/scene") => "scene.json",
        p if p.starts_with("/manifest") => MANIFEST,
        _ => "",
    }
    .to_owned()
}

fn finding(doc: &Value, level: &str, pointer: &str, message: &str) -> Value {
    json!({ "level": level, "pointer": pointer, "file": file_of(doc, pointer), "message": message })
}

pub fn platforms(manifest: &Manifest, facts: &Facts) -> Value {
    let plays = |p: Platform| {
        let refused = features::playable(manifest, facts, p).err().map(|e| e.to_string());
        json!({"refused": refused, "without": features::lacks(p.name(), &facts.features)})
    };
    Platform::ALL
        .into_iter()
        .map(|p| (p.name().to_owned(), plays(p)))
        .collect::<serde_json::Map<_, _>>()
        .into()
}

pub fn needs(manifest: &Manifest, facts: &Facts) -> (Vec<Value>, Vec<Value>) {
    let why = |f: &String| json!({"name": f, "why": explain(f)});
    let listed = manifest.features();
    let missing = facts.features.iter().filter(|f| manifest.v2() && !listed.contains(f));
    (facts.features.iter().map(why).collect(), missing.map(why).collect())
}

pub fn report(dir: &Path, doc: &Value) -> Value {
    let mut findings: Vec<Value> = problems(dir, doc)
        .iter()
        .map(|p| finding(doc, if p.warning { "warning" } else { "error" }, &p.pointer, &p.message))
        .collect();
    let mut out = json!({ "signature": signature(dir, &[]) });
    if let Some((manifest, facts)) = facts_of(doc, Some(dir)) {
        let notes = features::warnings(&manifest, &facts);
        findings.extend(notes.iter().map(|n| finding(doc, "note", "/manifest/requires/features", n)));
        let platforms = platforms(&manifest, &facts);
        for (name, plays) in platforms.as_object().into_iter().flatten() {
            if let Some(reason) = plays["refused"].as_str() {
                findings.push(finding(doc, "warning", "/manifest/platforms", &format!("{name}: {reason}")));
            }
        }
        let (needed, added) = needs(&manifest, &facts);
        out["needs"] = json!(needed);
        out["build_adds"] = json!(added);
        out["platforms"] = platforms;
        out["facets"] = json!(facts.facets);
    }
    out["findings"] = json!(findings);
    out
}

fn accepted(keys: &[String]) -> Result<Vec<(String, String)>, Error> {
    let texts = keys.iter().map(|k| wprs_format::key_text(k)).collect::<Result<Vec<_>, _>>()?;
    texts.iter().map(|t| Ok((wprs_format::fingerprint(t)?, wprs_format::public_key(t)?))).collect()
}

pub fn signature(path: &Path, keys: &[String]) -> Value {
    let (manifest, signed) = match path.is_dir() {
        true => (
            fs::read(path.join(MANIFEST)).map_err(Error::from),
            fs::read_to_string(path.join(SIGNATURE)).ok(),
        ),
        false => return packed(path, keys),
    };
    let Some(signed) = signed else { return json!({"state": "unsigned"}) };
    let Ok(json) = manifest else {
        return json!({"state": "invalid", "error": "manifest.json cannot be read"});
    };
    match Manifest::from_json(&json) {
        Ok(m) => judged(&json, &signed, &wprs_format::manifest_comment(&m.id, &m.version), keys),
        Err(e) => json!({"state": "invalid", "error": e.to_string()}),
    }
}

fn packed(path: &Path, keys: &[String]) -> Value {
    let mut package = match wprs_format::open(path, &wprs_format::Limits::default()) {
        Ok(p) => p,
        Err(e) => return json!({"state": "invalid", "error": e.to_string()}),
    };
    let Ok(signed) = package.read_file(SIGNATURE) else { return json!({"state": "unsigned"}) };
    let Ok(json) = package.read_file(MANIFEST) else {
        return json!({"state": "invalid", "error": "no manifest"});
    };
    let comment = wprs_format::manifest_comment(&package.manifest.id, &package.manifest.version);
    judged(&json, &String::from_utf8_lossy(&signed), &comment, keys)
}

fn judged(data: &[u8], signed: &str, comment: &str, keys: &[String]) -> Value {
    let pairs = match accepted(keys) {
        Ok(p) => p,
        Err(e) => return json!({"state": "invalid", "error": e.to_string()}),
    };
    if pairs.is_empty() {
        return json!({"state": "unchecked", "blocks": signed.lines().count() / 4});
    }
    let known: Vec<(&str, &str)> = pairs.iter().map(|(f, k)| (f.as_str(), k.as_str())).collect();
    match wprs_format::verify_blocks(data, signed, comment, &known) {
        Ok(signers) if !signers.is_empty() => json!({"state": "valid", "signers": signers}),
        Ok(_) => json!({"state": "invalid", "error": "none of the given keys signed it"}),
        Err(e) => json!({"state": "invalid", "error": e.to_string()}),
    }
}
