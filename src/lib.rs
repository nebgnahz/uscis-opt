#[macro_use]
extern crate log;
extern crate serde_json;
extern crate chrono;
extern crate rayon;
extern crate reqwest;
extern crate scraper;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate csv;

pub const INCREMENT: u64 = 100;

pub mod crawler;
pub use crawler::crawl;
pub mod status;
pub use status::Statuses;
