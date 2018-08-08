extern crate chrono;
extern crate rayon;
extern crate uscis;

use rayon::prelude::*;

fn main() {
    let proxy_file = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "proxy-list");
    let proxies = uscis::read_proxy(proxy_file);

    let status = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "data/status.csv");
    let mut statuses = uscis::status::AllStatus::new(status).unwrap();
    // for i in 1890230101..1890230110 {
    //     let record = uscis::crawl(i, &proxies[0]).unwrap();
    //     statuses.update(record);
    // }

    let proxy_len = proxies.len();
    rayon::ThreadPoolBuilder::new()
        .num_threads(proxy_len)
        .build_global()
        .unwrap();
    let tasks: Vec<u64> = (1890230550..1890230650).collect();
    let records: Vec<uscis::Record> = tasks
        .par_iter()
        .map(|&i| uscis::crawl(i, &proxies[i as usize % proxy_len]))
        .filter_map(|r| r.ok())
        .collect();

    for r in records {
        statuses.update(r);
    }
    statuses.commit().expect("failed to update status CSV");
}

fn _test() {
    let pending_file = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "data/pending.txt");
    let mut pendings = uscis::pending::Pending::new(pending_file).unwrap();
    // pendings.swap(vec![1890230606, 1890230608]);
    // pendings.grow();
    // pendings.commit().expect("failed to update pending file");
}
