fn main() {
    parse_smartphone();
}

use std::{fs::OpenOptions, io::Read};

fn parse_smartphone() {
    let mut file = OpenOptions::new()
        .read(true)
        .open("us-smartphone-ownership.csv").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let buf: Vec<&str> = buf.split('\n').collect();
    let mut year_total = 0;
    let mut year_n = 0;
    let mut cur_year: i32;
    for i in 0..buf.len() {
        
    }
}