// Trebuchet?! Part 2

use std::io::BufRead;

use crate::utils::extract_file;

/**
 * Credits to @ChristopherBiscardi
 */
fn process_line(line: &str) -> u32 {
    let mut it = (0..line.len()).filter_map(|index| {
        let reduced_line = &line[index..];
        let result = if reduced_line.starts_with("one") {
            '1'
        } else if reduced_line.starts_with("two") {
            '2'
        } else if reduced_line.starts_with("three") {
            '3'
        } else if reduced_line.starts_with("four") {
            '4'
        } else if reduced_line.starts_with("five") {
            '5'
        } else if reduced_line.starts_with("six") {
            '6'
        } else if reduced_line.starts_with("seven") {
            '7'
        } else if reduced_line.starts_with("eight") {
            '8'
        } else if reduced_line.starts_with("nine") {
            '9'
        } else {
            reduced_line.chars().next().unwrap()
        };

        // Error if there is no digit
        result.to_digit(10)
    });
    let first = it.next().expect("should be a number");

    // First has consumed, so last will take last occurrence.
    // If there is no next val, then None
    match it.last() {
        Some(num) => format!("{first}{num}"),
        None => format!("{first}{first}"),
    }
    .parse::<u32>()
    .expect("should be a valid number")
}

pub fn trebuchet_part_two(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file.");

    let mut sum = 0;
    for line in reader.lines() {
        if let Ok(line_content) = line {
            sum += process_line(line_content.as_str());
        }
    }
    println!("Sum: {}", sum);
}
