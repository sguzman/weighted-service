extern crate postgres;
extern crate rand;

use util;

fn conn_str() -> String {
    let db_host: String = util::env("DB_HOST", "192.168.1.63");
    let db_port: String = util::env("DB_PORT", "30000");

    return format!("postgresql://postgres:@{}:{}/youtube", db_host, db_port);
}

fn conn() -> postgres::Connection {
    let tls = postgres::TlsMode::None;
    let params: String = conn_str();

    return postgres::Connection::connect(params, tls).unwrap();
}

pub fn channels() -> Vec<rand::distributions::Weighted<String>> {
    let c: postgres::Connection = conn();
    let query: &str = "select serial, subs from youtube.entities.chan_stats where (serial, time) in (select serial, max(time) from youtube.entities.chan_stats group by serial)";

    let mut vec = Vec::new();
    for row in &c.query(query, &[]).unwrap() {
        let serial: String = row.get(0);
        let subs: u32 = row.get(1);
        let weight = rand::distributions::Weighted {
            weight: subs,
            item: serial
        };

        vec.push(weight);

        println!("{} -> {}", serial, subs);
    }

    return vec;
}