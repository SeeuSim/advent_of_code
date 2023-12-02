use clap::{Arg, Command};

use crate::day_1::part_1::trebuchet;
use crate::day_1::part_2::trebuchet_part_two;

pub mod day_1;

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
    
    if *day == "1" && *part == "1" {
        let file_name: &String = _matches.get_one("input-file").expect("file name is required");
        trebuchet(file_name);
    } else if *day == "1" && *part == "2" {
        let file_name: &String = _matches.get_one("input-file").expect("file name is required"); 
        trebuchet_part_two(file_name);
    }
}
