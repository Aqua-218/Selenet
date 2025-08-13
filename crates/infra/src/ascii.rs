/// ASCII utilities aligned with WHATWG Infra behaviors where applicable.
/// Note: These functions operate on ASCII bytes only. Callers must validate input range when needed.

/// Returns true when the byte is in 'A'..='Z'.
#[inline]
pub fn is_ascii_uppercase(byte: u8) -> bool {
  (b'A'..=b'Z').contains(&byte)
}

/// Returns true when the byte is in 'a'..='z'.
#[inline]
pub fn is_ascii_lowercase(byte: u8) -> bool {
  (b'a'..=b'z').contains(&byte)
}

/// Returns true when the byte is an ASCII alphabetic character.
#[inline]
pub fn is_ascii_alpha(byte: u8) -> bool {
  is_ascii_uppercase(byte) || is_ascii_lowercase(byte)
}

/// Returns true when the byte is in '0'..='9'.
#[inline]
pub fn is_ascii_digit(byte: u8) -> bool {
  (b'0'..=b'9').contains(&byte)
}

/// Returns true when the byte is alphanumeric.
#[inline]
pub fn is_ascii_alphanumeric(byte: u8) -> bool {
  is_ascii_alpha(byte) || is_ascii_digit(byte)
}

/// Returns true when the byte is a hexadecimal digit (0-9, A-F, a-f).
#[inline]
pub fn is_ascii_hex_digit(byte: u8) -> bool {
  is_ascii_digit(byte)
    || (b'a'..=b'f').contains(&byte)
    || (b'A'..=b'F').contains(&byte)
}

/// Convert uppercase ASCII letter to lowercase; otherwise return unchanged.
#[inline]
pub fn to_ascii_lowercase(byte: u8) -> u8 {
  if is_ascii_uppercase(byte) {
    byte + 32
  } else {
    byte
  }
}

/// Convert lowercase ASCII letter to uppercase; otherwise return unchanged.
#[inline]
pub fn to_ascii_uppercase(byte: u8) -> u8 {
  if is_ascii_lowercase(byte) {
    byte - 32
  } else {
    byte
  }
}

/// Parse a single ASCII hex digit to its numeric value.
/// Returns None if the byte is not a valid hex digit.
#[inline]
pub fn hex_value(byte: u8) -> Option<u8> {
  match byte {
    b'0'..=b'9' => Some(byte - b'0'),
    b'a'..=b'f' => Some(10 + (byte - b'a')),
    b'A'..=b'F' => Some(10 + (byte - b'A')),
    _ => None,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn classify_ascii() {
    assert!(is_ascii_uppercase(b'Z'));
    assert!(is_ascii_lowercase(b'a'));
    assert!(is_ascii_alpha(b'Q'));
    assert!(is_ascii_digit(b'7'));
    assert!(is_ascii_alphanumeric(b'0'));
    assert!(is_ascii_hex_digit(b'F'));
    assert!(is_ascii_hex_digit(b'f'));
    assert!(!is_ascii_hex_digit(b'g'));
  }

  #[test]
  fn case_mapping() {
    assert_eq!(to_ascii_lowercase(b'A'), b'a');
    assert_eq!(to_ascii_uppercase(b'z'), b'Z');
    assert_eq!(to_ascii_lowercase(b'!'), b'!');
  }

  #[test]
  fn hex_values() {
    assert_eq!(hex_value(b'0'), Some(0));
    assert_eq!(hex_value(b'9'), Some(9));
    assert_eq!(hex_value(b'a'), Some(10));
    assert_eq!(hex_value(b'F'), Some(15));
    assert_eq!(hex_value(b'X'), None);
  }
}


