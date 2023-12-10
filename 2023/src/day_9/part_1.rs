// Mirage Maintainenance

use crate::utils::extract_file;
use std::io::BufRead;

fn parse_line(seq: String) -> i32 {
    let num_seq = seq
        .split(" ")
        .filter_map(|x| {
            let v = x.trim();
            match v.is_empty() {
                true => None,
                _ => v.parse::<i32>().ok(),
            }
        })
        .collect::<Vec<i32>>();

    let mut sequences: Vec<Vec<i32>> = vec![num_seq];
    loop {
        let curr = sequences.last().unwrap().clone();
        let c_len = curr.len();
        if c_len < 2 {
            break;
        }

        let differences = curr
            .clone()
            .into_iter()
            .take(c_len - 1)
            .zip(curr.into_iter().skip(1))
            .map(|(l, r)| r - l)
            .collect::<Vec<i32>>();

        sequences.push(differences.clone());

        let is_all_zero = differences.into_iter().all(|x| x == 0);
        if is_all_zero {
            break;
        }
    }
    sequences
        .into_iter()
        .map(|v| {
            let l = v.len();
            v.into_iter().skip(l - 1).next().unwrap()
        })
        .fold(0, |acc, e| acc + e)
}

pub fn mirage_maintenance(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    let v = reader
        .lines()
        .filter_map(std::result::Result::ok)
        .map(parse_line)
        .sum::<i32>();

    println!("Sum: {}", v);
}
