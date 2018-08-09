use chrono;
use chrono::naive::NaiveDate;
use crawl2::Record;
use csv;
use std::collections::BTreeMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use INCREMENT;

const RECEIVED: &'static str = "Case Was Received";
const PRODUCED: &'static str = "New Card Is Being Produced";
const MAILED: &'static str = "Card Was Mailed To Me";
const USPS: &'static str = "Card Was Picked Up By The United States Postal Service";
const DELIVERED: &'static str = "Card Was Delivered To Me By The Post Office";
const REJECTED: &'static str = "Case Rejected Because I Sent An Incorrect Fee";
const REJECTED2: &'static str = "Case Rejected For Incorrect Fee And Form Not Signed";
const REJECTED3: &'static str = "Case Was Rejected Because It Was Improperly Filed";
const RETURNED: &'static str =
    "Notice Was Returned To USCIS Because The Post Office Could Not Deliver It";
const UPDATE: &'static str = "Correspondence Was Received And USCIS Is Reviewing It";
const RFE: &'static str = "Request for Initial Evidence Was Mailed";

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub id: u64,
    pub received: Option<NaiveDate>,
    pub delivered: Option<NaiveDate>,
    pub produced: Option<NaiveDate>,
    pub pickedup: Option<NaiveDate>,
    pub mailed: Option<NaiveDate>,
    pub returned: Option<NaiveDate>,
    pub rejected: Option<NaiveDate>,
    pub rfe: Option<NaiveDate>,
    pub update: Option<NaiveDate>,
    pub other: Option<NaiveDate>,
    pub last_update: Option<NaiveDate>,
    pub last_crawl: Option<NaiveDate>,
    pub is_done: bool,
    pub is_i765: bool,
}

impl Status {
    pub fn new(id: u64) -> Self {
        Status {
            id: id,
            is_done: false,
            is_i765: true,
            received: None,
            produced: None,
            mailed: None,
            pickedup: None,
            delivered: None,
            returned: None,
            rejected: None,
            rfe: None,
            update: None,
            other: None,
            last_update: None,
            last_crawl: None,
        }
    }

    pub fn update(&mut self, title: &str, description: &str) {
        let date = parse_date(description).ok();

        match title {
            RECEIVED => self.received = date,
            PRODUCED => self.produced = date,
            USPS => self.pickedup = date,
            MAILED => self.mailed = date,
            DELIVERED => self.delivered = date,
            REJECTED => self.rejected = date,
            REJECTED2 => self.rejected = date,
            REJECTED3 => self.rejected = date,
            RETURNED => self.returned = date,
            UPDATE => self.update = date,
            RFE => self.rfe = date,
            _ => self.other = date,
        }

        if title == DELIVERED {
            self.is_done = true;
        }

        let today = chrono::Utc::now().naive_utc().date();
        self.last_crawl = Some(today);

        self.is_i765 = !is_i130(&description) && !is_i129(&description) && !is_g28(&description);

        if self.last_update.is_some() && date.is_some() && date.unwrap() <= self.last_update.unwrap() {
        } else {
            self.last_update = date;
        }
    }
}

#[derive(Debug)]
pub struct Statuses {
    filename: PathBuf,
    statuses: BTreeMap<u64, Status>,
}

impl Statuses {
    /// If there is such file, we read it; otherwise, we create it.
    pub fn new<P: AsRef<Path>>(path: P, range: u64) -> Result<Self, io::Error> {
        let filename = path.as_ref().to_path_buf();
        if let Ok(mut rdr) = csv::Reader::from_path(path) {
            let statuses: BTreeMap<u64, Status> = rdr
                .deserialize()
                .map(|r: Result<Status, csv::Error>| r.unwrap())
                .map(|r| (r.id, r))
                .collect();
            Ok(Statuses {
                filename: filename,
                statuses: statuses,
            })
        } else {
            let statuses: BTreeMap<u64, Status> = ((range / INCREMENT * INCREMENT)
                ..((range / INCREMENT + 1) * INCREMENT))
                .map(|i| (i, Status::new(i)))
                .collect();
            Ok(Statuses {
                filename: filename,
                statuses: statuses,
            })
        }
    }

    pub fn update(&mut self, record: &Record) {
        let id = record.id[3..].parse::<u64>().unwrap();
        let entry = self.statuses.entry(id).or_insert(Status::new(id));
        entry.update(&record.title, &record.description)
    }

    pub fn commit(&self) -> Result<(), io::Error> {
        let tmp = self.filename.with_extension("tmp");

        {
            let mut wtr = csv::Writer::from_path(&tmp)?;

            for s in &self.statuses {
                wtr.serialize(s.1)?
            }
        }

        fs::rename(tmp, &self.filename)?;

        let crawl_info = self.filename.with_extension("txt");
        let crawl_time = chrono::Utc::now().naive_utc();
        let mut file = fs::File::create(crawl_info)?;
        write!(file, "{:?}", crawl_time)?;

        Ok(())
    }
}

pub fn is_i130(description: &str) -> bool {
    description.find("I-130").is_some()
}

pub fn is_g28(description: &str) -> bool {
    description.find("G-28").is_some()
}

pub fn is_i129(description: &str) -> bool {
    description.find("I-129").is_some()
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
