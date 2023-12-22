use crate::utils::extract_file;
use std::io::BufRead;

pub fn placeholder(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    let grid = reader
        .lines()
        .filter_map(std::result::Result::ok)
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let accum_loss = vec![vec![u32::MAX; grid[0].len()]; grid.len()];
}
