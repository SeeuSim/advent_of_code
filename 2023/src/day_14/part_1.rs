use crate::utils::extract_file;
use std::io::BufRead;

fn process_pattern(pattern: &Vec<Vec<char>>) -> u32 {
    let (r, c) = (pattern.len(), pattern[0].len());

    let mut res = 0;
    for c_idx in 0..c {
        let mut n_limit = 0;
        let mut o_count = 0;
        for r_idx in 0..r {
            let curr = pattern[r_idx][c_idx];
            if curr == '#' {
                // For o_count iters, add to sum.
                res += (0..o_count).map(|x| (c - x - n_limit) as u32).sum::<u32>();
                n_limit = r_idx + 1;
                o_count = 0;
            } else if curr == 'O' {
                o_count += 1;
            }
        }
        res += (0..o_count).map(|x| (c - x - n_limit) as u32).sum::<u32>();
    }
    res
}

pub fn placeholder(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    let pat_1 = "
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

    let ans_1 = process_pattern(&pat_1);

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

    let pat = reader.lines().filter_map(|x| {
        match x.ok() {
            Some(v) => {
                Some(v.chars().collect::<Vec<_>>())
            },
            None => None
        }
    }).collect::<Vec<_>>();

    let ans = process_pattern(&pat);

    println!("Answer: {ans}");
}
