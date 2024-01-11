use std::fs;

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day2/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 2,
        part_nr: 1,
        day_funcs: get_function_list(),
        solution: solution(),
    }
}

pub fn get_function_list() -> Vec<DayFunc> {
    vec![
        DayFunc{name: "First Try", func : first_try},
    ]
}

pub fn solution() -> u32 {
    2176
}

pub fn first_try() -> u32 {
    let contents = fs::read_to_string(INPUT_PATH);
    match contents {
        Ok(contents) => {
            let mut sum = 0;
            for line in contents.lines() {
                let mut game_nr = 0;
                for (i, line_part) in line.split(':').enumerate() {
                    match i {
                        0 => {
                            if let Some(nr) = line_part.split(' ').last() {
                                game_nr = nr.parse().unwrap_or(0);
                            }
                        },
                        1 => {
                            let mut draw_nr = 0;
                            for word in line_part.split_whitespace() {
                                draw_nr = word.parse::<i32>().unwrap_or(draw_nr);
                                match word.trim_matches(&[',', ';']) {
                                    "red" => {
                                        if draw_nr > 12 {
                                            game_nr = 0;
                                        }
                                    }
                                    "green" => {
                                        if draw_nr > 13 {
                                            game_nr = 0;
                                        }
                                    }
                                    "blue" => {
                                        if draw_nr > 14 {
                                            game_nr = 0;
                                        }
                                    }
                                    _ => (),
                                }
                            }
                        }
                        _ => (),
                    }
                }
                sum += game_nr;
            }
            return sum;
        },
        Err(error) => println!("{}", error),
    }
    0
}