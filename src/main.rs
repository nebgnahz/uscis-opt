extern crate chrono;
extern crate env_logger;
extern crate rayon;
extern crate uscis;

use rayon::prelude::*;
use std::collections::HashSet;

fn _test() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", uscis::crawl(1890200001, Some((&args[1], 0))));
}

fn main() {
    env_logger::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Type range");
        return;
    }

    let range: u64 = args[1].parse().unwrap();
    if range < 1890200000 || range > 1890500000 {
        println!("Invalid range");
        return;
    }

    let prefix = range / uscis::INCREMENT * uscis::INCREMENT;

    let dir = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "data");

    let proxy_file = format!("{}/{}", dir, "proxy-list");
    let proxies = uscis::read_proxy(&proxy_file);

    let pending_file = format!("{}/pending/{}.txt", dir, prefix);
    let mut pendings = uscis::pending::Pending::new(pending_file, range).unwrap();

    let status_file = format!("{}/{}.csv", dir, prefix);
    let mut statuses = uscis::status::Statuses::new(status_file, range).unwrap();

    let proxy_len = proxies.len();
    if proxy_len != 0 {
        rayon::ThreadPoolBuilder::new()
            .num_threads(proxy_len)
            .build_global()
            .unwrap();
    }

    let records: Vec<uscis::Record> = pendings
        .tasks()
        .par_iter()
        .map(|&i| {
            let proxy = if proxy_len == 0 {
                None
            } else {
                let idx = i as usize % proxy_len;
                Some((&proxies[idx], idx))
            };

            uscis::crawl(i, proxy)
        })
        .filter_map(|r| r.ok())
        .collect();

    let mut good_proxies = HashSet::new();
    for r in records {
        statuses.update(&r);

        if r.title == "Card Was Delivered To Me By The Post Office" {
            pendings.set_done(r.id);
        }

        if !r.is_i765 {
            pendings.set_non_i765(r.id);
        }

        pendings.set_crawl(r.id, r.crawl_time.date());

        if let Some(proxy) = r.proxy {
            good_proxies.insert(proxy);
        }
    }

    let updated_proxies: Vec<&String> = proxies
        .iter()
        .enumerate()
        .filter(|(i, _)| good_proxies.get(&i).is_some())
        .map(|(_, p)| p)
        .collect();

    println!("{:?}", updated_proxies);
    uscis::write_proxy(&updated_proxies, proxy_file);
    pendings.commit().expect("failed to commit pending");
    statuses.commit().unwrap();
}
