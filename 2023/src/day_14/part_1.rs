use crate::utils::extract_file;
use std::io::BufRead;

fn slide_top(matrix: &mut Vec<Vec<char>>) {
    let (r, c) = (matrix.len(), matrix[0].len());

    for c_idx in 0..c {
        let mut r_idx = 0;
        // iterate from north -> south
        while r_idx < r {
            while r_idx < r && matrix[r_idx][c_idx] == '#' {
                r_idx += 1;
            }
            let mut o_count = 0;
            let start = r_idx;
            while r_idx < r && matrix[r_idx][c_idx] != '#' {
                if matrix[r_idx][c_idx] == 'O' {
                    o_count += 1;
                }
                r_idx += 1;
            }
            for mut_r_i in start..start + o_count {
                matrix[mut_r_i][c_idx] = 'O';
            }
            for mut_r_i in start + o_count..r_idx {
                matrix[mut_r_i][c_idx] = '.';
            }
        }
    }
}

fn process_pattern(pattern: &mut Vec<Vec<char>>) -> u32 {
    slide_top(pattern);
    let (r, c) = (pattern.len(), pattern[0].len());

    let mut res = 0;
    for _r in 0..r {
        for _c in 0..c {
            if pattern[_r][_c] == 'O' {
                res += (r - _r) as u32;
            }
        }
    }
    res
}

pub fn parabolic_reflector_dish(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    let mut pat_1 = "
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
    "
    .trim()
    .split("\n")
    .map(|x| x.trim().chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();

    let ans_1 = process_pattern(&mut pat_1);

    /*
       10 9 8 7  34
       10 9 8    27
       10 | 4 3  17
       10        10
       8          8
       7          7
       7          7
       10 | 4    14
       8 | 4     12
       ------------
                 136
    */

    assert!(ans_1 == 136, "Ans: {ans_1}");

    let mut pat = reader
        .lines()
        .filter_map(|x| match x.ok() {
            Some(v) => Some(v.chars().collect::<Vec<_>>()),
            None => None,
        })
        .collect::<Vec<_>>();

    let ans = process_pattern(&mut pat);

    println!("Answer: {ans}");
}
