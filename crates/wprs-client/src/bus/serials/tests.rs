use super::{FIXED, Fixed, Serials, reply_serial_at, set_serial};
use std::num::NonZeroU32;
use zbus::message::Message;
use zbus::wire::Endian;

fn call(serial: u32, endian: Endian) -> Message {
    Message::method_call("/io/github/wallpaper_rs", "State")
        .destination("io.github.wallpaper_rs")
        .interface("io.github.wallpaper_rs.Daemon")
        .serial(NonZeroU32::new(serial).unwrap())
        .endian(endian)
        .build(&("a body", 7u32))
        .unwrap()
}

fn reply(to: &Message, serial: u32, endian: Endian) -> Vec<u8> {
    let header = to.header();
    let built = Message::method_return(&header)
        .serial(NonZeroU32::new(serial).unwrap())
        .endian(endian)
        .build(&"done")
        .unwrap();
    built.data().to_vec()
}

fn error(to: &Message, serial: u32, endian: Endian) -> Vec<u8> {
    let header = to.header();
    let built = Message::error(&header, "org.freedesktop.DBus.Error.Failed")
        .serial(NonZeroU32::new(serial).unwrap())
        .endian(endian)
        .build(&"failed")
        .unwrap();
    built.data().to_vec()
}

fn reply_serial(bytes: &[u8]) -> u32 {
    let at = reply_serial_at(bytes).unwrap();
    let word = [bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]];
    if bytes[0] == b'B' { u32::from_be_bytes(word) } else { u32::from_le_bytes(word) }
}

#[test]
fn the_fixed_header_of_a_call_gives_its_length_and_serial() {
    for order in [Endian::Little, Endian::Big] {
        let message = call(41, order);
        let fixed = Fixed::read(message.data()).unwrap();
        assert_eq!(fixed.serial, 41);
        assert_eq!(fixed.len().unwrap(), message.data().len());
        assert!(fixed.expects_reply());
    }
}

#[test]
fn a_call_that_wants_no_reply_and_a_reply_are_not_remembered() {
    let quiet = Message::method_call("/a", "B")
        .with_flags(zbus::message::Flags::NO_REPLY_EXPECTED)
        .build(&())
        .unwrap();
    let answered = reply(&call(3, Endian::Little), 4, Endian::Little);
    let mut serials = Serials::default();
    serials.renumber(&mut quiet.data()[..FIXED].to_vec()).unwrap();
    serials.renumber(&mut answered[..FIXED].to_vec()).unwrap();
    assert_eq!(serials.remembered(), 0);
}

#[test]
fn renumbering_counts_up_from_one_whatever_the_library_chose() {
    let mut serials = Serials::default();
    let mut wire = Vec::new();
    for original in [900, 7, 899, 8] {
        let mut header = call(original, Endian::Little).data()[..FIXED].to_vec();
        serials.renumber(&mut header).unwrap();
        wire.push(Fixed::read(&header).unwrap().serial);
    }
    assert_eq!(wire, [1, 2, 3, 4]);
    assert_eq!(serials.remembered(), 4);
}

#[test]
fn a_reply_and_an_error_get_back_the_serial_of_their_call() {
    for order in [Endian::Little, Endian::Big] {
        let mut serials = Serials::default();
        let first = call(500, Endian::Little);
        let second = call(77, Endian::Little);
        serials.renumber(&mut first.data()[..FIXED].to_vec()).unwrap();
        serials.renumber(&mut second.data()[..FIXED].to_vec()).unwrap();
        let mut answer = reply(&call(2, order), 10, order);
        let mut failure = error(&call(1, order), 11, order);
        serials.restore(&mut answer);
        serials.restore(&mut failure);
        assert_eq!(reply_serial(&answer), 77);
        assert_eq!(reply_serial(&failure), 500);
        assert_eq!(serials.remembered(), 0);
    }
}

#[test]
fn a_reply_to_a_serial_never_sent_is_left_as_it_is() {
    let mut serials = Serials::default();
    let mut answer = reply(&call(9, Endian::Big), 10, Endian::Big);
    let before = answer.clone();
    serials.restore(&mut answer);
    assert_eq!(answer, before);
}

#[test]
fn the_reply_serial_field_is_found_after_string_fields_in_both_orders() {
    for order in [Endian::Little, Endian::Big] {
        assert_eq!(reply_serial(&reply(&call(12345, order), 3, order)), 12345);
    }
}

#[test]
fn set_serial_writes_in_the_byte_order_of_the_message() {
    for order in [Endian::Little, Endian::Big] {
        let mut header = call(4, order).data()[..FIXED].to_vec();
        set_serial(&mut header, 0x0102_0304);
        assert_eq!(Fixed::read(&header).unwrap().serial, 0x0102_0304);
    }
}

#[test]
fn a_call_has_no_reply_serial_and_a_cut_header_is_refused() {
    let bytes = call(5, Endian::Little).data().to_vec();
    assert_eq!(reply_serial_at(&bytes), None);
    assert!(Fixed::read(&bytes[..FIXED - 1]).is_err());
    let mut strange = bytes.clone();
    strange[0] = b'x';
    assert!(Fixed::read(&strange).is_err());
}
