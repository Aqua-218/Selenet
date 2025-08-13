// SPDX-License-Identifier: MIT
//! Minimal URL structure and parser/serializer for absolute HTTP(S) URLs.
//! This is a conservative implementation intended to bootstrap further work.
//! - Supports: scheme, userinfo, host (domain/IPv4/IPv6), port, path, query, fragment
//! - Limitations: No relative-URL resolution, no special schemes beyond http/https, no IDNA/Punycode yet

use std::fmt::{Display, Formatter};
use std::net::{Ipv4Addr, Ipv6Addr};

use crate::percent;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Host {
    Domain(String),
    Ipv4(Ipv4Addr),
    Ipv6(Ipv6Addr),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Url {
    pub scheme: String,
    pub username: String,
    pub password: Option<String>,
    pub host: Host,
    pub port: Option<u16>,
    pub path: Vec<String>,
    pub query: Option<String>,
    pub fragment: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UrlParseError {
    MissingScheme,
    InvalidScheme,
    MissingAuthority,
    InvalidUserinfo,
    InvalidPort,
    InvalidHost,
    InvalidPercent,
}

impl Display for UrlParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use UrlParseError::*;
        match self {
            MissingScheme => write!(f, "missing scheme"),
            InvalidScheme => write!(f, "invalid scheme"),
            MissingAuthority => write!(f, "missing authority"),
            InvalidUserinfo => write!(f, "invalid userinfo"),
            InvalidPort => write!(f, "invalid port"),
            InvalidHost => write!(f, "invalid host"),
            InvalidPercent => write!(f, "invalid percent-encoding"),
        }
    }
}

impl std::error::Error for UrlParseError {}

impl Url {
    /// Parse an absolute URL (http/https) into components.
    pub fn parse(input: &str) -> Result<Self, UrlParseError> {
        // Split scheme
        let (scheme, rest) = split_once(input, ':').ok_or(UrlParseError::MissingScheme)?;
        if !is_valid_scheme(scheme) {
            return Err(UrlParseError::InvalidScheme);
        }

        // Expect //authority...
        let rest = rest.strip_prefix("//").ok_or(UrlParseError::MissingAuthority)?;

        // Split off fragment
        let (before_frag, fragment) = split_once(rest, '#').map_or((rest, None), |(a, b)| (a, Some(b.to_string())));
        // Split off query
        let (before_query, query) = split_once(before_frag, '?').map_or((before_frag, None), |(a, b)| (a, Some(b.to_string())));

        // Authority + path
        let (authority, path_str) = if let Some((a, p)) = split_once(before_query, '/') {
            (a, Some(p))
        } else {
            (before_query, None)
        };

        // userinfo@host:port
        let (userinfo_opt, hostport) = if let Some((u, h)) = split_once(authority, '@') {
            (Some(u), h)
        } else {
            (None, authority)
        };

        // Parse userinfo
        let (username, password) = if let Some(ui) = userinfo_opt {
            if let Some((u, p)) = split_once(ui, ':') {
                (u.to_string(), Some(p.to_string()))
            } else {
                (ui.to_string(), None)
            }
        } else {
            (String::new(), None)
        };

        // Parse host and port
        let (host, port) = parse_host_port(hostport)?;

        // Parse path segments (percent-decoded)
        let path: Vec<String> = if let Some(p) = path_str {
            if p.is_empty() { Vec::new() } else {
                p.split('/')
                    .map(|seg| percent::percent_decode(seg).map_err(|_| UrlParseError::InvalidPercent)
                        .and_then(|v| String::from_utf8(v).map_err(|_| UrlParseError::InvalidPercent)))
                    .collect::<Result<Vec<_>, _>>()?
            }
        } else { Vec::new() };

        Ok(Url {
            scheme: scheme.to_string(),
            username,
            password,
            host,
            port,
            path,
            query,
            fragment,
        })
    }

