//Cube Conundrum

use std::fs::File;
use std::io::{BufRead, BufReader};

use std::path::Path;

fn process_line(line: &String) -> bool {
    let actl_line = line.split(": ").into_iter().nth(1).unwrap();
    actl_line
        .split("; ")
        .map(|game| {
            let _red_limit = 12;
            let _green_limit = 13;
            let _blue_limit = 14;

            let is_pos = game.split(", ")
                .map(|val| {
                    if val.ends_with("red") {
                        val.split(" ")
                            .into_iter()
                            .nth(0)
                            .unwrap()
                            .parse::<i32>()
                            .unwrap_or(0)
                            <= _red_limit
                    } else if val.ends_with("blue") {
                        val.split(" ")
                            .into_iter()
                            .nth(0)
                            .unwrap()
                            .parse::<i32>()
                            .unwrap_or(0)
                            <= _blue_limit
                    } else if val.ends_with("green") {
                        val.split(" ")
                            .into_iter()
                            .nth(0)
                            .unwrap()
                            .parse::<i32>()
                            .unwrap_or(0)
                            <= _green_limit
                    } else {
                        false
                    }
                })
                .all(|v| v);
              
              println!("Possible: {}", is_pos);

              is_pos
        })
        .all(|v| v)
}

pub fn cube_conundrum(file_name: &String) {
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

    let out = reader
        .lines()
        .into_iter()
        .map(|line| match line {
            Ok(line_content) => process_line(&line_content),
            Err(_err) => false,
        })
        .enumerate()
        .map(
            |(index, is_game_possible)| {
                if is_game_possible {
                    index + 1
                } else {
                    0
                }
            },
        );

    println!("Sum: {}", out.into_iter().sum::<usize>())
}
