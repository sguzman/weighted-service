use util;

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

pub fn server_init() {
    let host: String = env("HOST", "0.0.0.0");
    let port: String = env("PORT", "8080");
    let addr: String = format!("{}:{}", host, port);

    actix_web::server::new(factory)
        .bind(addr).unwrap()
        .run();
}