    /// Serialize URL back to string form with percent-encoding for path segments.
    pub fn serialize(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.scheme);
        s.push(':');
        s.push_str("//");
        if !self.username.is_empty() {
            s.push_str(&self.username);
            if let Some(pw) = &self.password { s.push(':'); s.push_str(pw); }
            s.push('@');
        }
        match &self.host {
            Host::Domain(d) => s.push_str(d),
            Host::Ipv4(ip) => s.push_str(&ip.to_string()),
            Host::Ipv6(ip) => { s.push('['); s.push_str(&ip.to_string()); s.push(']'); }
        }
        if let Some(port) = self.port { s.push(':'); s.push_str(&port.to_string()); }
        if !self.path.is_empty() {
            s.push('/');
            let mut first = true;
            for seg in &self.path {
                if !first { s.push('/'); }
                first = false;
                let enc = percent::percent_encode(seg.as_bytes(), percent::is_unreserved_rfc3986);
                s.push_str(&enc);
            }
        }
        if let Some(q) = &self.query { s.push('?'); s.push_str(q); }
        if let Some(f) = &self.fragment { s.push('#'); s.push_str(f); }
        s
    }
}

fn split_once<'a>(input: &'a str, delim: char) -> Option<(&'a str, &'a str)> {
    input.split_once(delim)
}

fn is_valid_scheme(s: &str) -> bool {
    let mut chars = s.chars();
    match chars.next() { Some(c) if c.is_ascii_alphabetic() => {}, _ => return false }
    for c in chars {
        if !(c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.')) { return false }
    }
    true
}

fn parse_host_port(input: &str) -> Result<(Host, Option<u16>), UrlParseError> {
    if let Some(host) = input.strip_prefix('[') {
        // IPv6 literal [..]
        let (inside, rest) = split_once(host, ']').ok_or(UrlParseError::InvalidHost)?;
        let ipv6 = inside.parse::<Ipv6Addr>().map_err(|_| UrlParseError::InvalidHost)?;
        let port = if let Some(rest) = rest.strip_prefix(':') {
            Some(parse_port(rest)?)
        } else { None };
        return Ok((Host::Ipv6(ipv6), port));
    }
    // Split host:port optionally
    let (h, p) = if let Some((h, p)) = split_once(input, ':') { (h, Some(p)) } else { (input, None) };
    // Try IPv4
    if let Ok(ip4) = h.parse::<Ipv4Addr>() {
        let port = if let Some(p) = p { Some(parse_port(p)?) } else { None };
        return Ok((Host::Ipv4(ip4), port));
    }
    // Domain
    let host = h.to_string();
    if host.is_empty() { return Err(UrlParseError::InvalidHost); }
    let port = if let Some(p) = p { Some(parse_port(p)?) } else { None };
    Ok((Host::Domain(host), port))
}

fn parse_port(p: &str) -> Result<u16, UrlParseError> {
    p.parse::<u16>().map_err(|_| UrlParseError::InvalidPort)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_http_basic() {
        let u = Url::parse("http://example.com/").unwrap();
        assert_eq!(u.scheme, "http");
        assert_eq!(u.username, "");
        assert!(u.password.is_none());
        assert_eq!(u.host, Host::Domain("example.com".into()));
        assert!(u.port.is_none());
        assert!(u.path.is_empty());
        assert!(u.query.is_none());
        assert!(u.fragment.is_none());
        assert_eq!(u.serialize(), "http://example.com");
    }

    #[test]
    fn parse_with_userinfo_ipv6_and_query_fragment() {
        let u = Url::parse("https://user:pw@[2001:db8::1]:8443/a/b%20c?x=1#frag").unwrap();
        assert_eq!(u.scheme, "https");
        assert_eq!(u.username, "user");
        assert_eq!(u.password.as_deref(), Some("pw"));
        assert!(matches!(u.host, Host::Ipv6(_)));
        assert_eq!(u.port, Some(8443));
        assert_eq!(u.path, vec!["a".to_string(), "b c".to_string()]);
        assert_eq!(u.query.as_deref(), Some("x=1"));
        assert_eq!(u.fragment.as_deref(), Some("frag"));
        assert_eq!(u.serialize(), "https://user:pw@[2001:db8::1]:8443/a/b%20c?x=1#frag");
    }

    #[test]
    fn parse_ipv4_with_port() {
        let u = Url::parse("http://192.168.0.1:8080").unwrap();
        assert!(matches!(u.host, Host::Ipv4(_)));
        assert_eq!(u.port, Some(8080));
        assert_eq!(u.serialize(), "http://192.168.0.1:8080");
    }
}


