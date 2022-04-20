fn main() {
    parse_smartphone();
}

use std::{fs::OpenOptions, io::Read, collections::HashMap};

fn parse_smartphone() {
    let mut file = OpenOptions::new()
        .read(true)
        .open("us-smartphone-ownership.csv").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let buf: Vec<&str> = buf.split('\n').collect();
    let mut avg_map: HashMap<i32, (i32, i32)> = HashMap::new();
    for i in 0..buf.len() {
        let data_point: Vec<&str> = buf[i].split(',').collect();
        let (year, percent) = (data_point[1].trim().parse::<i32>().unwrap(), data_point[2].trim().parse::<i32>().unwrap());
        if avg_map.contains_key(&year) {
            let (p, n) = avg_map.get(&year).unwrap();
            avg_map.insert(year, (percent + p, n + 1));
        } else {
            avg_map.insert(year, (percent, 1));
        }
    }
    let mut cellphone_data: HashMap<i32, f64> = HashMap::new();
    for year in avg_map.keys() {
        let (cur_percent, cur_n) = avg_map.get(year).unwrap();
        cellphone_data.insert(*year, *cur_percent as f64/ *cur_n as f64);
    }
    dbg!(cellphone_data);
}