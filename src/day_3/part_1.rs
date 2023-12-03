// Gear Ratios

use regex::Regex;
use std::cmp::{max, min};
use std::io::{BufRead, Error};

use crate::utils::extract_file;

pub fn gear_ratio(file_name: &String) {
    let reader = extract_file(file_name).expect("An Error occurred while reading the file");

    let re = Regex::new(r"(\d+)").unwrap();

    let results: Vec<Result<String, Error>> = reader.lines().into_iter().collect();
    let mut sm = 0;
    for line_num in 0..results.len() {
        let line = &results[line_num];
        if let Ok(line_content) = line {
            let line_len = line_content.len();
            let content = line_content.chars().into_iter().collect::<Vec<char>>();
            sm += re
                .captures_iter(line_content)
                .map(|capture| {
                    let _match = capture.get(0).unwrap();
                    let l = _match.start();
                    let r = _match.end() - 1;
                    let val = _match.as_str().parse::<u32>().unwrap_or(0);

                    let mut found = false;
                    // Check Left
                    if l > 0 && content[l - 1] != '.' {
                        found = true;
                    }
                    // Check Right
                    if !found && r < line_len - 1 && content[r + 1] != '.' {
                        found = true;
                    }
                    // Check along Top (incl diag)
                    if !found && line_num > 0 {
                        let top_line = &results[line_num - 1];
                        if let Ok(top_line_content) = top_line {
                            let top_chars =
                                top_line_content.chars().into_iter().collect::<Vec<char>>();
                            for i in max(l.saturating_sub(1), 0)..min(r.saturating_add(2), line_len)
                            {
                                if top_chars[i] != '.' {
                                    found = true;
                                    break;
                                }
                            }
                        }
                    }
                    // Check along Bottom (incl diag)
                    if !found && line_num < results.len() - 1 {
                        let bottom_line = &results[line_num + 1];
                        if let Ok(bottom_line_content) = bottom_line {
                            let bottom_chars = bottom_line_content
                                .chars()
                                .into_iter()
                                .collect::<Vec<char>>();
                            for i in max(l.saturating_sub(1), 0)..min(r.saturating_add(2), line_len)
                            {
                                if bottom_chars[i] != '.' {
                                    found = true;
                                    break;
                                }
                            }
                        }
                    }
                    if found {
                        val
                    } else {
                        0
                    }
                })
                .sum::<u32>();
        }
    }
    println!("Sum: {}", sm);
}
