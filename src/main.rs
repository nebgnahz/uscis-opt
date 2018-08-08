extern crate chrono;
extern crate uscis;

use chrono::naive::NaiveDate;

fn main() {
    let status = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "data/status.txt");
    let mut statuses = uscis::status::AllStatus::new(status).unwrap();
    statuses.update(
        1890230101,
        "Case Was Received",
        NaiveDate::from_ymd(2018, 08, 15),
    );
    println!("{:?}", statuses);
    statuses.commit().expect("failed to update status CSV");
}

fn _test() {
    let proxy_file = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "proxy-list");
    let proxies = uscis::read_proxy(proxy_file);

    let pending_file = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), "data/pending.txt");
    let mut pendings = uscis::pending::Pending::new(pending_file).unwrap();
    // pendings.swap(vec![1890230606, 1890230608]);
    // pendings.grow();
    // pendings.commit().expect("failed to update pending file");

    println!("{:?}", uscis::crawl(1890230101, &proxies[0]).unwrap());
}
