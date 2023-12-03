// Gear Ratio Part 2

use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use std::path::Path;

pub fn gear_ratio_two(file_name: &String) {
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

    let re = Regex::new(r"(\d+)").unwrap();

    let reader = BufReader::new(file);
    let results: Vec<Result<String, Error>> = reader.lines().into_iter().collect();

    let mut coords: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for line_num in 0..results.len() {
        let line = &results[line_num];
        if let Ok(line_content) = line {
            let line_len = line_content.len();
            let content = line_content.chars().into_iter().collect::<Vec<char>>();
            re.captures_iter(line_content).for_each(|capture| {
                let _match = capture.get(0).unwrap();
                let l = _match.start();
                let r = _match.end() - 1;
                let val = _match.as_str().parse::<u32>().unwrap_or(0);

                // Check Left
                if l > 0 && content[l - 1] == '*' {
                    let key = (line_num, l - 1);
                    let new_val = match coords.get(&key) {
                        Some(nv) => [nv.clone(), [val].to_vec()].concat(),
                        None => {
                            vec![val]
                        }
                    };
                    coords.insert(key, new_val);
                }
                // Check Right
                if r < line_len - 1 && content[r + 1] == '*' {
                    let key = (line_num, r + 1);
                    let new_val = match coords.get(&key) {
                        Some(nv) => [nv.clone(), [val].to_vec()].concat(),
                        None => {
                            vec![val]
                        }
                    };
                    coords.insert(key, new_val);
                }
                // Check along Top (incl diag)
                if line_num > 0 {
                    let top_line = &results[line_num - 1];
                    if let Ok(top_line_content) = top_line {
                        let top_chars = top_line_content.chars().into_iter().collect::<Vec<char>>();
                        for i in max(l.saturating_sub(1), 0)..min(r.saturating_add(2), line_len) {
                            if top_chars[i] == '*' {
                                let key = (line_num - 1, i);
                                let new_val = match coords.get(&key) {
                                    Some(nv) => [nv.clone(), [val].to_vec()].concat(),
                                    None => {
                                        vec![val]
                                    }
                                };
                                coords.insert(key, new_val);
                            }
                        }
                    }
                }
                // Check along Bottom (incl diag)
                if line_num < results.len() - 1 {
                    let bottom_line = &results[line_num + 1];
                    if let Ok(bottom_line_content) = bottom_line {
                        let bottom_chars = bottom_line_content
                            .chars()
                            .into_iter()
                            .collect::<Vec<char>>();
                        for i in max(l.saturating_sub(1), 0)..min(r.saturating_add(2), line_len) {
                            if bottom_chars[i] == '*' {
                                let key = (line_num + 1, i);
                                let new_val = match coords.get(&key) {
                                    Some(nv) => [nv.clone(), [val].to_vec()].concat(),
                                    None => {
                                        vec![val]
                                    }
                                };
                                coords.insert(key, new_val);
                            }
                        }
                    }
                }
            })
        }
    }

    let sm: u32 = coords
        .values()
        .map(|parts| match parts.len() {
            2 => parts.iter().fold(1, |acc, &x| acc * x),
            _ => 0,
        })
        .sum();
    println!("Sum: {}", sm);
}
