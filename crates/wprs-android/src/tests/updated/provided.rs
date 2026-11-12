use crate::library;
use crate::tests::ask;
use base64::Engine;
use serde_json::{Value, json};
use std::io::{Cursor, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering::Relaxed};

const PICTURES: u8 = 7;

struct Hits {
    listings: AtomicU32,
    downloads: AtomicU32,
}

fn picture(n: u8) -> Vec<u8> {
    let mut bytes = Cursor::new(Vec::new());
    let image = image::RgbImage::from_pixel(16, 8, image::Rgb([n * 30, 255 - n * 30, 90]));
    image.write_to(&mut bytes, image::ImageFormat::Png).unwrap();
    bytes.into_inner()
}

fn listing() -> String {
    let each = |n: u8| {
        format!(
            "<d:response><d:href>/dav/n{n}.png</d:href><d:propstat><d:prop><d:resourcetype/></d:prop></d:propstat></d:response>"
        )
    };
    let folder = "<d:response><d:href>/dav/</d:href><d:propstat><d:prop><d:resourcetype><d:collection/></d:resourcetype></d:prop></d:propstat></d:response>";
    let files: String = (0..PICTURES).map(each).collect();
    format!("<?xml version=\"1.0\"?><d:multistatus xmlns:d=\"DAV:\">{folder}{files}</d:multistatus>")
}

fn answer(mut stream: TcpStream, hits: &Hits, secret: &str) {
    let mut head = Vec::new();
    let mut byte = [0u8; 1];
    while !head.ends_with(b"\r\n\r\n") && stream.read(&mut byte).unwrap_or(0) == 1 {
        head.push(byte[0]);
    }
    let text = String::from_utf8_lossy(&head).into_owned();
    let length = text
        .lines()
        .find_map(|l| l.to_lowercase().strip_prefix("content-length:").and_then(|n| n.trim().parse().ok()));
    let mut body = vec![0u8; length.unwrap_or(0)];
    drop(stream.read_exact(&mut body));
    let wanted = base64::engine::general_purpose::STANDARD.encode(format!("me:{secret}"));
    let allowed = text.to_lowercase().contains(&format!("authorization: basic {}", wanted.to_lowercase()));
    let mut words = text.split_whitespace();
    let (method, path) = (words.next().unwrap_or_default(), words.next().unwrap_or_default());
    let (status, payload) = match (allowed, method, path) {
        (false, ..) => ("401 Unauthorized", Vec::new()),
        (_, "PROPFIND", _) => {
            hits.listings.fetch_add(1, Relaxed);
            ("207 Multi-Status", listing().into_bytes())
        }
        (_, "GET", path) => {
            hits.downloads.fetch_add(1, Relaxed);
            let n = path.trim_start_matches("/dav/n").trim_end_matches(".png").parse().unwrap_or(0);
            ("200 OK", picture(n))
        }
        _ => ("404 Not Found", Vec::new()),
    };
    let head = format!(
        "HTTP/1.1 {status}\r\nContent-Type: application/xml\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        payload.len()
    );
    drop(stream.write_all(head.as_bytes()).and_then(|()| stream.write_all(&payload)));
}

fn serve(secret: &'static str) -> (u16, Arc<Hits>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let hits = Arc::new(Hits { listings: AtomicU32::new(0), downloads: AtomicU32::new(0) });
    let counted = hits.clone();
    std::thread::spawn(move || {
        for stream in listener.incoming().flatten() {
            let hits = counted.clone();
            std::thread::spawn(move || answer(stream, &hits, secret));
        }
    });
    (port, hits)
}

fn account(request: Value) -> Value {
    serde_json::from_str(&crate::account::request(&request.to_string())).unwrap()
}

fn mine(id: &str) -> Value {
    let listed = account(json!({ "op": "providers" }));
    listed["providers"].as_array().unwrap().iter().find(|p| p["id"] == id).cloned().unwrap_or(Value::Null)
}

fn brought(id: &str) -> usize {
    ask(json!({ "op": "items", "query": format!("tag:from-{id}") })).as_array().map_or(0, Vec::len)
}

