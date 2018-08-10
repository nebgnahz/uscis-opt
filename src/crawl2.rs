use std::env;
use reqwest;
use serde_json;

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

pub fn crawl(from: u64, to: u64) -> Vec<Record> {
    trace!("Crawling from {} to {}", from, to);
    let url = env::var("USCIS_URL").unwrap();
    let url = format!("{}?start={}&end={}", url, from, to);
    let body = reqwest::get(&url).unwrap().text().unwrap();
    let p: Response = serde_json::from_str(&body).unwrap();
    p.crawled
}
