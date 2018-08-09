use serde_json;
use reqwest;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    ip: String,
    crawled: Vec<Record>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Record {
    pub id: String,
    pub title: String,
    pub description: String,
}

const URL: &'static str = "https://3yqhf6gl55.execute-api.us-east-1.amazonaws.com/crawl";

pub fn crawl(from: u64, to: u64) -> Vec<Record> {
    trace!("Crawling from {} to {}", from, to);
    let url = format!("{}/crawl?start={}&end={}", URL, from, to);
    let body = reqwest::get(&url).unwrap().text().unwrap();
    let p: Response = serde_json::from_str(&body).unwrap();
    p.crawled
}
