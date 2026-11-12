use wprs_script::syntax;

fn source(path: &str, text: &str) -> (String, String) {
    (path.to_owned(), text.to_owned())
}

#[test]
fn a_syntax_error_names_its_script_and_line() {
    let good = "export function update(frame) {\n  return frame.time;\n}\n";
    let bad = "export function update(frame) {\n  return frame.time;\n\nlet = = 3;\n}\n";
    let faults = syntax(&[source("scripts/a.js", good), source("scripts/b.js", bad)], &[]);
    assert_eq!(faults.len(), 1, "{faults:?}");
    assert_eq!(faults[0].0, 1);
    assert_eq!(faults[0].1.line, Some(4), "{:?}", faults[0].1);
    assert!(!faults[0].1.message.is_empty());
}

#[test]
fn modules_that_import_each_other_compile_and_nothing_runs() {
    let a =
        "import { b } from './b.js';\nthrow new Error('never');\nexport function update() { return b; }\n";
    let b = "export const b = 1;\n";
    let faults = syntax(&[source("scripts/a.js", a), source("scripts/b.js", b)], &[]);
    assert!(faults.is_empty(), "{faults:?}");
}
