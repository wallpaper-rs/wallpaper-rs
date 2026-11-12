use rquickjs::{Context, Runtime};

const STUBS: &str = include_str!("sounding.js");
const PHONE: &str = include_str!("../../page/phone.js");

fn said(steps: &str) -> String {
    let runtime = Runtime::new().unwrap();
    let context = Context::full(&runtime).unwrap();
    context.with(|ctx| {
        let seen = ctx.eval::<String, _>(format!("{STUBS}\n{PHONE}\n{steps}\nposted.join(',')"));
        seen.unwrap_or_else(|_| panic!("{:?}", ctx.catch().as_exception().map(|e| (e.message(), e.stack()))))
    })
}

#[test]
fn a_page_says_when_it_sounds_and_when_it_falls_silent() {
    assert_eq!(said(""), "false", "a page starts silent");
    let media = "const a = new HTMLMediaElement(); a.play(); a.muted = true; a.fire('volumechange'); \
                 a.muted = false; a.fire('volumechange'); a.pause();";
    assert_eq!(said(media), "false,true,false,true,false");
    let quiet = "const a = new HTMLMediaElement(); a.volume = 0; a.play(); a.pause();";
    assert_eq!(said(quiet), "false", "a player at volume 0 is silent");
    let context = "const c = new AudioContext(); c.suspend();";
    assert_eq!(said(context), "false,true,false");
    let both = "const a = new HTMLMediaElement(); const c = new AudioContext(); a.play(); c.suspend(); a.pause();";
    assert_eq!(said(both), "false,true,false", "silent only when nothing sounds");
}
