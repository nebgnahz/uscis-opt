use chrono::naive::NaiveDate;
use crawl::Record;
use csv;
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const RECEIVED: &'static str = "Case Was Received";
const PRODUCED: &'static str = "New Card Is Being Produced";
const MAILED: &'static str = "Card Was Mailed To Me";
const DELIVERED: &'static str = "Card Was Delivered To Me By The Post Office";
const REJECTED: &'static str = "Case Rejected Because I Sent An Incorrect Fee";
const REJECTED2: &'static str = "Case Rejected For Incorrect Fee And Form Not Signed";
const RETURNED: &'static str =
    "Notice Was Returned To USCIS Because The Post Office Could Not Deliver It";
const UPDATE: &'static str = "Correspondence Was Received And USCIS Is Reviewing It";
const RFE: &'static str = "Request for Initial Evidence Was Mailed";

#[derive(Serialize, Deserialize, Debug)]
pub struct Status {
    pub id: u64,
    pub received: Option<NaiveDate>,
    pub produced: Option<NaiveDate>,
    pub mailed: Option<NaiveDate>,
    pub delivered: Option<NaiveDate>,
    pub returned: Option<NaiveDate>,
    pub rejected: Option<NaiveDate>,
    pub rfe: Option<NaiveDate>,
    pub update: Option<NaiveDate>,
    pub other: Option<NaiveDate>,
    pub last_update: Option<NaiveDate>,
}

impl Status {
    pub fn new(id: u64) -> Self {
        Status {
            id: id,
            received: None,
            produced: None,
            mailed: None,
            delivered: None,
            returned: None,
            rejected: None,
            rfe: None,
            update: None,
            other: None,
            last_update: None,
        }
    }

    pub fn update(&mut self, status: &str, date: NaiveDate) {
        match status {
            RECEIVED => self.received = Some(date),
            PRODUCED => self.produced = Some(date),
            MAILED => self.mailed = Some(date),
            DELIVERED => self.delivered = Some(date),
            REJECTED => self.rejected = Some(date),
            REJECTED2 => self.rejected = Some(date),
            RETURNED => self.returned = Some(date),
            UPDATE => self.update = Some(date),
            RFE => self.rfe = Some(date),
            _ => self.other = Some(date),
        }

        if self.last_update.is_some() && date == self.last_update.unwrap() {
        } else {
            self.last_update = Some(date);
        }
    }
}

#[derive(Debug)]
pub struct AllStatus {
    filename: PathBuf,
    statuses: BTreeMap<u64, Status>,
}

impl AllStatus {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, io::Error> {
        let filename = path.as_ref().to_path_buf();
        let mut rdr = csv::Reader::from_path(path)?;
        let statuses: BTreeMap<u64, Status> = rdr
            .deserialize()
            .map(|r: Result<Status, csv::Error>| r.unwrap())
            .map(|r| (r.id, r))
            .collect();
        Ok(AllStatus {
            filename: filename,
            statuses: statuses,
        })
    }

    pub fn update(&mut self, record: Record) {
        let id = record.id;
        let entry = self.statuses.entry(id).or_insert(Status::new(id));
        entry.update(&record.title, record.update_date)
    }

    pub fn commit(&self) -> Result<(), io::Error> {
        let tmp = self.filename.with_extension("tmp");

        {
            let mut wtr = csv::Writer::from_path(&tmp)?;

            for s in &self.statuses {
                wtr.serialize(s.1)?
            }
        }

        fs::rename(tmp, &self.filename)
    }
}
