// SPDX-License-Identifier: MIT
//! Minimal UTF-8 validation and code point iterator.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Utf8ErrorKind {
    InvalidStart,
    UnexpectedEof,
    InvalidContinuation,
    Overlong,
    Surrogate,
    OutOfRange,
}

pub fn validate_utf8(bytes: &[u8]) -> Result<(), Utf8ErrorKind> {
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        if b < 0x80 {
            i += 1;
            continue;
        } else if (0xC2..=0xDF).contains(&b) {
            if i + 1 >= bytes.len() { return Err(Utf8ErrorKind::UnexpectedEof); }
            if !is_cont(bytes[i+1]) { return Err(Utf8ErrorKind::InvalidContinuation); }
            i += 2;
        } else if b == 0xE0 {
            if i + 2 >= bytes.len() { return Err(Utf8ErrorKind::UnexpectedEof); }
            let b1 = bytes[i+1]; let b2 = bytes[i+2];
            if !(0xA0..=0xBF).contains(&b1) || !is_cont(b2) { return Err(Utf8ErrorKind::InvalidContinuation); }
            i += 3;
        } else if (0xE1..=0xEC).contains(&b) || (0xEE..=0xEF).contains(&b) {
            if i + 2 >= bytes.len() { return Err(Utf8ErrorKind::UnexpectedEof); }
            if !is_cont(bytes[i+1]) || !is_cont(bytes[i+2]) { return Err(Utf8ErrorKind::InvalidContinuation); }
            i += 3;
        } else if b == 0xED {
            if i + 2 >= bytes.len() { return Err(Utf8ErrorKind::UnexpectedEof); }
            // 0xED 0xA0..0xBF => surrogates U+D800..U+DFFF (invalid)
            let b1 = bytes[i+1]; let b2 = bytes[i+2];
            if !(0x80..=0x9F).contains(&b1) || !is_cont(b2) { return Err(Utf8ErrorKind::InvalidContinuation); }
            return Err(Utf8ErrorKind::Surrogate);
        } else if b == 0xF0 {
            if i + 3 >= bytes.len() { return Err(Utf8ErrorKind::UnexpectedEof); }
            let b1 = bytes[i+1]; let b2 = bytes[i+2]; let b3 = bytes[i+3];
            if !(0x90..=0xBF).contains(&b1) || !is_cont(b2) || !is_cont(b3) { return Err(Utf8ErrorKind::InvalidContinuation); }
            i += 4;
        } else if (0xF1..=0xF3).contains(&b) {
            if i + 3 >= bytes.len() { return Err(Utf8ErrorKind::UnexpectedEof); }
            if !is_cont(bytes[i+1]) || !is_cont(bytes[i+2]) || !is_cont(bytes[i+3]) { return Err(Utf8ErrorKind::InvalidContinuation); }
            i += 4;
        } else if b == 0xF4 {
            if i + 3 >= bytes.len() { return Err(Utf8ErrorKind::UnexpectedEof); }
            let b1 = bytes[i+1]; let b2 = bytes[i+2]; let b3 = bytes[i+3];
            if !(0x80..=0x8F).contains(&b1) || !is_cont(b2) || !is_cont(b3) { return Err(Utf8ErrorKind::InvalidContinuation); }
            i += 4;
        } else {
            return Err(Utf8ErrorKind::InvalidStart);
        }
    }
    Ok(())
}

#[inline]
fn is_cont(b: u8) -> bool { (b & 0xC0) == 0x80 }

pub struct CodePoints<'a> {
    bytes: &'a [u8],
    i: usize,
}

impl<'a> CodePoints<'a> {
    pub fn new(bytes: &'a [u8]) -> Self { Self { bytes, i: 0 } }
}

impl<'a> Iterator for CodePoints<'a> {
    type Item = Result<char, Utf8ErrorKind>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.bytes.len() { return None; }
        let b = self.bytes[self.i];
        if b < 0x80 { self.i += 1; return Some(Ok(b as char)); }
        // For brevity, reuse validate logic by slicing; in perf paths, a fused decoder would be used.
        for len in [2usize,3,4] {
            if self.i + len <= self.bytes.len() {
                if validate_utf8(&self.bytes[self.i..self.i+len]).is_ok() {
                    let ch = std::str::from_utf8(&self.bytes[self.i..self.i+len]).ok()?.chars().next().unwrap();
                    self.i += len;
                    return Some(Ok(ch));
                }
            }
        }
        Some(Err(Utf8ErrorKind::InvalidStart))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_sequences() {
        assert!(validate_utf8("hello".as_bytes()).is_ok());
        assert!(validate_utf8("こんにちは".as_bytes()).is_ok());
    }

    #[test]
    fn invalid_sequences() {
        assert_eq!(validate_utf8(&[0xC0, 0x80]).unwrap_err(), Utf8ErrorKind::Overlong.or(Utf8ErrorKind::InvalidStart));
    }
}


