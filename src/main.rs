extern crate actix_web;

mod http;
mod postgres;
mod util;

fn get_random(items: &mut Vec<rand::distributions::Weighted<String>>) -> String {
    use http::rand::distributions::IndependentSample;
    use http::rand::distributions::Distribution;

    let wc: rand::distributions::WeightedChoice<String> = rand::distributions::WeightedChoice::new(items);
    let mut rng = rand::thread_rng();
    let serial: String = wc.ind_sample(&mut rng);

    return serial;
}

fn main() {
    let bound = 0;
    type Sender = std::sync::mpsc::SyncSender<std::collections::HashSet<String>>;
    type Receive = std::sync::mpsc::Receiver<std::collections::HashSet<String>>;

    let (sender, receiver):  (Sender, Receive) = std::sync::mpsc::sync_channel(bound);
    std::thread::spawn(move|| {
        let mut items: Vec<rand::distributions::Weighted<String>> = postgres::channels();
        let mut set = std::collections::HashSet::new();
        loop {
            println!("{}", "Collecting serials...");
            while set.len() < 50 {
                let serial = get_random(&mut items);
                println!("Got {}", serial);
                set.insert(serial);
            }
            sender.send(set.clone()).unwrap();
            set.clear();
        }
    });

    http::server_init(receiver);
}