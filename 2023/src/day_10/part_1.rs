use num::Float;

use crate::utils::extract_file;
use std::{
    collections::{HashMap, VecDeque},
    io::BufRead,
};

pub fn placeholder(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    let mut s_loc = (usize::MAX, usize::MAX);

    let maze = reader
        .lines()
        .enumerate()
        .filter_map(|(lnum, v)| match v.ok() {
            None => None,
            Some(line) => {
                let out = line;

                let f_s = out
                    .clone()
                    .chars()
                    .enumerate()
                    .filter(|(_, v)| v.eq(&'S'))
                    .next()
                    .unwrap_or((usize::MAX, 'v'));

                if f_s.0 < usize::MAX {
                    s_loc = (lnum, f_s.0);
                }

                Some(out.chars().collect::<Vec<_>>())
            }
        })
        .collect::<Vec<_>>();

    let mut visited: HashMap<(usize, usize), i32> = HashMap::from([(s_loc, 0)]);

    let mut queue: VecDeque<(usize, usize)> = VecDeque::from([s_loc]);

    let north_chars = vec!['F', '7', '|'];
    let south_chars = vec!['L', 'J', '|'];
    let west_chars = vec!['-', 'F', 'L'];
    let east_chars = vec!['-', 'J', '7'];

    /*
    The pipes are arranged in a two-dimensional grid of tiles:

    | is a vertical pipe connecting north and south.
    - is a horizontal pipe connecting east and west.
    L is a 90-degree bend connecting north and east.
    J is a 90-degree bend connecting north and west.
    7 is a 90-degree bend connecting south and west.
    F is a 90-degree bend connecting south and east.
    . is ground; there is no pipe in this tile.
    S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
    */
    while queue.len() > 0 {
        let pos = queue.pop_front().unwrap();

        let (r, c) = pos;
        let d = visited.get(&pos).unwrap();

        // Top
        // F, 7, J, |
        if r > 0 && north_chars.contains(&maze[r - 1][c]) {
            let next = (r - 1, c);
            let c_dist =  visited.entry(next).or_insert(i32::MAX);
            *c_dist = *c_dist.min(&mut (*d + 1));
            queue.push_back(next);
        }

        // Bottom
        if r < maze.len() - 1 && south_chars.contains(&maze[r + 1][c]) {
            let next = (r + 1, c);
            queue.push_back(next);
        }

        // Left
        if c > 0 && west_chars.contains(&maze[r][c - 1]) {
            let next = (r, c - 1);
            queue.push_back(next);
        }

        // Right
        if c < maze[0].len() - 1 && east_chars.contains(&maze[r][c + 1]) {
            let next = (r, c + 1);
            queue.push_back(next);
        }
    }
}
