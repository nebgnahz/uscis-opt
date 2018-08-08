//! Manage a file that contains a list of pending ids.

use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Pending {
    filename: PathBuf,
    ids: Vec<u32>,
}

const DELTA: u32 = 1000;
const MAX_PENDING: usize = 10000;

impl Pending {
    pub fn new<P: AsRef<Path>>(filename: P) -> Result<Self, io::Error> {
        let filename = filename.as_ref().to_path_buf();
        let f = File::open(&filename).expect("file not found");
        let reader = BufReader::new(&f);
        let ids = reader
            .lines()
            .filter_map(|x| x.ok())
            .filter_map(|x| x.parse::<u32>().ok())
            .collect();
        Ok(Pending {
            filename: filename,
            ids: ids,
        })
    }

    pub fn commit(&self) -> Result<(), io::Error> {
        let tmp = self.filename.with_extension("tmp");

        {
            let tmpfile = File::create(&tmp)?;
            let mut writer = BufWriter::new(&tmpfile);
            for id in &self.ids {
                writeln!(writer, "{}", id).unwrap();
            }
        }

        fs::rename(tmp, &self.filename)
    }

    pub fn swap(&mut self, ids: Vec<u32>) {
        self.ids = ids;
    }

    pub fn grow(&mut self) {
        if self.ids.len() < MAX_PENDING {
            let max = self.ids.iter().max().unwrap().clone();
            let _end = max + DELTA;
            self.ids.extend((max + 1)..(max + DELTA));
        }
    }
}