pub(super) fn picture_sources_download_only_when_turned_on() {
    let listed = account(json!({ "op": "providers" }));
    assert_eq!(listed["providers"], json!([]));
    let kinds: Vec<&str> =
        listed["kinds"].as_array().unwrap().iter().map(|k| k["kind"].as_str().unwrap()).collect();
    assert_eq!(kinds, ["bing", "apod", "wikimedia", "wallhaven", "webdav", "nextcloud", "immich"]);
    assert_eq!(listed["network"], json!("unmetered"));

    let (port, hits) = serve("hunter2");
    let url = format!("http://127.0.0.1:{port}/dav/");
    let add = |url: &str, secret: &str, on: bool| {
        account(
            json!({ "op": "provider_add", "kind": "webdav", "options": { "url": url, "user": "me" }, "secret": secret, "on": on }),
        )
    };
    assert_eq!(add(&url, "", false)["id"], json!("android.provider_secret"));
    assert_eq!(add("http://example.org/dav/", "x", false)["id"], json!("android.provider_https"));
    assert_eq!(add("not a web address", "x", false)["id"], json!("android.provider_address"));
    let wrong =
        account(json!({ "op": "provider_add", "kind": "bing", "options": { "market": "bad market" } }));
    assert_eq!(wrong["id"], json!("android.provider_option"));
    assert_eq!(
        account(json!({ "op": "provider_add", "kind": "mystery" }))["id"],
        json!("android.provider_kind")
    );

    assert_eq!(add(&url, "hunter2", false), json!("webdav"));
    let shown = mine("webdav");
    assert_eq!(
        (&shown["on"], &shown["count"], &shown["hosts"]),
        (&json!(false), &json!(0), &json!(["127.0.0.1"]))
    );
    assert_eq!(shown["secret"], json!({ "name": "password", "required": true, "held": true }));
    let refused = account(json!({ "op": "provider_sync", "id": "webdav", "force": true }));
    assert_eq!(refused["id"], json!("android.provider_off"));
    assert_eq!(account(json!({ "op": "provider_sync", "id": "", "force": true })), json!([]));
    assert_eq!(
        (hits.listings.load(Relaxed), hits.downloads.load(Relaxed)),
        (0, 0),
        "nothing is contacted while it is off"
    );

    account(json!({ "op": "provider_on", "id": "webdav", "on": true }));
    account(json!({ "op": "provider_secret", "id": "webdav", "secret": "wrong" }));
    let rejected = account(json!({ "op": "provider_sync", "id": "webdav", "force": false }));
    assert_eq!(rejected["id"], json!("android.provider_secret_rejected"));
    assert_eq!(mine("webdav")["last_error"]["id"], json!("android.provider_secret_rejected"));
    account(json!({ "op": "provider_secret", "id": "webdav", "secret": "hunter2" }));

    account(json!({ "op": "provider_set", "id": "webdav", "key": "keep", "value": "5" }));
    let first = account(json!({ "op": "provider_sync", "id": "webdav", "force": false }));
    assert_eq!((&first[0]["listed"], &first[0]["brought"]), (&json!(7), &json!(5)), "{first}");
    assert_eq!(brought("webdav"), 5);
    assert_eq!(mine("webdav")["credits"].as_array().map(Vec::len), Some(5));
    let before = hits.downloads.load(Relaxed);
    let second = account(json!({ "op": "provider_sync", "id": "", "force": false }));
    assert_eq!(second, json!([]), "a source that is not due is left alone");
    let third = account(json!({ "op": "provider_sync", "id": "webdav", "force": true }));
    assert_eq!(third[0]["brought"], json!(2), "{third}");
    assert_eq!(hits.downloads.load(Relaxed) - before, 2, "what came before is not downloaded again");
    assert_eq!(brought("webdav"), 5, "the oldest go once more than the kept number arrive");

    let favorite =
        ask(json!({ "op": "items", "query": "tag:from-webdav" }))[0]["uid"].as_str().unwrap().to_owned();
    ask(json!({ "op": "favorite", "uids": [favorite], "favorite": true }));
    account(json!({ "op": "provider_set", "id": "webdav", "key": "keep", "value": "5" }));
    let moved = account(
        json!({ "op": "provider_set", "id": "webdav", "key": "url", "value": format!("http://localhost:{port}/dav/") }),
    );
    assert_eq!(moved, Value::Null);
    let after = mine("webdav");
    assert_eq!(
        (&after["on"], &after["secret"]["held"]),
        (&json!(false), &json!(false)),
        "a new address needs a new yes"
    );

    let gone = account(json!({ "op": "provider_remove", "id": "webdav", "purge": true }));
    assert_eq!(gone, Value::Null);
    assert_eq!(mine("webdav"), Value::Null);
    assert_eq!(brought("webdav"), 0);
    assert!(library::with(|lib| Ok(lib.item(&favorite).is_err())).unwrap());
}
