use chrono;
use chrono::naive::{NaiveDate, NaiveDateTime};
use reqwest::{self, Error};
use scraper::{Html, Selector};
use std::time::Duration;

#[derive(Debug)]
pub struct Record {
    pub id: u64,
    pub title: String,
    // pub description: String,
    pub update_date: NaiveDate,
    pub crawl_time: NaiveDateTime,
    pub is_i765: bool,
    pub proxy: Option<usize>,
}

const EGOV_URL: &'static str = "https://egov.uscis.gov/casestatus/mycasestatus.do?appReceiptNum=";
const PREFIX: &'static str = "YSC";

pub fn crawl(id: u64, proxy: Option<(&String, usize)>) -> Result<Record, Error> {
    trace!("Crawling {}", id);

    let uri = format!("{}{}{}", EGOV_URL, PREFIX, id);
    let mut client_builder = reqwest::Client::builder();
    if let Some(proxy) = proxy {
        let proxy = format!("http://{}", proxy.0);
        client_builder.proxy(reqwest::Proxy::all(&proxy)?);
    }

    client_builder.timeout(Some(Duration::new(5, 0)));
    let client = client_builder.build()?;

    let body = client.get(&uri).send()?.text()?;
    let document = Html::parse_document(&body);
    let title = Selector::parse("div.appointment-sec div.text-center h1").unwrap();
    let description = Selector::parse("div.appointment-sec div.text-center p").unwrap();

    let title = document.select(&title).last().map(|i| i.inner_html());
    let description = document.select(&description).last().map(|i| i.inner_html());

    let description = description.unwrap();
    let is_i765 = !is_i130(&description);
    let update_date = parse_date(&description).unwrap();

    trace!("Crawled {}", id);

    Ok(Record {
        id: id,
        title: title.unwrap(),
        update_date: update_date,
        crawl_time: chrono::Utc::now().naive_utc(),
        is_i765: is_i765,
        proxy: proxy.map(|i| i.1),
    })
}

pub fn is_i130(description: &str) -> bool {
    description.find("I-130").is_some()
}

/// Assuming the input has the form "On <Month> <Day>, <Year>, xxx".
pub fn parse_date(description: &str) -> Result<NaiveDate, chrono::ParseError> {
    let date = &description[3..]
        .split(',')
        .take(2)
        .collect::<Vec<&str>>()
        .join(",");
    chrono::naive::NaiveDate::parse_from_str(date, "%B %d, %Y")
}

#[test]
fn test_parse() {
    let test = "On July 19, 2018, we mailed your new card";
    let parsed = parse_date(&test);
    assert!(parsed.is_ok());
    assert_eq!(NaiveDate::from_ymd(2018, 7, 19), parsed.unwrap());
}
