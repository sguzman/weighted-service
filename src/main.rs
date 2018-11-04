extern crate actix_web;

fn f(_: actix_web::Path<()>) -> impl actix_web::Responder {
    println!("Hello");
    return format!("<h1>Hello</h1>\n");
}

fn factory() -> actix_web::App {
    let path: &str = "/";
    let method: actix_web::http::Method = actix_web::http::Method::GET;

    return actix_web::App::new()
        .route(path, method, f);
}

fn env(key: &str, default: &str) -> String {
    return match std::env::var(key) {
        Ok(lang) => lang,
        Err(e) => format!("{}", default)
    };
}

fn server_init() {
    let host: String = env("HOST", "0.0.0.0");
    let port: String = env("PORT", "8080");
    let addr: String = format!("{}:{}", host, port);

    actix_web::server::new(factory)
        .bind(addr).unwrap()
        .run();
}

fn main() {
    println!("start");

    server_init();
}