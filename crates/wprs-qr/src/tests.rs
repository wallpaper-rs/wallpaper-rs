use super::*;

const FP: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

fn link(server: &str, invite: Option<&str>) -> Link {
    Link::new(server, Some(FP), invite).unwrap()
}

pub(crate) fn decode(code: &Code) -> String {
    let (scale, quiet) = (6, 4);
    let side = (code.size() + 2 * quiet) as usize * scale;
    let pixel = |x: usize, y: usize| {
        let (mx, my) = ((x / scale) as i32 - quiet, (y / scale) as i32 - quiet);
        if code.dark(mx, my) { 0 } else { 255 }
    };
    let mut image = rqrr::PreparedImage::prepare_from_greyscale(side, side, pixel);
    let grids = image.detect_grids();
    assert_eq!(grids.len(), 1);
    grids[0].decode().unwrap().1
}

#[test]
fn text_round_trips() {
    for l in [
        link("https://wallpapers.example.org", None),
        link("https://192.168.1.20:8443", Some("0123456789abcdef0123456789abcdef")),
        link("https://[fe80::1]:443", Some("abc-DEF_12")),
        link("https://a.b-c.example", None),
        link("https://h.example:8443/wp/v2", Some("abcdefgh")),
    ] {
        assert_eq!(Link::parse(&l.text()).unwrap(), l);
    }
}

#[test]
fn a_code_without_a_pin_carries_the_address_alone() {
    for invite in [None, Some("abcdefgh")] {
        let l = Link::new("https://h.example", None, invite).unwrap();
        assert!(!l.text().contains("fp="));
        assert_eq!(Link::parse(&l.text()).unwrap(), l);
        assert_eq!(decode(&Code::new(&l.text()).unwrap()), l.text());
    }
    assert_eq!(
        Link::new("https://h.example", None, None).unwrap().text(),
        "wprs://connect/1?server=https://h.example"
    );
}

#[test]
fn text_is_fixed() {
    assert_eq!(
        link("https://host.example:8443", None).text(),
        format!("wprs://connect/1?server=https://host.example:8443&fp={FP}")
    );
    let with = link("https://h.example", Some("abcdefgh"));
    assert!(with.text().ends_with(&format!("&fp={FP}&invite=abcdefgh")));
    assert_eq!(with.invitation_url().unwrap(), "https://h.example/register?invite=abcdefgh");
}

#[test]
fn builder_tidies_what_people_paste() {
    let l = Link::new(" HTTPS://Host.Example/Wp/ ", Some(&FP.to_uppercase()), None).unwrap();
    assert_eq!((l.server.as_str(), l.fingerprint.as_deref().unwrap()), ("https://host.example/Wp", FP));
}

#[test]
fn builder_refuses_bad_parts() {
    assert_eq!(Link::new("http://h.example", Some(FP), None), Err(Error::Server));
    assert_eq!(Link::new("https://h.example/a//b", Some(FP), None), Err(Error::Server));
    assert_eq!(Link::new("https://h.example/a/../b", Some(FP), None), Err(Error::Server));
    assert_eq!(Link::new("https://h.example/a?x=1", Some(FP), None), Err(Error::Server));
    assert_eq!(Link::new("https://h.example/a b", Some(FP), None), Err(Error::Server));
    assert_eq!(Link::new("https://h.example:0", Some(FP), None), Err(Error::Server));
    assert_eq!(Link::new("https://h.example:70000", Some(FP), None), Err(Error::Server));
    assert_eq!(Link::new("https://user@h.example", Some(FP), None), Err(Error::Server));
    assert_eq!(Link::new("https://", Some(FP), None), Err(Error::Server));
    assert_eq!(Link::new("https://-h.example", Some(FP), None), Err(Error::Server));
    assert_eq!(Link::new("https://h.example", Some(&FP[1..]), None), Err(Error::Fingerprint));
    assert_eq!(Link::new("https://h.example", Some(&FP.replace('0', "g")), None), Err(Error::Fingerprint));
    assert_eq!(Link::new("https://h.example", Some(FP), Some("short")), Err(Error::Invite));
    assert_eq!(Link::new("https://h.example", Some(FP), Some("has space here")), Err(Error::Invite));
}

