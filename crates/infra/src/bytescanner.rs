// SPDX-License-Identifier: MIT
//! Simple forward byte scanner utility.

/// A lightweight forward-only scanner over a byte slice.
#[derive(Clone, Debug)]
pub struct ByteScanner<'a> {
    bytes: &'a [u8],
    index: usize,
}

impl<'a> ByteScanner<'a> {
    /// Create a new scanner.
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, index: 0 }
    }

    /// Peek the current byte without advancing.
    pub fn peek(&self) -> Option<u8> {
        self.bytes.get(self.index).copied()
    }

    /// Advance and return the next byte.
    pub fn next(&mut self) -> Option<u8> {
        if self.index >= self.bytes.len() {
            return None;
        }
        let b = self.bytes[self.index];
        self.index += 1;
        Some(b)
    }

    /// Returns true if there are no more bytes.
    pub fn is_eof(&self) -> bool {
        self.index >= self.bytes.len()
    }

    /// Current position index.
    pub fn position(&self) -> usize {
        self.index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scanning() {
        let mut s = ByteScanner::new(b"abc");
        assert_eq!(s.peek(), Some(b'a'));
        assert_eq!(s.next(), Some(b'a'));
        assert_eq!(s.position(), 1);
        assert_eq!(s.next(), Some(b'b'));
        assert_eq!(s.next(), Some(b'c'));
        assert!(s.is_eof());
        assert_eq!(s.next(), None);
    }
}


