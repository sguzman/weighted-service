extern crate postgres;

use util;

fn conn_str() -> String {
    let db_host: String = util::env("DB_HOST", "192.168.1.63");
    let db_port: String = util::env("DB_PORT", "30000");

    return format!("postgresql://postgres:@{}:{}/youtube", db_host, db_port);
}

fn conn() -> postgres::Connection {
    let ssl = postgres::TlsMode::None;
    let conn_string: String = conn_str();

    return postgres::Connection::connect(conn_string, ssl).unwrap();
}

pub fn query() {
    let c: postgres::Connection = conn();
    let query: &str = "SELECT serial, subs FROM youtube.entities.chan_stats ORDER BY RANDOM() LIMIT 10";

    for row in &c.query(query, &[]).unwrap() {
        let id: String = row.get(0);
        let name: i64 = row.get(1);
        println!("{} -> {}", id, name);
    }
}