use super::ask;
use serde_json::{Value, json};

pub(super) fn one_question(pulse: &str) {
    let form = |uid: &str| ask(json!({ "op": "scene", "uid": uid, "languages": [] }));
    let asks = |uid: &str| ask(json!({ "op": "asks", "uid": uid }));
    let decide = |allow: Value, deny: Value| {
        ask(json!({ "op": "decide", "uid": pulse, "allow": allow, "deny": deny }))
    };
    assert_eq!((&form(pulse)["undecided"], asks(pulse)), (&json!(["audio"]), json!(["audio"])));
    assert!(decide(json!(["audio"]), json!(["audio"]))["error"].is_string());
    assert!(decide(json!(["network"]), json!([]))["error"].is_string());
    assert_eq!(decide(json!([]), json!(["audio"])), Value::Null);
    let declined = form(pulse);
    assert_eq!((&declined["undecided"], &declined["granted"]["audio"]), (&json!([]), &json!(false)));
    assert_eq!(decide(json!(["audio"]), json!([])), Value::Null);
    let allowed = form(pulse);
    assert_eq!((&allowed["undecided"], &allowed["granted"]["audio"]), (&json!([]), &json!(true)));
    assert_eq!(ask(json!({ "op": "grant", "uid": pulse, "input": "audio", "on": false })), Value::Null);
    let off = form(pulse);
    assert_eq!((&off["undecided"], &off["granted"]["audio"]), (&json!([]), &json!(false)));
}

pub(super) fn told_once(pulse: &str) {
    let tell = || ask(json!({ "op": "tell", "uid": pulse }));
    assert_eq!((tell(), tell()), (json!(true), json!(false)));
}
