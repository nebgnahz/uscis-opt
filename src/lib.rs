#[macro_use]
extern crate log;

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

mod crawl;
pub use crawl::{crawl, Record};

pub mod pending;
pub mod status;
