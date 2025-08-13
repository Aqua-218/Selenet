use std::env;

fn print_help() {
    println!("Selenet CLI\n");
    println!("USAGE:\n  selenet <command> [args]\n");
    println!("COMMANDS:\n  encode <text>   Percent-encode input (RFC3986 unreserved stays as-is)\n  decode <text>   Percent-decode input\n  help            Show this help\n");
    println!("日本語:\n  encode <text>   入力をパーセントエンコード（RFC3986 非予約は素通し）\n  decode <text>   入力をパーセントデコード\n  help            このヘルプを表示\n");
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
        _ => {
            print_help();
        }
    }
}


