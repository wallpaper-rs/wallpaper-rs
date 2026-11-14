pub(crate) fn read(luma: &[u8], width: usize, height: usize, stride: usize) -> Option<String> {
    wprs_qr::read(luma, width, height, stride)
}

#[cfg(test)]
mod tests {
    use super::read;
    use crate::connect::parse;
    use wprs_qr::{Code, Link};

    const FP: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    const SIDE: usize = 480;
    const STRIDE: usize = 512;

    fn ours() -> String {
        Link::new("https://casa.example:8443", Some(FP), Some("abcdefgh")).unwrap().text()
    }

    fn picture(text: &str, module: f32, turn: f32) -> Vec<u8> {
        let code = Code::new(text).unwrap();
        let half = code.size() as f32 / 2.0;
        let (sin, cos) = turn.to_radians().sin_cos();
        let mut frame = vec![0u8; STRIDE * SIDE];
        for y in 0..SIDE {
            for x in 0..SIDE {
                let (dx, dy) = (x as f32 - SIDE as f32 / 2.0, y as f32 - SIDE as f32 / 2.0);
                let (u, v) = ((dx * cos + dy * sin) / module + half, (dy * cos - dx * sin) / module + half);
                let dark = code.dark(u.floor() as i32, v.floor() as i32);
                let paper = 210 + ((x * 7 + y * 13) % 11) as u8;
                frame[y * STRIDE + x] = if dark { 35 + ((x + y) % 7) as u8 } else { paper };
            }
        }
        frame
    }

    fn seen(frame: &[u8]) -> Option<String> {
        read(frame, SIDE, SIDE, STRIDE)
    }

    #[test]
    fn a_straight_code_gives_the_server_and_its_fingerprint() {
        let text = seen(&picture(&ours(), 5.0, 0.0)).unwrap();
        let got = parse(&text).unwrap();
        assert_eq!(
            (got["server"].as_str(), got["fingerprint"].as_str()),
            (Some("https://casa.example:8443"), Some(FP))
        );
        assert_eq!(got["invite"], "abcdefgh");
    }

    #[test]
    fn a_tilted_code_is_read() {
        for turn in [12.0, -25.0, 40.0] {
            assert_eq!(seen(&picture(&ours(), 4.5, turn)).as_deref(), Some(ours().as_str()), "turned {turn}");
        }
    }

    #[test]
    fn a_small_code_is_read() {
        assert_eq!(seen(&picture(&ours(), 3.0, 0.0)).as_deref(), Some(ours().as_str()));
    }

    #[test]
    fn a_code_of_something_else_is_read_and_refused_as_not_ours() {
        let text = seen(&picture("https://example.org/menu", 5.0, 0.0)).unwrap();
        let reply: serde_json::Value = serde_json::from_str(&crate::connect::reply(&text)).unwrap();
        assert_eq!(reply["id"], "android.connect_not_ours");
    }

    #[test]
    fn a_frame_without_a_code_or_too_short_gives_nothing() {
        let blank: Vec<u8> = (0..STRIDE * SIDE).map(|i| (i * 31 % 251) as u8).collect();
        assert_eq!(seen(&blank), None);
        assert_eq!(read(&blank[..100], SIDE, SIDE, STRIDE), None);
        assert_eq!(read(&blank, SIDE, SIDE, SIDE - 1), None);
        assert_eq!(read(&blank, SIDE, 0, STRIDE), None);
    }

    #[test]
    fn a_damaged_code_gives_nothing() {
        let mut frame = picture(&ours(), 5.0, 0.0);
        for y in 180..300 {
            frame[y * STRIDE + 150..y * STRIDE + 330].fill(255);
        }
        assert_eq!(seen(&frame), None);
    }
}
