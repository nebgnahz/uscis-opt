extern crate rayon;
extern crate reqwest;
extern crate scraper;

use rayon::prelude::*;
use reqwest::Error;
use scraper::{Html, Selector};

fn main() {
    crawl_range(1890220002, 1890220009).unwrap();
}

struct Record {
    id: u64,
    title: String,
    description: String,
}

fn crawl_range(from: u64, to: u64) -> Result<(), Error> {
    let tasks: Vec<u64> = (from..to).collect();
    let results: Vec<Result<Option<Record>, Error>> =
        tasks.par_iter().map(|&i| crawl_single(i)).collect();
    for r in results {
        if r.is_ok() {
            if let Some(record) = r.unwrap() {
                println!("{}, {}, {}", record.id, record.title, record.description);
            }
        }
    }
    Ok(())
}

fn crawl_single(id: u64) -> Result<Option<Record>, Error> {
    let uri = format!(
        "https://egov.uscis.gov/casestatus/mycasestatus.do?appReceiptNum=YSC{}",
        id
    );

    let body = reqwest::get(&uri)?.text()?;
    let document = Html::parse_document(&body);
    let title = Selector::parse("div.appointment-sec div.text-center h1").unwrap();
    let description = Selector::parse("div.appointment-sec div.text-center p").unwrap();

    let title = document.select(&title).last().map(|i| i.inner_html());
    let description = document.select(&description).last().map(|i| i.inner_html());

    if title.is_some() && description.is_some() {
        Ok(Some(Record {
            id: id,
            title: title.unwrap(),
            description: description.unwrap(),
        }))
    } else {
        Ok(None)
    }
}
