// Trebuchet?!
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::path::Path;

pub fn trebuchet(file_name: &String) {
    let path = Path::new(file_name);
    let display = path.display();

    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening the file: {}", e);
            eprintln!("Fpath: {}", display);
            return;
        }
    };

    let reader = BufReader::new(file);

    let re = Regex::new(r"\d").unwrap();

    let mut sum = 0;
    for line in reader.lines() {
        if let Ok(line_content) = line {
            let numbers: Vec<&str> = re
                .find_iter(&line_content)
                .map(|mat| mat.as_str())
                .collect();
            if numbers.len() > 0 {
                sum += (numbers[0].parse::<i32>().unwrap_or(0)) * 10;
                sum += numbers[numbers.len() - 1].parse::<i32>().unwrap_or(0);
            }
        }
    }
    println!("Sum: {}", sum);
}
