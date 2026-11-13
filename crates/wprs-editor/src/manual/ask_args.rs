use serde_json::{Value, json};
use wprs_core::message::{Message, said};

const TEXT: &str = include_str!("asks_args.txt");

pub struct Field {
    pub name: &'static str,
    pub kind: &'static str,
    pub required: bool,
    pub about: &'static str,
}

fn parse(line: &'static str) -> Option<(&'static str, Field)> {
    let mut part = line.split('|');
    let query = part.next()?;
    let field = Field {
        name: part.next()?,
        kind: part.next()?,
        required: part.next()? == "required",
        about: part.next()?,
    };
    Some((query, field))
}

pub fn known(query: &str) -> bool {
    TEXT.lines().any(|l| l.split('|').next() == Some(query))
}

pub fn fields(query: &str) -> Vec<Field> {
    TEXT.lines().filter_map(parse).filter(|(q, _)| *q == query).map(|(_, f)| f).collect()
}

fn property(field: &Field) -> Value {
    match field.kind {
        "any" => json!({ "description": field.about }),
        kind => json!({ "type": kind, "description": field.about }),
    }
}

pub fn schema(query: &str) -> Value {
    let all = fields(query);
    if let [bare] = all.as_slice().iter().filter(|f| f.name.is_empty()).collect::<Vec<_>>()[..] {
        return json!({ "title": query, "oneOf": [property(bare), { "type": "object", "maxProperties": 0 }] });
    }
    let mut properties = serde_json::Map::new();
    all.iter().for_each(|f| _ = properties.insert(f.name.into(), property(f)));
    let required: Vec<&str> = all.iter().filter(|f| f.required).map(|f| f.name).collect();
    json!({ "title": query, "type": "object", "properties": properties, "required": required,
        "additionalProperties": false })
}

pub fn describe(query: &str) -> Vec<String> {
    let all = fields(query);
    let how = |f: &Field| if f.required { "required" } else { "optional" };
    match all.as_slice() {
        [] => vec!["  no arguments".into()],
        [bare] if bare.name.is_empty() => {
            vec![format!("  the argument is a bare JSON {} ({}): {}", bare.kind, how(bare), bare.about)]
        }
        _ => all.iter().map(|f| format!("  {}: {} ({}) - {}", f.name, f.kind, how(f), f.about)).collect(),
    }
}

fn fits(kind: &str, value: &Value) -> bool {
    match kind {
        "string" => value.is_string(),
        "number" => value.is_number(),
        "integer" => value.is_i64() || value.is_u64(),
        "boolean" => value.is_boolean(),
        "array" => value.is_array(),
        "object" => value.is_object(),
        _ => true,
    }
}

pub fn check(query: &str, args: &Value, layer: &str) -> Result<(), Message> {
    let all = fields(query);
    let wrong = |name: &str, kind: &str| {
        said("editor.ask_kind", "the argument %1 of the query %2 must be a JSON %3", &[&name, &query, &kind])
    };
    if let [bare] = all.as_slice().iter().filter(|f| f.name.is_empty()).collect::<Vec<_>>()[..] {
        let empty = args.as_object().is_some_and(|o| o.is_empty());
        return if args.is_string() || (empty && !bare.required) {
            Ok(())
        } else {
            Err(wrong(query, bare.kind))
        };
    }
    let Some(given) = args.as_object() else {
        return Err(said(
            "editor.ask_object",
            "the arguments of the query %1 must be a JSON object",
            &[&query],
        ));
    };
    if let Some(key) = given.keys().find(|k| !all.iter().any(|f| f.name == k.as_str())) {
        let listed = all.iter().map(|f| f.name).collect::<Vec<_>>().join(", ");
        let listed = if listed.is_empty() { "none".to_owned() } else { listed };
        return Err(said(
            "editor.ask_unknown",
            "%1 is not an argument of the query %2; it takes: %3",
            &[&key, &query, &listed],
        ));
    }
    for f in &all {
        match given.get(f.name) {
            Some(value) if !fits(f.kind, value) => return Err(wrong(f.name, f.kind)),
            None if f.required && !(f.name == "layer" && !layer.is_empty()) => {
                return Err(said(
                    "editor.ask_missing",
                    "the query %1 needs the argument %2",
                    &[&query, &f.name],
                ));
            }
            _ => {}
        }
    }
    Ok(())
}
