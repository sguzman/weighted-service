extern crate actix_web;

mod http;

fn env(key: &str, default: &str) -> String {
    return match std::env::var(key) {
        Ok(lang) => lang,
        Err(_) => format!("{}", default)
    };
}

fn main() {
    println!("start");

    http::server_init();
}