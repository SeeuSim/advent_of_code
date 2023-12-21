use crate::utils::extract_file;
use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
    iter::zip, ops::Index,
};

fn cycle(pat: &mut Vec<String>) {
    for _ in 0..4 {
        let transposed: Vec<String> = (0..pat[0].len())
            .map(|i| pat.iter().map(|row| row.chars().nth(i).unwrap()).collect())
            .collect();
        *pat = transposed;

        for row in pat.iter_mut() {
            *row = row
                .chars()
                .collect::<String>()
                .split('#')
                .map(|group| {
                    let mut sorted_group: Vec<char> = group.chars().collect();
                    sorted_group.sort_by(|a, b| b.cmp(a)); // Sort in descending order
                    sorted_group.iter().collect::<String>()
                })
                .collect::<Vec<String>>()
                .join("#")
                .chars()
                .collect();
        }
    
        // Reverse each row
        pat.iter_mut().for_each(|row| {
            *row = row.chars().rev().collect::<String>()
        });
    }
}

pub fn placeholder_two(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    let mut pat = reader
        .lines()
        .filter_map(std::result::Result::ok)
        .collect::<Vec<String>>();

    let mut seen = HashSet::new();
    let mut idx = Vec::new();

    let mut it = 0;
    loop {
        it += 1;
        cycle(&mut pat);
        if seen.contains(&pat) {
            break;
        }
        seen.insert(pat.clone());
        idx.push(pat.clone());
    }

    let f = idx.clone().iter().position(|v| {
        for i in 0..v.len() {
            if !v[i].eq_ignore_ascii_case(&pat[i]) {
                return false
            }
        }
        true
    }).unwrap();

    let v = &idx[(1000000000 - f) % (it - f) + f];

    let ans = v.iter().enumerate().map(|(idx, r)| {
        (r.chars().filter(|&x| x == 'O').count() * (v.len() - idx)) as u32
    }).sum::<u32>();

    println!("Answer: {ans}");

    /*
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

    let pat_2 = "
    .....#....
    ....#...O#
    ...OO##...
    .OO#......
    .....OOO#.
    .O#...O#.#
    ....O#....
    ......OOOO
    #...O###..
    #..OO#....
    "
    .trim()
    .split("\n")
    .map(|x| x.trim().chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();

    cycle(&mut pat_1);

    for r in 0..pat_1.len() {
        for c in 0..pat_1[0].len() {
            assert!(pat_1[r][c] == pat_2[r][c], "{r},{c}");
        }
    }

    let pat_3 = "
    .....#....
    ....#...O#
    .....##...
    ..O#......
    .....OOO#.
    .O#...O#.#
    ....O#...O
    .......OOO
    #..OO###..
    #.OOO#...O
    "
    .trim()
    .split("\n")
    .map(|x| x.trim().chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();

    cycle(&mut pat_1);

    for r in 0..pat_1.len() {
        for c in 0..pat_1[0].len() {
            assert_eq!(pat_1[r][c], pat_3[r][c]);
        }
    }

    let pat_4 = "
    .....#....
    ....#...O#
    .....##...
    ..O#......
    .....OOO#.
    .O#...O#.#
    ....O#...O
    .......OOO
    #...O###.O
    #.OOO#...O
    "
    .trim()
    .split("\n")
    .map(|x| x.trim().chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();

    cycle(&mut pat_1);

    for r in 0..pat_1.len() {
        for c in 0..pat_1[0].len() {
            assert_eq!(pat_1[r][c], pat_4[r][c]);
        }
    }
    */

    /*
    let mut pat = reader
    .lines()
    .filter_map(|x| match x.ok() {
        Some(v) => Some(v.chars().collect::<Vec<_>>()),
        None => None,
    })
    .collect::<Vec<_>>();

    let ans = process_pat(&mut pat);

    println!("Answer: {ans}");
    */
}

/*
fn rotate_matrix(matrix: &mut Vec<Vec<char>>) {
    let n = matrix.len();

    for layer in 0..n / 2 {
        let first = layer;
        let last = n - 1 - layer;

        for i in first..last {
            let offset = i - first;
            let top = matrix[first][i];

            // Left -> Top
            matrix[first][i] = matrix[last - offset][first];
            // Bottom -> Left
            matrix[last - offset][first] = matrix[last][last - offset];
            // Right -> Bottom
            matrix[last][last - offset] = matrix[i][last];
            // Top -> Right
            matrix[i][last] = top;
        }
    }
}

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

fn cycle(pattern: &mut Vec<Vec<char>>) {
    for i in 0..4 {
        for _ in 0..(4 - i % 4) {
            rotate_matrix(pattern);
        }
        slide_top(pattern);
        for _ in 0..(i % 4) {
            rotate_matrix(pattern);
        }
    }
}

fn get_score(pattern: &Vec<Vec<char>>) -> u32 {
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

fn process_pat(pattern: &mut Vec<Vec<char>>) -> u32 {
    let mut seen = HashMap::new();
    let mut seen_grids = HashMap::new();

    for it in 0..10_u64.pow(9) {
        cycle(pattern);

        let hash = pattern.clone().join(&'\n').iter().collect::<String>();

        if seen.contains_key(&hash) {
            let period = it - seen.get(&hash).unwrap();
            let ans = seen_grids
                .get_mut(
                    &((10_u64.pow(9) - 1 - seen.get(&hash).unwrap()) % period
                        + seen.get(&hash).unwrap()),
                )
                .unwrap();
            return get_score(ans);
        }

        seen.insert(hash, it);
        seen_grids.insert(it, pattern.clone());
    }
    0
}
*/
