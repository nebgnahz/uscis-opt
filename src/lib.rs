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

mod proxy;
pub use proxy::read_proxy;
pub use proxy::write_proxy;

pub mod crawl2;

pub mod status;
