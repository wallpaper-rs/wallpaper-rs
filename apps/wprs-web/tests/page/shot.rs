use super::helper::{Page, available, start};
use std::os::fd::OwnedFd;
use std::os::unix::fs::FileExt;
use std::time::{Duration, Instant};
use wprs_web::{FromWeb, Isolation, Start, ToWeb};

#[test]
fn a_frame_is_copied_on_request_from_the_page_as_it_is_shown() {
    if !available() {
        return eprintln!("skipped: no WPE WebKit 2.54");
    }
    let dir = tempfile::tempdir().unwrap();
    let start = Start { device: None, storage: None, mask_gpu: true, ..start(dir.path(), 9) };
    let mut page = Page::open(start, Isolation { network: true, devices: true, limits: false });
    let mut ring = None::<std::fs::File>;
    let next = |page: &mut Page, ring: &mut Option<std::fs::File>, until: Instant| {
        while Instant::now() < until {
            match page.link.recv::<FromWeb>(false) {
                Ok(Some((FromWeb::Pixels { view, slot, size, stride, .. }, fds))) => {
                    let fds: Vec<OwnedFd> = fds;
                    fds.into_iter().next().into_iter().for_each(|fd| *ring = Some(std::fs::File::from(fd)));
                    page.link.send(&ToWeb::Freed { view, slot }, &[]).unwrap();
                    return Some((view, slot, size, stride));
                }
                Ok(Some((FromWeb::Frame { view, buffer }, _))) => {
                    page.link.send(&ToWeb::Released { view, buffer }, &[]).unwrap();
                }
                Ok(Some(_)) => {}
                Ok(None) => panic!("the helper stopped"),
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(Duration::from_millis(5))
                }
                Err(e) => panic!("{e}"),
            }
        }
        None
    };
    let (view, ..) = next(&mut page, &mut ring, Instant::now() + Duration::from_secs(20)).expect("no frame");
    page.link.send(&ToWeb::Visible { view, visible: false }, &[]).unwrap();
    assert_eq!(page.wait("paused", 5), true);
    while next(&mut page, &mut ring, Instant::now() + Duration::from_millis(1500)).is_some() {}
    page.link.send(&ToWeb::Shot { view }, &[]).unwrap();
    let found = next(&mut page, &mut ring, Instant::now() + Duration::from_secs(10))
        .map(|(_, slot, size, stride)| (slot, size, stride));
    let (slot, size, stride) = found.expect("no frame came after the request");
    assert_eq!(size, (320, 180));
    let mut row = vec![0u8; size.0 as usize * 4];
    let at = u64::from(slot) * u64::from(stride) * u64::from(size.1);
    ring.expect("no frame buffer").read_exact_at(&mut row, at).unwrap();
    assert!(row.iter().skip(3).step_by(4).all(|a| *a == 255), "the copied frame is not opaque: {:?}", &row[..8]);
    assert!(row.iter().any(|b| *b != 0), "the copied frame is empty");
}
