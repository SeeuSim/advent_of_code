// Scratchcards

use crate::utils::extract_file;

use std::collections::{BTreeMap, HashSet};
use std::io::BufRead;

fn process_line(line: String) -> u32 {
    if line.is_empty() {
        return 0;
    }
    let mut line_parts = line.split(": ");
    let _actual = line_parts.nth(1);

    let data = _actual.unwrap_or("");
    if data.is_empty() {
        return 0;
    }

    let mut left_right = data.split(" | ");

    let _left = left_right.next();

    if _left.is_none() {
        return 0;
    }

    let winning_set: HashSet<String> = HashSet::from_iter(
        _left
            .unwrap()
            .split(' ')
            .map(|x| x.trim().to_string())
            .filter(|x| !x.is_empty()),
    );

    let _right = left_right.next();
    if _right.is_none() {
        return 0;
    }
    let count = _right
        .unwrap()
        .split(' ')
        .map(|x| x.trim().to_string())
        .filter(|x| !x.is_empty() && winning_set.contains(x))
        .fold(0, |acc, _e| acc + 1);

    count
}

pub fn scratch_cards_two(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    let scores = reader
        .lines()
        .map(|line| {
            if let Ok(line_content) = line {
                return line_content;
            }
            "".to_string()
        })
        .map(process_line)
        .collect::<Vec<_>>();

    // Cred @ChristopherBiscardi

    // Initialise count of each card to 1
    let card_counts = (0..scores.len())
        .map(|idx| (idx, 1))
        // Faster than HashMap as keys are traversed from low to high
        .collect::<BTreeMap<_, u32>>();

    let result = scores
        .iter()
        .enumerate()
        .fold(card_counts, |mut acc, (index, score)| {
            for i in (index + 1)..(index + 1 + *score as usize) {
                let to_add = *acc.get(&index).unwrap();
                acc.entry(i).and_modify(|value| *value += to_add);
            }
            acc
        })
        .values()
        .sum::<u32>();

    println!("Sum: {}", result);
}
