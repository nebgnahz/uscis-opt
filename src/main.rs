use std::{thread, time};

extern crate chrono;
extern crate env_logger;
extern crate uscis;
#[macro_use]
extern crate log;

fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: cargo run -- <start> <end>");
        return;
    }

    let start: u64 = args[1].parse().unwrap();
    let end: u64 = args[2].parse().unwrap();
    let start = start / uscis::INCREMENT * uscis::INCREMENT;
    let end = end / uscis::INCREMENT * uscis::INCREMENT;

    let mut current = start;
    let sleep_time = time::Duration::from_secs(60 * 45);

    loop {
        if current >= end {
            current = start;
        }

        current = crawl_one_round(current);
        trace!("sleeping");
        thread::sleep(sleep_time);
        trace!("wake up and work on {}, until {}", current, end);
    }
}

fn crawl_one_round(mut prefix: u64) -> u64 {
    let dir = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "data");
    let apis = uscis::crawler::read_apis(dir.clone() + "/apis");

    // we first wake up all APIs by sending small requests to them
    for api in &apis {
        uscis::crawler::wakeup(&api);
    }

    // we then crawl the next set of tasks
    'outer: for api in apis {
        for _i in 0..20 {
            let status_file = format!("{}/raw-data/{}.csv", dir, prefix);
            let mut statuses = uscis::Statuses::new(status_file, prefix).unwrap();
            let records = uscis::crawl(&api, prefix, prefix + uscis::INCREMENT);

            if records.len() != 0 {
                for r in records {
                    statuses.update(&r);
                }
                statuses.commit().unwrap();
                prefix += uscis::INCREMENT;
            } else {
                warn!("Crawling endpoint {} is not working properly", api);
                continue 'outer;
            }
        }
    }

    return prefix;
}
