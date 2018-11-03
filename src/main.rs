extern crate actix_web;

fn f(_: actix_web::Path<()>) -> impl actix_web::Responder {
    println!("Hello");
    return format!("<h1>Hello</h1>\n");
}

fn main() {
    fn factory() -> actix_web::App {
        let path: &str = "/";
        let method: actix_web::http::Method = actix_web::http::Method::GET;
        return actix_web::App::new()
        .route(path, method, f);
    }

    let addr: &str = "127.0.0.1:8080";
    actix_web::server::new(factory)
        .bind(addr).unwrap()
        .run();
}