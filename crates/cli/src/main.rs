// SPDX-License-Identifier: MIT
// Command line interface for Selenet
use std::env;

fn print_help() {
    println!("Selenet CLI\n");
    println!("USAGE:\n  selenet <command> [args]\n");
    println!("COMMANDS:\n  encode <text>             Percent-encode input (RFC3986 unreserved as-is)\n  decode <text>             Percent-decode input\n  url encode [--form] <t>   URL encode (RFC3986 or form mode)\n  url decode [--form] <t>   URL decode (RFC3986 or form mode)\n  help                      Show this help\n");
    println!("日本語:\n  encode <text>             入力をパーセントエンコード（RFC3986 非予約は素通し）\n  decode <text>             入力をパーセントデコード\n  url encode [--form] <t>   URL エンコード（RFC3986/フォーム互換）\n  url decode [--form] <t>   URL デコード（RFC3986/フォーム互換）\n  help                      このヘルプを表示\n");
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
                _ => print_help(),
            }
        }
        _ => {
            print_help();
        }
    }
}


