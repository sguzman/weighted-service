extern crate actix_web;

mod http;
mod util;


fn main() {
    println!("start");

    http::server_init();
}