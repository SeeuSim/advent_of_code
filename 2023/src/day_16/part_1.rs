use crate::utils::extract_file;
use std::{
    collections::{HashSet, VecDeque},
    io::BufRead,
};

fn nav_grid(grid: &Vec<Vec<char>>) -> u32 {
    let (n_r, n_c) = (grid.len(), grid[0].len());

    let mut queue = VecDeque::from([
        // r, c, dr, dc
        (0_i8, -1_i8, 0, 1),
    ]);

    let mut visited = HashSet::new();

    while queue.len() > 0 {
        let (r, c, delta_r, delta_c) = match queue.pop_front() {
            Some(v) => v,
            None => break,
        };

        let r = r + delta_r;
        let c = c + delta_c;

        if r < 0 || r as usize >= n_r || c < 0 || c as usize >= n_c {
            continue;
        }

        let curr_symbol = grid[r as usize][c as usize];

        // Going in curr direction
        if curr_symbol == '.'
            // Going east/west in curr direction
            || (curr_symbol == '-' && delta_c != 0)
            // Going north/south in curr_direction
            || (curr_symbol == '|' && delta_r != 0)
        {
            // row and column have already been modified
            // simply propagate the delta
            let nxt = (r, c, delta_r, delta_c);
            if !visited.contains(&nxt) {
                visited.insert(nxt.clone());
                queue.push_back(nxt);
            }
        // In all these cases, the deltas are flipped and negated.
        // East: (0, 1) => (-1, 0) North
        // West: (0, -1) => (1, 0) South
        // North: (-1, 0) => (0, 1) East
        // South: (1, 0) => (0, -1) West
        } else if curr_symbol == '/' {
            let nxt = (r, c, -delta_c, -delta_r);
            if !visited.contains(&nxt) {
                visited.insert(nxt.clone());
                queue.push_back(nxt);
            }
        // In all these cases, the deltas are flipped.
        // East: (0, 1) => (1, 0) South
        // West: (0, -1) => (-1, 0) North
        // North: (-1, 0) => (0, -1) West
        // South: (1, 0) => (0, 1) East
        } else if curr_symbol == '\\' {
            let nxt = (r, c, delta_c, delta_r);
            if !visited.contains(&nxt) {
                visited.insert(nxt.clone());
                queue.push_back(nxt);
            }
        // Vertical or Horizontal splitter
        } else {
            let dirs = match curr_symbol {
                '|' => vec![(1, 0), (-1, 0)],
                '-' => vec![(0, 1), (0, -1)],
                _ => unreachable!(),
            };
            for (n_delta_r, n_delta_c) in dirs {
                let nxt = (r, c, n_delta_r, n_delta_c);
                if !visited.contains(&nxt) {
                    visited.insert(nxt.clone());
                    queue.push_back(nxt);
                }
            }
        }
    }
    let visited_coords: HashSet<(i8, i8)> =
        HashSet::from_iter(visited.iter().map(|(r, c, _, _)| (*r, *c)));
    visited_coords.len() as u32
}

pub fn the_floor_will_be_lava(file_name: &String) {
    let test = r#"
    .|...\....
    |.-.\.....
    .....|-...
    ........|.
    ..........
    .........\
    ..../.\\..
    .-.-/..|..
    .|....-|.\
    ..//.|....
    "#
    .trim()
    .split('\n')
    .map(|line| line.trim().chars().collect::<Vec<_>>())
    .collect::<Vec<_>>();

    let test_r = nav_grid(&test);

    assert!(test_r == 46, "Test: {test_r}");

    let reader = extract_file(file_name).expect("An error occurred while reading the file");

    let grid = reader
        .lines()
        .filter_map(std::result::Result::ok)
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let ans = nav_grid(&grid);

    println!("Answer: {ans}");
}
