use crate::utils::extract_file;
use std::io::BufRead;

use super::part_1;

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

fn get_pattern_score(pattern: &Vec<Vec<char>>, avoid: Option<(i32, i32)>) -> (i32, i32) {
    let r_max = pattern.len() as i32;
    let c_max = pattern[0].len() as i32;
    let (avoid_r, avoid_c) = match avoid {
        Some(v) => v,
        None => (-1, -1)
    };

    let mut hori_line = -1;
    for start_row in 0..r_max - 1 {
        if avoid_r != start_row && get_hori_line(pattern, start_row) {
            hori_line = start_row;
            break;
        }
    }

    let mut vert_line = -1;
    for start_col in 0..c_max - 1 {
        if avoid_c != start_col && get_vert_line(pattern, start_col) {
            vert_line = start_col;
            break;
        }
    }
    
    (hori_line, vert_line)
}

fn get_pattern_score_two(pattern: &Vec<Vec<char>>) -> i32 {
    let mut pattern = pattern.clone();
    
    let original_score = get_pattern_score(&pattern, None);
    
    for r in 0..pattern.len() {
        for c in 0..pattern[0].len() {
            let need_modify = pattern[r][c] == '#';
            if need_modify {
                pattern[r][c] = '.';
            }

            let new_score = get_pattern_score(&pattern, Some(original_score));

            if ![original_score, (-1, -1)].contains(&new_score) {
                if new_score.0 != -1 {
                    return (new_score.0 + 1) * 100;
                }
                return new_score.1 + 1;
            }

            if need_modify {
                pattern[r][c] = '#';
            }
            
        }
    }
    0    
}

pub fn point_of_incidence_two(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    let mut pat_1 = vec![
        "#.##..##.",
        "..#.##.#.",
        "##......#",
        "##......#",
        "..#.##.#.",
        "..##..##.",
        "#.#.##.#.",
    ]
    .iter()
    .map(|&v| v.chars().collect())
    .collect();

    assert!(get_pattern_score_two(&mut pat_1) == 300);

    let mut pat_2 = vec![
        "#...##..#",
        "#....#..#",
        "..##..###",
        "#####.##.",
        "#####.##.",
        "..##..###",
        "#....#..#",
    ]
    .iter()
    .map(|&v| v.chars().collect())
    .collect();
    assert!(get_pattern_score_two(&mut pat_2) == 100);


    let mut patterns = Vec::new();

    reader.lines().filter_map(std::result::Result::ok).fold(&mut patterns, |acc, e| {
        let nline = e.trim();
        if acc.len() == 0 && e.len() > 0 {
            acc.push(vec![nline.chars().collect::<Vec<_>>()]);
        } else if e.len() > 0 {
            let last_idx = acc.len() - 1;
            acc[last_idx].push(nline.chars().collect::<Vec<_>>())
        } else {
            acc.push(vec![]);
        }
        acc
    });

    let ans = patterns.iter().map(|x| {
        get_pattern_score_two(x)
    }).sum::<i32>();

    println!("Answer: {ans}");

}
