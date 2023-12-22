use crate::utils::extract_file;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
    io::BufRead,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct CrucibleState {
    heat_loss: u32,
    ro: usize,
    col: usize,
    delta_r: i16,
    delta_c: i16,
    cells_travelled: u8,
}

impl Ord for CrucibleState {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .heat_loss
            .cmp(&self.heat_loss)
            // .then_with(|| self.ro.cmp(&other.ro))
            // .then_with(|| self.col.cmp(&other.col))
    }
}

impl PartialOrd for CrucibleState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_loss(grid: &Vec<Vec<u32>>) -> u32 {
    let (n_r, n_c) = (grid.len(), grid[0].len());

    let mut heap = BinaryHeap::from([CrucibleState {
        heat_loss: 0,
        ro: 0,
        col: 0,
        delta_r: 0,
        delta_c: 0,
        cells_travelled: 0,
    }]);

    let mut visited = HashSet::new();

    while let Some(CrucibleState {
        heat_loss,
        ro,
        col,
        delta_r,
        delta_c,
        cells_travelled,
    }) = heap.pop()
    {
        if ro == n_r - 1 && col == n_c - 1 {
            return heat_loss;
        }

        let key = (ro, col, delta_r, delta_c, cells_travelled);

        if visited.contains(&key) {
            continue;
        }
        visited.insert(key);

        if cells_travelled < 3 && (delta_r, delta_c) != (0, 0) {
            let (new_ro, new_col) = (ro as i16 + delta_r, col as i16 + delta_c);
            if new_ro >= 0 && new_ro < n_r as i16 && new_col >= 0 && new_col < n_c as i16 {
                heap.push(CrucibleState {
                    heat_loss: heat_loss + grid[new_ro as usize][new_col as usize],
                    ro: new_ro as usize,
                    col: new_col as usize,
                    delta_r,
                    delta_c,
                    cells_travelled: cells_travelled + 1,
                });
            }
        }

        for (new_delta_r, new_delta_c) in vec![(0, 1), (1, 0), (0, -1), (-1, 0)] {
            if (new_delta_r, new_delta_c).ne(&(delta_r, delta_c))
                && (new_delta_r, new_delta_c).ne(&(-delta_r, -delta_c))
            {
                let (new_ro, new_col) = (ro as i16 + new_delta_r, col as i16 + new_delta_c);
                if new_ro >= 0 && new_ro < n_r as i16 && new_col >= 0 && new_col < n_c as i16 {
                    heap.push(CrucibleState {
                        heat_loss: heat_loss + grid[new_ro as usize][new_col as usize],
                        ro: new_ro as usize,
                        col: new_col as usize,
                        delta_r: new_delta_r,
                        delta_c: new_delta_c,
                        cells_travelled: 1,
                    });
                }
            }
        }
    }

    0
}

pub fn placeholder(file_name: &String) {
    let test_grid = "
    2413432311323
    3215453535623
    3255245654254
    3446585845452
    4546657867536
    1438598798454
    4457876987766
    3637877979653
    4654967986887
    4564679986453
    1224686865563
    2546548887735
    4322674655533
    "
    .trim()
    .lines()
    .map(|line| {
        line.chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

    let test_score = get_loss(&test_grid);
    assert!(test_score == 102, "Test Score Found: {test_score}");

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

    let ans = get_loss(&grid);

    println!("Ans: {ans}");
}
