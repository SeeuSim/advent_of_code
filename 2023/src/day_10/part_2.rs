use crate::utils::extract_file;

use std::{collections::HashSet, collections::VecDeque, io::BufRead};

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

/**
 * Attempt at replicating success from other soln, but gives wrong vals
 */
pub fn pipe_maze_two(file_name: &String) {
    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    let mut s_loc = (usize::MAX, usize::MAX);
    let mut maze = reader
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

    let mut possible_s_vals = HashSet::from(['|', '-', 'J', 'L', '7', 'F']);

    let mut visited = HashSet::from([s_loc]);

    let mut queue = VecDeque::from([s_loc]);

    while queue.len() > 0 {
        let (r, c) = match queue.pop_front() {
            None => break,
            Some(val) => val,
        };

        let curr_pipe = maze[r][c];

        if r > 0
            && matches!(curr_pipe, 'S' | '|' | 'J' | 'L')
            && matches!(maze[r - 1][c], '|' | '7' | 'F')
            && !visited.contains(&(r - 1, c))
        {
            visited.insert((r - 1, c));
            queue.push_back((r - 1, c));
            if curr_pipe == 'S' {
                possible_s_vals = HashSet::from_iter(
                    possible_s_vals
                        .intersection(&HashSet::from(['|', 'J', 'L']))
                        .map(|&v| v),
                );
            }
        }

        if r < maze.len() - 1
            && matches!(curr_pipe, 'S' | '|' | '7' | 'F')
            && matches!(maze[r + 1][c], '|' | 'J' | 'L')
            && !visited.contains(&(r + 1, c))
        {
            visited.insert((r + 1, c));
            queue.push_back((r + 1, c));
            if curr_pipe == 'S' {
                possible_s_vals = HashSet::from_iter(
                    possible_s_vals
                        .intersection(&HashSet::from(['|', '7', 'F']))
                        .map(|&v| v),
                );
            }
        }

        if c > 0
            && matches!(curr_pipe, 'S' | '-' | 'J' | '7')
            && matches!(maze[r][c - 1], '-' | 'L' | 'F')
            && !visited.contains(&(r, c - 1))
        {
            visited.insert((r, c - 1));
            queue.push_back((r, c - 1));
            if curr_pipe == 'S' {
                possible_s_vals = HashSet::from_iter(
                    possible_s_vals
                        .intersection(&HashSet::from(['-', 'J', '7']))
                        .map(|&v| v),
                );
            }
        }

        if c < maze[0].len() - 1
            && matches!(curr_pipe, 'S' | '-' | 'L' | 'F')
            && matches!(maze[r][c + 1], '-' | 'J' | '7')
            && !visited.contains(&(r, c + 1))
        {
            visited.insert((r, c + 1));
            queue.push_back((r, c + 1));
            if curr_pipe == 'S' {
                possible_s_vals = HashSet::from_iter(
                    possible_s_vals
                        .intersection(&HashSet::from(['-', 'L', 'F']))
                        .map(|&v| v),
                );
            }
        }
    }

    // There should only be 1 S Value
    let s = match possible_s_vals.len() {
        1 => possible_s_vals.into_iter().next().unwrap(),
        _ => {
            eprintln!(
                "Supposed to have only 1 possible S value, but encountered this: {:?}",
                possible_s_vals
            );
            return;
        }
    };

    maze[s_loc.0][s_loc.1] = s;

    for r in 0..maze.len() {
        for c in 0..maze.len() {
            if !visited.contains(&(r, c)) {
                maze[r][c] = '.';
            }
        }
    }

    let mut outside_values: HashSet<(usize, usize)> = HashSet::new();

    for r in 0..maze.len() {
        let mut within = false;
        let mut up: Option<bool> = None;
        for c in 0..maze[0].len() {
            let chr = maze[r][c];
            match chr {
                '|' => {
                    if !up.is_none() {
                        eprintln!("Should be None: {}, {}, [{}]", r, c, chr);
                        std::process::exit(1);
                    }
                    within = !within;
                }
                '-' => {
                    if !up.is_some() {
                        eprintln!("Should be Some: {}, {}, [{}]", r, c, chr);
                        std::process::exit(1);
                    }
                }
                'L' | 'F' => {
                    if !up.is_none() {
                        eprintln!("Should be None: {}, {}, [{}]", r, c, chr);
                        std::process::exit(1);
                    }
                    up = Some(chr == 'L');
                }
                '7' | 'J' => {
                    if !up.is_some() {
                        eprintln!("Should be Some: {}, {}, [{}]", r, c, chr);
                        std::process::exit(1);
                    }
                    match up {
                        Some(v) => {
                            if (v && chr != 'J') || (!v && chr != '7') {
                                within = !within;
                            }
                        }
                        _ => {
                            if chr != '7' {
                                within = !within;
                            }
                        }
                    }
                    up = None;
                }
                '.' => {}
                _ => unreachable!(),
            }
            if !within {
                outside_values.insert((r, c));
            }
        }
    }

    println!(
        "Answer: {}",
        maze.len() * maze[0].len() - outside_values.union(&visited).count()
    );
}

/**
 * Broken solution, overcounts by 6
 */
pub fn og_pipe_maze_two(file_name: &String) {
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
