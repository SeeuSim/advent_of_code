use crate::utils::extract_file;
use std::io::BufRead;

fn get_hori_line(pattern: &Vec<Vec<char>>, start_row: i32) -> bool {
    let r_max = pattern.len() as i32;
    let c_max = pattern[0].len() as i32;

    for col_index in 0..c_max {
        for row_index in 0..r_max {
            let oth_index = start_row * 2 + 1 - row_index;
            if 0 > oth_index || oth_index >= r_max {
                continue;
            }
            if pattern[row_index as usize][col_index as usize]
                != pattern[oth_index as usize][col_index as usize]
            {
                return false;
            }
        }
    }
    true
}

fn get_vert_line(pattern: &Vec<Vec<char>>, start_col: i32) -> bool {
    let r_max = pattern.len() as i32;
    let c_max = pattern[0].len() as i32;
    for row_index in 0..r_max {
        for col_index in 0..c_max {
            let oth_col = start_col * 2 + 1 - col_index;
            if 0 > oth_col || oth_col >= c_max {
                continue;
            }
            if pattern[row_index as usize][col_index as usize]
                != pattern[row_index as usize][oth_col as usize]
            {
                return false;
            }
        }
    }
    true
}

fn get_pattern_score(pattern: &Vec<Vec<char>>) -> i32 {
    let r_max = pattern.len() as i32;
    let c_max = pattern[0].len() as i32;

    let mut hori_line = -1;
    for start_row in 0..r_max - 1 {
        if get_hori_line(pattern, start_row) {
            hori_line = start_row;
            break;
        }
    }

    let mut vert_line = -1;
    for start_col in 0..c_max - 1 {
        if get_vert_line(pattern, start_col) {
            vert_line = start_col;
            break;
        }
    }
    vert_line + 1 + 100 * (hori_line + 1)
}

pub fn point_of_incidence(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    let pat_1 = vec![
        "#.##..##.".chars().collect::<Vec<_>>(),
        "..#.##.#.".chars().collect::<Vec<_>>(),
        "##......#".chars().collect::<Vec<_>>(),
        "##......#".chars().collect::<Vec<_>>(),
        "..#.##.#.".chars().collect::<Vec<_>>(),
        "..##..##.".chars().collect::<Vec<_>>(),
        "#.#.##.#.".chars().collect::<Vec<_>>(),
    ];

    assert!(get_pattern_score(&pat_1) == 5);

    let pat_2 = vec![
        "#...##..#".chars().collect::<Vec<_>>(),
        "#....#..#".chars().collect::<Vec<_>>(),
        "..##..###".chars().collect::<Vec<_>>(),
        "#####.##.".chars().collect::<Vec<_>>(),
        "#####.##.".chars().collect::<Vec<_>>(),
        "..##..###".chars().collect::<Vec<_>>(),
        "#....#..#".chars().collect::<Vec<_>>(),
    ];
    assert!(get_pattern_score(&pat_2) == 400);
    
    // Array of patterns
    // Each pattern is an array of strings
    let mut patterns: Vec<Vec<Vec<char>>> = Vec::new();
    reader
        .lines()
        .filter_map(std::result::Result::ok)
        .fold(&mut patterns, |acc, e| {
            let curr_line = e.trim();
            if acc.len() == 0 && curr_line.len() != 0 {
                acc.push(vec![curr_line.chars().collect::<Vec<_>>()]);
            } else if curr_line.len() != 0 {
                let last_index = acc.len() - 1;
                acc[last_index].push(curr_line.chars().collect::<Vec<_>>())
            } else {
                acc.push(vec![]);
            }
            acc
        });

    let ans = patterns.iter().map(|pat| {
        get_pattern_score(pat)
    }).sum::<i32>();

    println!("Answer: {}", ans);
}
