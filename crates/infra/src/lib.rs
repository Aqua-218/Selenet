//! Infrastructure crate for Selenet
//! This crate hosts foundational utilities used across the project.

pub mod ascii;
pub mod bytescanner;
pub mod percent;

/// Returns true if the given byte is an ASCII whitespace per Infra definition (subset placeholder).
/// This is a minimal placeholder and will be extended to match WHATWG Infra precisely.
pub fn is_ascii_whitespace(byte: u8) -> bool {
    matches!(byte, b'\t' | b'\n' | b'\x0C' | b'\r' | b' ')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ascii_whitespace() {
        for b in [b'\t', b'\n', 0x0C, b'\r', b' '] {
            assert!(is_ascii_whitespace(b));
        }
        for b in [b'a', b'0', b'_'] {
            assert!(!is_ascii_whitespace(b));
        }
    }
}


