#[macro_use]
extern crate log;
extern crate chrono;
extern crate rayon;
extern crate reqwest;
extern crate scraper;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;
extern crate csv;

pub const INCREMENT: u64 = 100;

pub mod crawler;
pub use crawler::crawl;
pub mod status;
pub use status::Statuses;

use std::fs::File;
use std::io::prelude::*;

pub fn read_current() -> Option<u64> {
    let filename = format!("{}/progress", env!("CARGO_MANIFEST_DIR"));
    if let Ok(mut f) = File::open(filename) {
        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading progress file");
        Some(contents.parse().expect("failed to parse progress file"))
    } else {
        None
    }
}

pub fn write_current(current: u64) {
    let filename = format!("{}/progress", env!("CARGO_MANIFEST_DIR"));
    let mut f = File::open(&filename)
        .unwrap_or_else(|_| File::create(&filename).expect("failed to create progress file"));
    write!(f, "{}", current).expect("something went wrong writing progress file");
}

pub fn remove_current() {
    let filename = format!("{}/progress", env!("CARGO_MANIFEST_DIR"));
    ::std::fs::remove_file(filename).expect("failed to remove progress file");
}
