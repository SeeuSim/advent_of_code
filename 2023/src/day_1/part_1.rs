// Trebuchet?!

use regex::Regex;
use std::io::BufRead;

use crate::utils::extract_file;

pub fn trebuchet(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file.");

    let re = Regex::new(r"\d").unwrap();

    let mut sum = 0;
    for line in reader.lines() {
        if let Ok(line_content) = line {
            let numbers: Vec<&str> = re
                .find_iter(&line_content)
                .map(|mat| mat.as_str())
                .collect();
            if !numbers.is_empty() {
                sum += (numbers[0].parse::<i32>().unwrap_or(0)) * 10;
                sum += numbers[numbers.len() - 1].parse::<i32>().unwrap_or(0);
            }
        }
    }
    println!("Sum: {}", sum);
}
