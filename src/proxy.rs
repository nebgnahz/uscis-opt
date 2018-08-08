//! Read a file list of proxies and rotate among them.
//! Thanks to https://www.my-proxy.com/free-proxy-list.html

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

pub fn read_proxy<P: AsRef<Path>>(filename: P) -> Vec<String> {
    let f = File::open(filename).expect("file not found");
    let reader = BufReader::new(&f);
    reader.lines().filter_map(|x| x.ok()).collect()
}
