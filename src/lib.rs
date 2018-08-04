extern crate chrono;
extern crate rayon;
extern crate reqwest;
extern crate scraper;

use chrono::naive::NaiveDateTime;
// use rayon::prelude::*;
use reqwest::Error;
use scraper::{Html, Selector};

use chrono::naive::{NaiveDate};

#[derive(Debug)]
pub struct Record {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub update_date: NaiveDate,
    pub crawl_time: NaiveDateTime,
}

pub fn main() {}

pub fn crawl_one(id: i32) -> Result<String, Error> {
    let uri = format!(
        "https://egov.uscis.gov/casestatus/mycasestatus.do?appReceiptNum=YSC{}",
        id
    );

    let client = reqwest::Client::builder().build()?;
    let body = client.get(&uri).send()?.text()?;
    let document = Html::parse_document(&body);
    let title = Selector::parse("div.appointment-sec div.text-center h1").unwrap();
    let description = Selector::parse("div.appointment-sec div.text-center p").unwrap();

    let title = document.select(&title).last().map(|i| i.inner_html());
    let description = document.select(&description).last().map(|i| i.inner_html());

    Ok(format!(
        "{} {} {}",
        id,
        title.unwrap(),
        description.unwrap()
    ))
}
