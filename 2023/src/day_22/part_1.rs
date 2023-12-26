use crate::utils::extract_file;
use std::io::BufRead;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Brick {
    start: (usize, usize, usize),
    end: (usize, usize, usize),
}

impl Brick {
    fn parse(input: &String) -> Option<Brick> {
        let (_start, _end) = match input.split_once('~') {
            Some(v) => v,
            None => return None,
        };

        let start = _start
            .splitn(3, ',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let end = _end
            .splitn(3, ',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        if start.len() != 3 || end.len() != 3 {
            return None;
        }

        Some(Brick {
            start: (start[0], start[1], start[2]),
            end: (end[0], end[1], end[2]),
        })
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start
            .2
            .cmp(&other.start.2)
            .then_with(|| self.start.1.cmp(&other.start.1))
            .then_with(|| self.start.0.cmp(&other.start.0))
    }
}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_sort_bricks(input: &Vec<String>) -> Vec<Brick> {
    let mut bricks: Vec<Brick> = input.iter().filter_map(Brick::parse).collect::<Vec<_>>();
    bricks.sort_unstable();
    bricks = iter_map_dropped_bricks(bricks.into_iter())
        .map(|(brick, _)| brick)
        .collect::<Vec<_>>();
    bricks
}

fn iter_map_dropped_bricks(
    bricks: impl Iterator<Item = Brick>,
) -> impl Iterator<Item = (Brick, bool)> {
    let mut height_map = [0; 100];
    bricks.map(move |brick| {
        let highest_z = (brick.start.0..=brick.end.0)
            .flat_map(|x_index| {
                (brick.start.1..=brick.end.1).map(move |y_index| x_index * 10 + y_index)
            })
            .map(|i| height_map[i])
            .max()
            .unwrap_or(0);

        let z_delta = brick.start.2 - highest_z - 1;

        let new_brick = Brick {
            start: (brick.start.0, brick.start.1, brick.start.2 - z_delta),
            end: (brick.end.0, brick.end.1, brick.end.2 - z_delta),
        };

        for x_coord in new_brick.start.0..=new_brick.end.0 {
            for y_coord in new_brick.start.1..=new_brick.end.1 {
                height_map[x_coord * 10 + y_coord] = new_brick.end.2;
            }
        }

        (new_brick, new_brick.start.2 != brick.start.2)
    })
}

pub fn sand_slabs(file_name: &String) {
    let test = r#"
    1,0,1~1,2,1
    0,0,2~2,0,2
    0,2,3~2,2,3
    0,0,4~0,2,4
    2,0,5~2,2,5
    0,1,6~2,1,6
    1,1,8~1,1,9
    "#
    .trim()
    .split('\n')
    .map(|x| x.trim().to_string())
    .collect();

    let test_bricks = parse_sort_bricks(&test);

    let test_ans = (0..test_bricks.len())
        .filter(|&original_index| {
            iter_map_dropped_bricks(test_bricks.iter().enumerate().filter_map(
                move |(process_index, &brick)| (original_index != process_index).then_some(brick),
            ))
            .all(|(_, will_drop)| !will_drop)
        })
        .count();

    assert!(test_ans == 5, "Expected: 5, got: {test_ans}");

    let reader = extract_file(file_name).expect("An error occurred while reading the file");
    let input = reader.lines().filter_map(std::result::Result::ok).collect();
    let bricks = parse_sort_bricks(&input);

    let ans = (0..bricks.len())
        .filter(|&original_index| {
            iter_map_dropped_bricks(bricks.iter().enumerate().filter_map(
                move |(process_index, &brick)| (original_index != process_index).then_some(brick),
            ))
            .all(|(_, will_drop)| !will_drop)
        })
        .count();

    println!("Answer: {ans}");
}
