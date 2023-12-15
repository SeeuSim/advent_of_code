use crate::utils::extract_file;
use std::{
    io::BufRead,
};

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub fn placeholder(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    let _s_loc = (usize::MAX, usize::MAX);

    let maze = reader
        .lines()
        .filter_map(std::result::Result::ok)
        .fold("".to_string(), |mut acc, e| {
            acc.push_str(e.as_str());
            acc.push_str("\n");
            acc
        })
        .chars()
        .collect::<Vec<_>>();

    // println!("{:?}", maze);

    let width = maze.clone().into_iter().position(|x| x == '\n').unwrap();
    let start = maze.clone().into_iter().position(|x| x == 'S').unwrap();

    let (mut pos, mut dir) = {
        // Above, eliminating the newline char
        if matches!(maze[start - width - 1], '|' | '7' | 'F') {
            (start - width - 1, Dir::Up)
            // Below, skipping the newline char
        } else if matches!(maze[start + width + 1], '|' | 'L' | 'J') {
            (start + width + 1, Dir::Down)
            // Default to left, which shouldn't be the case
        } else {
            (start - 1, Dir::Left)
        }
    };

    // Finds the largest cycle to reach S, and then divides that by 2.
    let ans = (1 + std::iter::repeat(())
        .position(|_| unsafe {
            match (maze.get_unchecked(pos), dir) {
                ('|', Dir::Down) => pos += width + 1,
                ('|', Dir::Up) => pos -= width + 1,
                ('-', Dir::Left) => pos -= 1,
                ('-', Dir::Right) => pos += 1,
                // Going down or up, hit a right branch
                ('L', Dir::Down) | ('F', Dir::Up) => {
                    pos += 1;
                    dir = Dir::Right;
                }
                // Going Left or Right, hit an Up branch
                ('L', Dir::Left) | ('J', Dir::Right) => {
                    pos -= width + 1;
                    dir = Dir::Up;
                }
                // Going Up or Down, hit a Left branch
                ('7', Dir::Up) | ('J', Dir::Down) => {
                    pos -= 1;
                    dir = Dir::Left;
                }
                // Going Right or Left, hit a down branch
                ('7', Dir::Right) | ('F', Dir::Left) => {
                    pos += width + 1;
                    dir = Dir::Down;
                }
                // Returned to the start, end the loop
                ('S', _) => return true,
                (_, _) => unreachable!(),
            }
            false
        })
        .unwrap())
        / 2;

    println!("Answer: {}", ans);
}
