use crate::utils::extract_file;
use std::io::BufRead;

pub fn cosmic_expansion(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    // Expand the galaxy, row-wise
    let mut galaxy = reader
        .lines()
        .filter_map(std::result::Result::ok)
        .flat_map(|line| match line.clone().chars().position(|x| x == '#') {
            None => vec![line.chars().collect::<Vec<_>>(); 2],
            Some(_) => vec![line.chars().collect::<Vec<_>>()],
        })
        .collect::<Vec<_>>();

    let mut cols_to_expand = vec![];
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

    for col in cols_to_expand.into_iter().rev() {
        for r in 0..galaxy.len() {
            galaxy[r].insert(col, '.');
        }
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

    // 0 .. 438, or galaxy 1 .. galaxy 439
    let ans = (0..n_galaxies - 1)
        .into_iter()
        .flat_map(|start_index| {
            // 1 .. 439, or galaxy 2 .. galaxy 440
            (start_index + 1..n_galaxies)
                .into_iter()
                .map(|end_index| {
                    let ro = galaxies[end_index].0 as i32 - galaxies[start_index].0 as i32;
                    let col = galaxies[end_index].1 as i32 - galaxies[start_index].1 as i32;
                    let ro = match ro < 0 {
                        true => -ro,
                        false => ro
                    };
                    let col = match col < 0 {
                        true => -col,
                        false => col
                    };
                    ro + col
                })
                .collect::<Vec<_>>()
        })
        .sum::<i32>();

    println!("Ans: {}", ans);
}
