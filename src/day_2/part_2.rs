//Cube Conundrum Part 2

use std::cmp::max;
use std::collections::HashMap;
use std::io::BufRead;

use crate::utils::extract_file;

fn process_game_val(game_val: &str) -> i32 {
    game_val
        .split(" ")
        .into_iter()
        .nth(0)
        .unwrap()
        .parse::<i32>()
        .unwrap_or(0)
}

fn process_line(line: &String) -> i32 {
    let actl_line = line.split(": ").into_iter().nth(1).unwrap();

    actl_line
        .split("; ")
        .map(|game| {
            let mut out = game
                .split(", ")
                .map(|val| {
                    if val.ends_with("red") {
                        vec![0, process_game_val(val)]
                    } else if val.ends_with("blue") {
                        vec![1, process_game_val(val)]
                    } else if val.ends_with("green") {
                        vec![2, process_game_val(val)]
                    } else {
                        vec![0, 0]
                    }
                })
                .into_iter()
                .map(|entry| (entry[0], entry[1]))
                .collect::<HashMap<i32, i32>>();
            for i in 0..3 {
                out.entry(i).or_insert(0);
            }
            out
        })
        .reduce(|acc, e| {
            HashMap::from_iter(
                (0..3)
                    .map(|v| (v, *max(acc.get(&v).unwrap_or(&0), e.get(&v).unwrap_or(&0))))
                    .into_iter(),
            )
        })
        .unwrap()
        .values()
        .into_iter()
        .map(|v| v.to_owned())
        .reduce(|acc, e| acc * e)
        .unwrap_or(0)
}

pub fn cube_conundrum_part_two(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    let out: i32 = reader
        .lines()
        .into_iter()
        .map(|line| match line {
            Ok(line_content) => process_line(&line_content),
            Err(_err) => 0,
        })
        .sum();

    println!("Sum: {}", out);
}
