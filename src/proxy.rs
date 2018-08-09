//! Read a file list of proxies and rotate among them.
//! Thanks to https://www.my-proxy.com/free-proxy-list.html

use std::fs::File;
use std::io::Write;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::io::BufWriter;

pub fn read_proxy<P: AsRef<Path>>(filename: P) -> Vec<String> {
    let f = File::open(filename).expect("file not found");
    let reader = BufReader::new(&f);
    reader.lines().filter_map(|x| x.ok()).collect()
}

pub fn write_proxy<P: AsRef<Path>>(proxies: &Vec<&String>, filename: P) {
    let f = File::open(filename).expect("file not found");
    let mut writer = BufWriter::new(&f);
    for i in proxies {
        writeln!(writer, "{}", i).unwrap();
    }
}
