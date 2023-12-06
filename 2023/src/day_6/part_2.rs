// Wait For It

use crate::utils::extract_file;
use std::io::BufRead;

pub fn wait_for_it_two(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    let mut provider = reader
        .lines()
        .into_iter()
        .filter_map(std::io::Result::ok)
        .filter(|x| !x.is_empty());

    let raw_time = provider.next().unwrap();
    let time = raw_time
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .filter(|x| !x.is_empty())
        .fold("".to_string(), |acc, e| format!("{}{}", acc, e.to_string()))
        .parse::<u64>()
        .unwrap();

    let raw_record = provider.next().unwrap();
    let record = raw_record
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .filter(|x| !x.is_empty())
        .fold("".to_string(), |acc, e| format!("{}{}", acc, e.to_string()))
        .parse::<u64>()
        .unwrap();

    let count_records = (1..time).map(move |x| {
        let dist = (time - x) * x;
        dist
    }).filter(move |&x| x > record).count();

    println!("Count: {}", count_records);
}
