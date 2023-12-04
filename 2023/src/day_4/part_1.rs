// Scratchcards

use crate::utils::extract_file;

use std::collections::HashSet;
use std::io::BufRead;

fn process_line(line: String) -> u32 {
    if line.len() == 0 {
        return 0;
    }
    let mut line_parts = line.split(": ");
    let _actual = line_parts.nth(1);

    let data = match _actual {
        Some(card_ct) => card_ct,
        None => &"",
    };
    if data.len() == 0 {
        return 0;
    }

    let mut left_right = data.split(" | ").into_iter();

    let _left = left_right.next();

    if _left.is_none() {
        return 0;
    }

    let winning_set: HashSet<String> = HashSet::from_iter(
        _left
            .unwrap()
            .split(" ")
            .map(|x| x.trim().to_string())
            .filter(|x| x.len() > 0)
            .into_iter(),
    );

    let _right = left_right.next();
    if _right.is_none() {
        return 0;
    }
    let count = _right
        .unwrap()
        .split(" ")
        .map(|x| x.trim().to_string())
        .filter(|x| x.len() > 0 && winning_set.contains(x))
        .fold(0, |acc, _e| acc + 1);
    
    if count <= 0 {
        return count;
    }
    2_u32.pow(count - 1)
}

pub fn scratch_cards(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    let sm: u32 = reader
        .lines()
        .map(|line| {
            if let Ok(line_content) = line {
                return line_content;
            }
            "".to_string()
        })
        .map(process_line)
        .sum();
    println!("Sum: {}", sm);
}
