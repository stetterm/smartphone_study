fn main() {
    let (smartphone_data, cancer_data) 
        = (parse_smartphone(), parse_cancer());
    dbg!(&smartphone_data);
    dbg!(&cancer_data);
    let mut smartphone_values = vec![];
    let mut cancer_values = vec![];
    for i in 2004..2017 {
        smartphone_values.push(*smartphone_data.get(&i).unwrap());
        cancer_values.push(*cancer_data.get(&i).unwrap());
    }
    println!("The sample correlation is: {}", sample_correlation(&smartphone_values, &cancer_values));
}

use smartphone_study::stats::*;

use std::{fs::OpenOptions, io::Read, collections::HashMap};

fn parse_smartphone() -> HashMap<i32, f64> {
    let mut file = OpenOptions::new()
        .read(true)
        .open("us-smartphone-ownership.csv").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let buf: Vec<&str> = buf.split('\n').collect();
    let mut avg_map: HashMap<i32, (i32, i32)> = HashMap::new();
    for i in 0..buf.len() {
        let data_point: Vec<&str> = buf[i].split(',').collect();
        let (year, percent) 
            = (data_point[1].trim().parse::<i32>().unwrap(), data_point[2].trim().parse::<i32>().unwrap());
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
    cellphone_data
}

fn parse_cancer() -> HashMap<i32, f64> {
    let mut file = OpenOptions::new()
        .read(true)
        .open("us-brain-cancer-diagnosis.csv").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let buf = buf.split('\n').collect::<Vec<&str>>();
    let mut cancer_data = HashMap::new();
    for line in buf {
        let data_point = line.split(',').collect::<Vec<&str>>();
        cancer_data.insert(
            data_point[0].trim().parse::<i32>().unwrap(),
            data_point[1].trim().parse::<f64>().unwrap()
        );
    }
    cancer_data
}