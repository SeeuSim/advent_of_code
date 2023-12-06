// Wait For It

use crate::utils::extract_file;
use std::io::BufRead;

pub fn wait_for_it(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    let mut provider = reader
        .lines()
        .into_iter()
        .filter_map(std::io::Result::ok)
        .filter(|x| !x.is_empty());

    let raw_times = provider.next().unwrap();
    let times = raw_times
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<u32>().unwrap());

    let raw_distances = provider.next().unwrap();
    let distances = raw_distances
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<u32>().unwrap());

    let time_dist_pair = times.zip(distances);

    let res = time_dist_pair
        .map(|(time, record)| {
            (1..time + 1)
                .map(move |x| {
                    let dist = (time - x) * x;
                    dist
                })
                .filter(move |&x| x > record)
                .count()
        })
        .fold(1, |acc, e| acc * e);

    println!("Product: {}", res);
}
