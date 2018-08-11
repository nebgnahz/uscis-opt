//! Read a file that has a list of crawlers.

use reqwest;
use serde_json;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

pub fn read_apis<P: AsRef<Path>>(filename: P) -> Vec<String> {
    let f = File::open(filename).expect("file not found");
    let reader = BufReader::new(&f);
    reader.lines().filter_map(|x| x.ok()).collect()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    ip: Option<String>,
    crawled: Vec<Record>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub id: String,
    pub title: String,
    pub description: String,
}

pub fn crawl(base_url: &str, from: u64, to: u64) -> Vec<Record> {
    trace!("Crawling from {} to {}, with {}", from, to, base_url);
    let url = format!("{}?start={}&end={}", base_url, from, to);
    let body = reqwest::get(&url).unwrap().text().unwrap();
    let p: Response = serde_json::from_str(&body).unwrap();
    p.crawled
}

pub fn wakeup(base_url: &str) {
    trace!("Waking up {}", base_url);
    let _ = reqwest::get(base_url).unwrap().text().unwrap();
}
