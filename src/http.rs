extern crate rand;

use util;
use postgres;

fn get_random(items: &mut Vec<rand::distributions::Weighted<String>>) -> String {
    use http::rand::distributions::IndependentSample;
    use http::rand::distributions::Distribution;

    let wc: rand::distributions::WeightedChoice<String> = rand::distributions::WeightedChoice::new(items);
    let mut rng = rand::thread_rng();
    let serial: String = wc.ind_sample(&mut rng);

    return serial;
}

pub fn server_init() {
    let host: String = util::env("HOST", "0.0.0.0");
    let port: String = util::env("PORT", "8080");
    let addr: String = format!("{}:{}", host, port);

    let factory = move || {
        let path: &str = "/";
        let method: actix_web::http::Method = actix_web::http::Method::GET;

        let (sender, receiver) = std::sync::mpsc::sync_channel(0);
        std::thread::spawn(move|| {
            let mut items: Vec<rand::distributions::Weighted<String>> = postgres::channels();
            let mut set = std::collections::HashSet::new();
            loop {
                while set.len() < 50 {
                    set.insert(get_random(&mut items));
                }
                sender.send(set.clone()).unwrap();
                set.clear();
            }
        });

        let f = move |_: actix_web::Path<()>| {
            let serials = receiver.recv().unwrap();

            println!("{:?}",serials);
            return format!("<h1>{:?}</h1>\n", serials);
        };

        return actix_web::App::new()
            .route(path, method, f);
    };

    println!("{}", "start");
    actix_web::server::new(factory)
        .bind(addr).unwrap()
        .run();
}