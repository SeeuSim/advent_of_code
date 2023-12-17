use crate::utils::extract_file;

use std::{
    collections::{BTreeMap, HashSet},
    io::BufRead,
};

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub fn pipe_maze_two(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    let mut s_loc = (usize::MAX, usize::MAX);
    let maze = reader
        .lines()
        .filter_map(std::result::Result::ok)
        .enumerate()
        .map(|(lnum, line)| {
            let out = line.chars();
            let pos = out.clone().position(|c| c == 'S');
            if let Some(col) = pos {
                s_loc = (lnum, col);
            }
            out.collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let (mut pos, mut dir) = {
        if matches!(maze[s_loc.0 - 1][s_loc.1], '|' | '7' | 'F') {
            ((s_loc.0 - 1, s_loc.1), Dir::Up)
        } else if matches!(maze[s_loc.0 + 1][s_loc.1], '|' | 'J' | 'L') {
            ((s_loc.0 + 1, s_loc.1), Dir::Down)
        } else {
            ((s_loc.0, s_loc.1 - 1), Dir::Left)
        }
    };

    let mut pipes = HashSet::from([s_loc]);

    loop {
        let curr_p = maze[pos.0][pos.1];

        pipes.insert(pos);

        match (curr_p, dir) {
            ('|', Dir::Down) => pos = (pos.0 + 1, pos.1),
            ('|', Dir::Up) => pos = (pos.0 - 1, pos.1),
            ('-', Dir::Left) => pos = (pos.0, pos.1 - 1),
            ('-', Dir::Right) => pos = (pos.0, pos.1 + 1),
            // Going down or up, hit a right branch
            ('L', Dir::Down) | ('F', Dir::Up) => {
                pos = (pos.0, pos.1 + 1);
                dir = Dir::Right;
            }
            // Going Left or Right, hit an Up branch
            ('L', Dir::Left) | ('J', Dir::Right) => {
                pos = (pos.0 - 1, pos.1);
                dir = Dir::Up;
            }
            // Going Up or Down, hit a Left branch
            ('7', Dir::Up) | ('J', Dir::Down) => {
                pos = (pos.0, pos.1 - 1);
                dir = Dir::Left;
            }
            // Going Right or Left, hit a down branch
            ('7', Dir::Right) | ('F', Dir::Left) => {
                pos = (pos.0 + 1, pos.1);
                dir = Dir::Down;
            }
            // Returned to the start, end the loop
            ('S', _) => break,
            (_, _) => unreachable!(),
        }
    }

    let mut ans = 0;

    for r in 0..maze.len() {
        for c in 0..maze[0].len() {
            if pipes.contains(&(r, c)) {
                continue;
            }
            let mut count = 0;
            for i in 0..c {
                // Count inversions up to (but not including) c
                if !pipes.contains(&(r, i)) {
                    continue;
                }
                if vec!['J', 'L', '|'].contains(&maze[r][i]) {
                    count += 1;
                }
            }

            if count % 2 == 1 {
                ans += 1;
            }
        }
    }

    // Actl Ans is 317, overcount by 6?
    // Possibly need to replace 'S' with sth else
    println!("Count: {}", ans);
}
