pub mod utils;

pub mod day_1;
pub mod day_2;
pub mod day_3;
pub mod day_4;
pub mod day_5;
pub mod day_6;
pub mod day_7;

use clap::{Arg, Command};
use std::collections::HashMap;

use crate::day_1::part_1::trebuchet;
use crate::day_1::part_2::trebuchet_part_two;
use crate::day_2::part_1::cube_conundrum;
use crate::day_2::part_2::cube_conundrum_part_two;
use crate::day_3::part_1::gear_ratio;
use crate::day_3::part_2::gear_ratio_two;
use crate::day_4::part_1::scratch_cards;
use crate::day_4::part_2::scratch_cards_two;
use crate::day_5::part_1::seed_fertiliser;
use crate::day_5::part_2::seed_fertiliser_two;
use crate::day_6::part_1::wait_for_it;
use crate::day_6::part_2::wait_for_it_two;
use crate::day_7::part_1::camel_cards;
use crate::day_7::part_2::placeholder_two;

type FunctionWithParamFileName = fn(&String);

fn main() {
    let _matches = Command::new("Advent Of Code 2023 Runner")
        .version("0.1.0")
        .author("Ong Seeu Sim <seeusimong@gmail.com>")
        .about("Parses question number and arguments")
        .arg(
            Arg::new("day")
                .short('d')
                .long("day")
                .help("The day number"),
        )
        .arg(
            Arg::new("part")
                .short('p')
                .long("prt")
                .help("The question part, if any"),
        )
        .arg(
            Arg::new("input-file")
                .short('f')
                .long("file")
                .help("The file input, if any"),
        )
        .get_matches();
    let day: &String = _matches.get_one("day").expect("Day is required");
    let part: &String = _matches.get_one("part").expect("Part is required");

    let day_fn_map = HashMap::from([
        (
            "1".to_string(),
            HashMap::from([
                ("1".to_string(), trebuchet as FunctionWithParamFileName),
                (
                    "2".to_string(),
                    trebuchet_part_two as FunctionWithParamFileName,
                ),
            ]),
        ),
        (
            "2".to_string(),
            HashMap::from([
                ("1".to_string(), cube_conundrum as FunctionWithParamFileName),
                (
                    "2".to_string(),
                    cube_conundrum_part_two as FunctionWithParamFileName,
                ),
            ]),
        ),
        (
            "3".to_string(),
            HashMap::from([
                ("1".to_string(), gear_ratio as FunctionWithParamFileName),
                ("2".to_string(), gear_ratio_two as FunctionWithParamFileName),
            ]),
        ),
        (
            "4".to_string(),
            HashMap::from([
                ("1".to_string(), scratch_cards as FunctionWithParamFileName),
                (
                    "2".to_string(),
                    scratch_cards_two as FunctionWithParamFileName,
                ),
            ]),
        ),
        (
            "5".to_string(),
            HashMap::from([
                (
                    "1".to_string(),
                    seed_fertiliser as FunctionWithParamFileName,
                ),
                (
                    "2".to_string(),
                    seed_fertiliser_two as FunctionWithParamFileName,
                ),
            ]),
        ),
        (
            "6".to_string(),
            HashMap::from([
                ("1".to_string(), wait_for_it as FunctionWithParamFileName),
                (
                    "2".to_string(),
                    wait_for_it_two as FunctionWithParamFileName,
                ),
            ]),
        ),
        (
            "7".to_string(),
            HashMap::from([
                ("1".to_string(), camel_cards as FunctionWithParamFileName),
                (
                    "2".to_string(),
                    placeholder_two as FunctionWithParamFileName,
                ),
            ]),
        ),
    ]);

    let file_name: &String = _matches
        .get_one("input-file")
        .expect("File name should be provided");
    if day_fn_map.contains_key(day) && day_fn_map.get(day).is_some() {
        let day_table = day_fn_map.get(day).unwrap();
        if day_table.contains_key(part) && day_table.get(part).is_some() {
            let part_fn = day_table.get(part).unwrap();
            let f = part_fn.to_owned();
            f(file_name);
        } else {
            eprintln!("Part {} not defined", part);
        }
    } else {
        eprintln!("Day {} not defined", day);
    }
}
