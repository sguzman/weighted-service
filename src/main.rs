extern crate actix_web;

mod http;
mod postgres;
mod util;

fn main() {
    http::server_init();
}