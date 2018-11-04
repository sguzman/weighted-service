extern crate actix_web;

mod http;
mod postgres;
mod util;

fn main() {
    println!("start");
    postgres::query();
    //http::server_init();
}