#[test]
fn parser_refuses_hostile_text() {
    let good = link("https://h.example", Some("abcdefgh")).text();
    let cases = [
        ("", Error::NotOurs),
        ("https://h.example", Error::NotOurs),
        ("wprs://connect/2?server=https://h.example", Error::Version),
        ("wprs://connect/x?server=a", Error::NotOurs),
        ("wprs://connect/1?", Error::Fields),
        (&format!("wprs://connect/1?fp={FP}&server=https://h.example"), Error::Fields),
        (&format!("wprs://connect/1?server=https://h.example&fp={FP}&x=1"), Error::Fields),
        (
            &format!("wprs://connect/1?server=https://h.example&fp={FP}&invite=abcdefgh&invite=abcdefgh"),
            Error::Fields,
        ),
        (&format!("wprs://connect/1?server=https://h.example&fp={FP}&"), Error::Fields),
        ("wprs://connect/1?server=https://h.example&", Error::Fields),
        ("wprs://connect/1?server=https://h.example&fp=&invite=abcdefgh", Error::Fingerprint),
        ("wprs://connect/1?server=https://h.example&invite=abcdefgh&fp=0", Error::Fields),
        (&format!("wprs://connect/1?server=https://H.example&fp={FP}"), Error::Server),
        (&format!("wprs://connect/1?server=https://h.example%2f&fp={FP}"), Error::Server),
        (&format!("wprs://connect/1?server=https://h.example&fp={}", FP.to_uppercase()), Error::Fingerprint),
        (&format!("{good}\n"), Error::Invite),
        (&format!("{good} "), Error::Invite),
        (&format!("{good}{}", "a".repeat(300)), Error::TooLong),
        ("wprs://connect/1?server=https://h.example&fp=\u{0}", Error::Fingerprint),
        ("wprs://connect/1?server=\u{202e}&fp=", Error::Server),
    ];
    for (text, error) in cases {
        assert_eq!(Link::parse(text), Err(error), "{text:?}");
    }
}

#[test]
fn parser_never_panics_on_noise() {
    let mut seed = 0x2545_f491_4f6c_dd1d_u64;
    for _ in 0..2000 {
        let mut text = String::from("wprs://connect/1?server=https://h.example&fp=");
        for _ in 0..(seed % 90) {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            text.push(char::from_u32(((seed >> 33) % 0x250) as u32).unwrap_or('?'));
        }
        let _ = Link::parse(&text);
    }
}

#[test]
fn code_decodes_to_the_same_text() {
    for l in [
        link("https://h.example", None),
        link("https://192.168.1.20:8443", Some("0123456789abcdef0123456789abcdef")),
    ] {
        let code = Code::new(&l.text()).unwrap();
        assert_eq!(decode(&code), l.text());
        assert_eq!(Link::parse(&decode(&code)).unwrap(), l);
    }
}

#[test]
fn svg_is_one_colour_and_matches_the_matrix() {
    let code = Code::new("A").unwrap();
    let svg = code.svg();
    assert!(svg.starts_with("<svg ") && svg.ends_with("</svg>"));
    assert!(svg.contains("fill=\"currentColor\"") && svg.contains("viewBox=\"0 0 29 29\""));
    assert!(!svg.contains("rgb") && !svg.contains('#') && !svg.contains("<rect"));
    let area: i32 = svg
        .split('M')
        .skip(1)
        .map(|run| run[run.find('h').unwrap() + 1..run.find('v').unwrap()].parse::<i32>().unwrap())
        .sum();
    let dark = (0..21).flat_map(|y| (0..21).map(move |x| (x, y))).filter(|&(x, y)| code.dark(x, y)).count();
    assert_eq!(area as usize, dark);
}

#[test]
fn blocks_pair_two_rows_and_keep_a_quiet_zone() {
    let code = Code::new("A").unwrap();
    let text = code.blocks(false);
    let lines: Vec<_> = text.lines().collect();
    assert_eq!(lines.len(), 15);
    assert!(lines.iter().all(|l| l.chars().count() == 29));
    assert_eq!(lines[0].trim_matches('█'), "");
    assert_ne!(text, code.blocks(true));
}

#[test]
fn too_much_text_is_refused() {
    assert_eq!(Code::new(&"a".repeat(4000)).err(), Some(Error::TooLong));
}
