extern crate chrono;
extern crate diesel;
extern crate dotenv;
extern crate rayon;
extern crate reqwest;
extern crate scraper;
extern crate uscis;

use uscis::models::*;
use uscis::schema;

use chrono::naive::NaiveDateTime;
use diesel::mysql::*;
use diesel::prelude::*;
use rayon::prelude::*;
use reqwest::Error;
use scraper::{Html, Selector};

fn main() {
    let records = crawl_range(1890220002, 1890220003).unwrap();
    let conn = uscis::establish_connection();
    insert_entry(&conn, &records);
}

pub fn insert_entry<'a>(conn: &MysqlConnection, data: &Vec<Record>) {
    use self::schema::records::dsl::*;

    diesel::insert_into(records)
        .values(data)
        .execute(conn)
        .expect("Failed to insert records");
}

#[test]
fn test_parse() {
    let test = "On July 19, 2018, we mailed your new card";
    let parsed = parse_date(&test);
    assert!(parsed.is_ok());
    assert_eq!(NaiveDate::from_ymd(2018, 7, 19), parsed.unwrap());
}

fn crawl_range(from: i32, to: i32) -> Result<Vec<Record>, Error> {
    let tasks: Vec<_> = (from..to).collect();
    let results: Vec<Result<Option<Record>, Error>> =
        tasks.par_iter().map(|&i| crawl_single(i)).collect();

    Ok(results
        .into_iter()
        .filter_map(|r| r.ok())
        .map(|r| r.unwrap())
        .collect())
}

fn now() -> NaiveDateTime {
    chrono::offset::Utc::now().naive_utc()
}

/// Assuming the input has the form "On <Month> <Day>, <Year>, xxx".
fn parse_date(description: &str) -> Result<chrono::naive::NaiveDate, chrono::ParseError> {
    let date = &description[3..]
        .split(',')
        .take(2)
        .collect::<Vec<&str>>()
        .join(",");
    chrono::naive::NaiveDate::parse_from_str(date, "%B %d, %Y")
}

fn crawl_single(id: i32) -> Result<Option<Record>, Error> {
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
        let description = description.unwrap();
        let update_date = parse_date(&description).unwrap();
        Ok(Some(Record {
            id: id as u64,
            title: title.unwrap(),
            description: description,
            update_date: update_date,
            crawl_time: now(),
        }))
    } else {
        Ok(None)
    }
}
