#[cfg(test)]
mod tests;

use std::collections::{HashMap, VecDeque};
use std::io;

pub const FIXED: usize = 16;
const LARGEST: usize = 128 * 1024 * 1024;
const REMEMBERED: usize = 4096;
const REPLY_SERIAL: u8 = 5;
const METHOD_CALL: u8 = 1;
const NO_REPLY_EXPECTED: u8 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Fixed {
    pub big: bool,
    pub kind: u8,
    pub flags: u8,
    pub body: u32,
    pub serial: u32,
    pub fields: u32,
}

impl Fixed {
    pub fn read(bytes: &[u8]) -> io::Result<Self> {
        let header = bytes.get(..FIXED).ok_or_else(|| invalid("a message header is incomplete"))?;
        let big = match header[0] {
            b'l' => false,
            b'B' => true,
            _ => return Err(invalid("a message has an unknown byte order")),
        };
        let fixed = Self {
            big,
            kind: header[1],
            flags: header[2],
            body: word(big, &header[4..8]),
            serial: word(big, &header[8..12]),
            fields: word(big, &header[12..16]),
        };
        fixed.len()?;
        Ok(fixed)
    }

    pub fn len(&self) -> io::Result<usize> {
        let total = align(FIXED + self.fields as usize, 8) + self.body as usize;
        if total > LARGEST {
            return Err(invalid("a message is larger than the bus allows"));
        }
        Ok(total)
    }

    pub fn expects_reply(&self) -> bool {
        self.kind == METHOD_CALL && self.flags & NO_REPLY_EXPECTED == 0
    }
}

pub fn set_serial(header: &mut [u8], serial: u32) {
    put(header[0] == b'B', &mut header[8..12], serial);
}

pub fn reply_serial_at(message: &[u8]) -> Option<usize> {
    let fixed = Fixed::read(message).ok()?;
    let end = FIXED.checked_add(fixed.fields as usize)?;
    let fields = message.get(..end)?;
    let mut at = FIXED;
    loop {
        at = align(at, 8);
        if at >= end {
            return None;
        }
        let code = *fields.get(at)?;
        let signature = *fields.get(at + 1)? as usize;
        if signature != 1 {
            return None;
        }
        let kind = *fields.get(at + 2)?;
        at += 4;
        match kind {
            b'u' => {
                at = align(at, 4);
                fields.get(at..at + 4)?;
                if code == REPLY_SERIAL {
                    return Some(at);
                }
                at += 4;
            }
            b's' | b'o' => {
                at = align(at, 4);
                let len = word(fixed.big, fields.get(at..at + 4)?) as usize;
                at = at.checked_add(4 + len + 1)?;
            }
            b'g' => {
                let len = *fields.get(at)? as usize;
                at += 1 + len + 1;
            }
            _ => return None,
        }
    }
}

#[derive(Debug, Default)]
pub struct Serials {
    last: u32,
    sent: HashMap<u32, u32>,
    order: VecDeque<u32>,
}

impl Serials {
    pub fn renumber(&mut self, header: &mut [u8]) -> io::Result<Fixed> {
        let fixed = Fixed::read(header)?;
        self.last = self.last.checked_add(1).unwrap_or(1);
        let wire = self.last;
        set_serial(header, wire);
        if fixed.expects_reply() {
            if self.order.len() >= REMEMBERED
                && let Some(oldest) = self.order.pop_front()
            {
                self.sent.remove(&oldest);
            }
            self.order.push_back(wire);
            self.sent.insert(wire, fixed.serial);
        }
        Ok(fixed)
    }

    pub fn restore(&mut self, message: &mut [u8]) {
        let Some(at) = reply_serial_at(message) else { return };
        let big = message[0] == b'B';
        let wire = word(big, &message[at..at + 4]);
        if let Some(original) = self.sent.remove(&wire) {
            put(big, &mut message[at..at + 4], original);
        }
    }

    #[cfg(test)]
    pub fn remembered(&self) -> usize {
        self.sent.len()
    }
}

fn align(at: usize, to: usize) -> usize {
    at.div_ceil(to) * to
}

fn word(big: bool, bytes: &[u8]) -> u32 {
    let bytes = [bytes[0], bytes[1], bytes[2], bytes[3]];
    if big { u32::from_be_bytes(bytes) } else { u32::from_le_bytes(bytes) }
}

fn put(big: bool, bytes: &mut [u8], value: u32) {
    bytes.copy_from_slice(&if big { value.to_be_bytes() } else { value.to_le_bytes() });
}

fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.to_string())
}
