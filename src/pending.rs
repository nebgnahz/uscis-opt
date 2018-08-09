//! Manage a file that contains a list of pending ids.

use chrono;
use chrono::naive::NaiveDate;
use csv;
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use INCREMENT;

#[derive(Serialize, Deserialize, Debug)]
pub struct PendingEntry {
    pub id: u64,
    pub done: bool,
    pub i765: bool,
    pub last_crawl: NaiveDate,
}

impl PendingEntry {
    pub fn new(i: u64) -> Self {
        PendingEntry {
            id: i,
            done: false,
            i765: true,
            last_crawl: NaiveDate::from_ymd(2008, 8, 8),
        }
    }
}

#[derive(Debug)]
pub struct Pending {
    filename: PathBuf,
    pendings: BTreeMap<u64, PendingEntry>,
}

impl Pending {
    pub fn new<P: AsRef<Path>>(filename: P, range: u64) -> Result<Self, io::Error> {
        let filename = filename.as_ref().to_path_buf();

        if let Ok(mut rdr) = csv::Reader::from_path(&filename) {
            let pendings: BTreeMap<u64, PendingEntry> = rdr
                .deserialize()
                .map(|r: Result<PendingEntry, csv::Error>| r.unwrap())
                .map(|r| (r.id, r))
                .collect();
            Ok(Pending {
                filename: filename,
                pendings: pendings,
            })
        } else {
            let pendings: BTreeMap<u64, PendingEntry> = ((range / INCREMENT * INCREMENT)
                ..((range / INCREMENT + 1) * INCREMENT))
                .map(|i| (i, PendingEntry::new(i)))
                .collect();
            Ok(Pending {
                filename: filename,
                pendings: pendings,
            })
        }
    }

    pub fn commit(&self) -> Result<(), io::Error> {
        let tmp = self.filename.with_extension("tmp");

        {
            let mut wtr = csv::Writer::from_path(&tmp)?;

            for s in &self.pendings {
                wtr.serialize(s.1)?
            }
        }

        fs::rename(tmp, &self.filename)
    }

    pub fn set_done(&mut self, id: u64) {
        self.pendings.get_mut(&id).unwrap().done = true;
    }

    pub fn set_non_i765(&mut self, id: u64) {
        self.pendings.get_mut(&id).unwrap().i765 = false;
    }

    pub fn set_crawl(&mut self, id: u64, crawl: NaiveDate) {
        self.pendings.get_mut(&id).unwrap().last_crawl = crawl;
    }

    pub fn tasks(&self) -> Vec<u64> {
        let today = chrono::Utc::now().naive_utc().date();

        self.pendings
            .iter()
            .filter(|(_k, v)| v.i765 && !v.done && (today - v.last_crawl).num_days() >= 1)
            .map(|(k, _v)| *k)
            .collect()
    }
}
