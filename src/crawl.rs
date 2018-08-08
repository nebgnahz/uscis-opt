use chrono;
use chrono::naive::{NaiveDate, NaiveDateTime};
use reqwest::{self, Error};
use scraper::{Html, Selector};

#[derive(Debug)]
pub struct Record {
    pub id: u32,
    pub title: String,
    // pub description: String,
    pub update_date: NaiveDate,
    pub crawl_time: NaiveDateTime,
}

const EGOV_URL: &'static str = "https://egov.uscis.gov/casestatus/mycasestatus.do?appReceiptNum=";
const PREFIX: &'static str = "YSC";

pub fn crawl(id: u32, proxy: &str) -> Result<Record, Error> {
    let uri = format!("{}{}{}", EGOV_URL, PREFIX, id);
    let proxy = format!("http://{}", proxy);
    let client = reqwest::Client::builder()
        .proxy(reqwest::Proxy::http(&proxy)?)
        .build()?;
    let body = client.get(&uri).send()?.text()?;
    let document = Html::parse_document(&body);
    let title = Selector::parse("div.appointment-sec div.text-center h1").unwrap();
    let description = Selector::parse("div.appointment-sec div.text-center p").unwrap();

    let title = document.select(&title).last().map(|i| i.inner_html());
    let description = document.select(&description).last().map(|i| i.inner_html());
    let update_date = parse_date(&description.unwrap()).unwrap();

    Ok(Record {
        id: id,
        title: title.unwrap(),
        update_date: update_date,
        crawl_time: chrono::Utc::now().naive_utc(),
    })
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
