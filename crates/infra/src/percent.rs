// SPDX-License-Identifier: MIT
// Copyright (c) SeleniaProject

//! Percent-encoding utilities compatible with URL-like use cases.
//! This module provides generic helpers; URL component specific rules can be built on top.

use crate::ascii;

/// Percent-encode bytes using the provided allowlist predicate.
/// Bytes for which `is_unreserved` returns true will be emitted as-is; others are %HH-encoded.
pub fn percent_encode<F>(input: &[u8], mut is_unreserved: F) -> String
where
    F: FnMut(u8) -> bool,
{
    let mut out = String::with_capacity(input.len() * 3);
    for &b in input {
        if is_unreserved(b) {
            out.push(b as char);
        } else {
            out.push('%');
            const HEX: &[u8; 16] = b"0123456789ABCDEF";
            out.push(HEX[(b >> 4) as usize] as char);
            out.push(HEX[(b & 0x0F) as usize] as char);
        }
    }
    out
}

/// Percent-decode a string to raw bytes.
/// Returns an error if the input contains an invalid percent triplet.
pub fn percent_decode(input: &str) -> Result<Vec<u8>, String> {
    let bytes = input.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'%' => {
                if i + 2 >= bytes.len() {
                    return Err("truncated percent escape".to_string());
                }
                let hi = ascii::hex_value(bytes[i + 1]).ok_or_else(|| "invalid hex in percent escape".to_string())?;
                let lo = ascii::hex_value(bytes[i + 2]).ok_or_else(|| "invalid hex in percent escape".to_string())?;
                out.push((hi << 4) | lo);
                i += 3;
            }
            b => {
                out.push(b);
                i += 1;
            }
        }
    }
    Ok(out)
}

/// RFC3986 unreserved: ALPHA / DIGIT / "-" / "." / "_" / "~"
pub fn is_unreserved_rfc3986(byte: u8) -> bool {
    ascii::is_ascii_alphanumeric(byte) || matches!(byte, b'-' | b'.' | b'_' | b'~')
}

/// x-www-form-urlencoded style encode (space -> '+', others percent-encoded unless unreserved).
pub fn form_urlencode(input: &[u8]) -> String {
    let mut out = String::with_capacity(input.len() * 3);
    for &b in input {
        match b {
            b' ' => out.push('+'),
            b if is_unreserved_rfc3986(b) => out.push(b as char),
            b => {
                out.push('%');
                const HEX: &[u8; 16] = b"0123456789ABCDEF";
                out.push(HEX[(b >> 4) as usize] as char);
                out.push(HEX[(b & 0x0F) as usize] as char);
            }
        }
    }
    out
}

/// x-www-form-urlencoded style decode ('+' -> space, %HH handled; returns error on invalid triplet).
pub fn form_urldecode(input: &str) -> Result<Vec<u8>, String> {
    let bytes = input.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            b'%' => {
                if i + 2 >= bytes.len() {
                    return Err("truncated percent escape".to_string());
                }
                let hi = ascii::hex_value(bytes[i + 1]).ok_or_else(|| "invalid hex in percent escape".to_string())?;
                let lo = ascii::hex_value(bytes[i + 2]).ok_or_else(|| "invalid hex in percent escape".to_string())?;
                out.push((hi << 4) | lo);
                i += 3;
            }
            b => {
                out.push(b);
                i += 1;
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_basic() {
        let s = "hello world!";
        let enc = percent_encode(s.as_bytes(), is_unreserved_rfc3986);
        assert_eq!(enc, "hello%20world%21");
    }

    #[test]
    fn decode_basic() {
        let dec = percent_decode("hello%20world%21").unwrap();
        assert_eq!(String::from_utf8(dec).unwrap(), "hello world!");
    }

    #[test]
    fn decode_error() {
        assert!(percent_decode("%G0").is_err());
        assert!(percent_decode("%0").is_err());
    }

    #[test]
    fn form_urlencoding_roundtrip() {
        let src = "a b+c%";
        let enc = form_urlencode(src.as_bytes());
        assert_eq!(enc, "a+b%2Bc%25");
        let dec = form_urldecode(&enc).unwrap();
        assert_eq!(String::from_utf8(dec).unwrap(), src);
    }
}


