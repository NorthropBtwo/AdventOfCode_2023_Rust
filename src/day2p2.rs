/* https://adventofcode.com/2023/day/2 */
use std::{fs, cmp};

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day2/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 2,
        part_nr: 2,
        day_funcs: get_function_list(),
        solution: solution(),
    }
}

pub fn get_function_list() -> Vec<DayFunc> {
    vec![
        DayFunc{name: "First Try", func : first_try},
    ]
}

pub fn solution() -> u64 {
    63700
}

pub fn first_try() -> u64 {
    let contents = fs::read_to_string(INPUT_PATH);
    match contents {
        Ok(contents) => {
            let mut sum = 0;
            for line in contents.lines() {
                let mut draw_nr = 0;
                let mut min_red = 0;
                let mut min_green = 0;
                let mut min_blue = 0;
                for (i, line_part) in line.split(':').enumerate() {
                    match i {
                        0 => {
                        },
                        1 => {
                            for word in line_part.split_whitespace() {
                                draw_nr = word.parse::<u32>().unwrap_or(draw_nr);
                                match word.trim_matches(&[',', ';']) {
                                    "red" => {
                                        min_red = cmp::max(min_red, draw_nr);
                                    }
                                    "green" => {
                                        min_green = cmp::max(min_green, draw_nr);
                                    }
                                    "blue" => {
                                        min_blue = cmp::max(min_blue, draw_nr);
                                    }
                                    _ => (),
                                }
                            }
                        }
                        _ => (),
                    }
                }
                sum += min_red * min_green * min_blue;
            }
            return sum as u64;
        },
        Err(error) => println!("{}", error),
    }
    0
}