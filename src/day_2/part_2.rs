//Cube Conundrum Part 2

use std::cmp::max;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::path::Path;

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

    let data = actl_line
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
                .map(|inner| (inner[0], inner[1]))
                .collect::<HashMap<i32, i32>>();
            out.entry(0).or_insert(0);
            out.entry(1).or_insert(0);
            out.entry(2).or_insert(0);
            out
        })
        .reduce(|acc, e| {
            HashMap::from([
                (0, *max(acc.get(&0), e.get(&0)).unwrap_or(&0)),
                (1, *max(acc.get(&1), e.get(&1)).unwrap_or(&0)),
                (2, *max(acc.get(&2), e.get(&2)).unwrap_or(&0)),
            ])
        })
        .unwrap();
    data.get(&0).unwrap() * data.get(&1).unwrap() * data.get(&2).unwrap()
}

pub fn cube_conundrum_part_two(file_name: &String) {
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
