use crate::utils::extract_file;
use std::io::BufRead;

pub fn cosmic_expansion_two(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    let mut rows_to_expand = vec![];
    let mut cols_to_expand = vec![];

    let galaxy = reader
        .lines()
        .filter_map(std::result::Result::ok)
        .enumerate()
        .map(|(lnum, line)| match line.clone().chars().position(|x| x == '#') {
            None => {
                rows_to_expand.push(lnum);
                line.chars().collect::<Vec<_>>()
            },
            Some(_) => line.chars().collect::<Vec<_>>(),
        })
        .collect::<Vec<_>>();

    for c in 0..galaxy[0].len() {
        let mut galaxy_found = false;
        for r in 0..galaxy.len() {
            if galaxy[r][c] == '#' {
                galaxy_found = true;
                break;
            }
        }
        if galaxy_found {
            continue;
        }
        cols_to_expand.push(c);
    }

    let mut galaxies = vec![];

    for r in 0..galaxy.len() {
        for c in 0..galaxy[0].len() {
            if galaxy[r][c] == '#' {
                galaxies.push((r, c));
            }
        }
    }

    let n_galaxies = galaxies.len();

    let ans = (0..n_galaxies - 1).flat_map(|start_index| {
        (start_index + 1..n_galaxies).map(|end_index| {
            let (s_r, s_c) = galaxies[start_index];
            let (e_r, e_c) = galaxies[end_index];
            let (s_r, e_r) = match s_r < e_r {
                true => (s_r, e_r),
                _ => (e_r, s_r)
            };
            let (s_c, e_c) = match s_c < e_c {
                true => (s_c, e_c),
                _ => (e_c, s_c)
            };

            let ro = (s_r+1..e_r).filter(|x| rows_to_expand.contains(x)).count() as i64
                * (1000000 - 1) + e_r as i64 - s_r as i64;
            let col = (s_c+1..e_c).filter(|x| cols_to_expand.contains(x)).count() as i64
                * (1000000 - 1) + e_c as i64 - s_c as i64;

            ro + col

        }).collect::<Vec<_>>()
    }).sum::<i64>();

    println!("Ans: {}", ans);
}
