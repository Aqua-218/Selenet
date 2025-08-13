// SPDX-License-Identifier: MIT
// Command line interface for Selenet
use std::env;

fn print_help() {
    println!("Selenet CLI\n");
    println!("USAGE:\n  selenet <command> [args]\n");
    println!("COMMANDS:\n  encode <text>             Percent-encode input (RFC3986 unreserved as-is)\n  decode <text>             Percent-decode input\n  url encode [--form] <t>   URL encode (RFC3986 or form mode)\n  url decode [--form] <t>   URL decode (RFC3986 or form mode)\n  url parse <url>           Parse URL into components\n  url serialize <url>       Parse then serialize URL\n  help                      Show this help\n");
    println!("日本語:\n  encode <text>             入力をパーセントエンコード（RFC3986 非予約は素通し）\n  decode <text>             入力をパーセントデコード\n  url encode [--form] <t>   URL エンコード（RFC3986/フォーム互換）\n  url decode [--form] <t>   URL デコード（RFC3986/フォーム互換）\n  url parse <url>           URL を解析して構成要素を表示\n  url serialize <url>       URL を解析して正規化して出力\n  help                      このヘルプを表示\n");
}

fn main() {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("encode") => {
            let text = args.next().unwrap_or_default();
            let out = selenet_infra::percent::percent_encode(text.as_bytes(), selenet_infra::percent::is_unreserved_rfc3986);
            println!("{}", out);
        }
        Some("decode") => {
            let text = args.next().unwrap_or_default();
            match selenet_infra::percent::percent_decode(&text) {
                Ok(bytes) => println!("{}", String::from_utf8_lossy(&bytes)),
                Err(e) => {
                    eprintln!("decode error: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some("url") => {
            match args.next().as_deref() {
                Some("encode") => {
                    let mut form = false;
                    let mut rest: Vec<String> = Vec::new();
                    for a in args { if a == "--form" { form = true } else { rest.push(a) } }
                    let text = rest.join(" ");
                    let out = if form {
                        selenet_infra::percent::form_urlencode(text.as_bytes())
                    } else {
                        selenet_infra::percent::percent_encode(text.as_bytes(), selenet_infra::percent::is_unreserved_rfc3986)
                    };
                    println!("{}", out);
                }
                Some("decode") => {
                    let mut form = false;
                    let mut rest: Vec<String> = Vec::new();
                    for a in args { if a == "--form" { form = true } else { rest.push(a) } }
                    let text = rest.join(" ");
                    let res = if form {
                        selenet_infra::percent::form_urldecode(&text)
                    } else {
                        selenet_infra::percent::percent_decode(&text)
                    };
                    match res {
                        Ok(bytes) => println!("{}", String::from_utf8_lossy(&bytes)),
                        Err(e) => { eprintln!("decode error: {}", e); std::process::exit(1); }
                    }
                }
                Some("parse") => {
                    let text = args.collect::<Vec<_>>().join(" ");
                    match selenet_infra::url::Url::parse(&text) {
                        Ok(u) => {
                            println!("scheme: {}", u.scheme);
                            if !u.username.is_empty() { println!("username: {}", u.username); }
                            if let Some(p) = &u.password { println!("password: {}", p); }
                            println!("host: {}", match u.host { selenet_infra::url::Host::Domain(ref d)=>d.clone(), selenet_infra::url::Host::Ipv4(ip)=>ip.to_string(), selenet_infra::url::Host::Ipv6(ip)=>format!("[{}]", ip)});
                            if let Some(port) = u.port { println!("port: {}", port); }
                            if !u.path.is_empty() { println!("path: /{}", u.path.join("/")); }
                            if let Some(q) = u.query { println!("query: {}", q); }
                            if let Some(f) = u.fragment { println!("fragment: {}", f); }
                        }
                        Err(e) => { eprintln!("parse error: {}", e); std::process::exit(1); }
                    }
                }
                Some("serialize") => {
                    let text = args.collect::<Vec<_>>().join(" ");
                    match selenet_infra::url::Url::parse(&text) {
                        Ok(u) => println!("{}", u.serialize()),
                        Err(e) => { eprintln!("parse error: {}", e); std::process::exit(1); }
                    }
                }
                _ => print_help(),
            }
        }
        _ => {
            print_help();
        }
    }
}


