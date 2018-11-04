extern crate rand;

use util;
use postgres;

pub fn server_init(receiver: std::sync::mpsc::Receiver<std::collections::HashSet<String>>) {
    let host: String = util::env("HOST", "0.0.0.0");
    let port: String = util::env("PORT", "8080");
    let addr: String = format!("{}:{}", host, port);

    let f = move |_: actix_web::Path<()>| {
        println!("{}", "Got request");
        let serials = receiver.recv().unwrap();

        println!("{:?}",serials);
        return format!("<h1>{:?}</h1>\n", serials);
    };

    let factory = || {
        let path: &str = "/";
        let method: actix_web::http::Method = actix_web::http::Method::GET;

        return actix_web::App::new()
            .route(path, method, f);
    };

    println!("{}", "start");
    actix_web::server::new(factory)
        .bind(addr).unwrap()
        .run();
}