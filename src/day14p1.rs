use std::{collections::{HashMap, VecDeque}, fs};

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day14/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 14,
        part_nr: 1,
        day_funcs: get_function_list(),
        solution: solution(),
    }
}

pub fn get_function_list() -> Vec<DayFunc> {
    vec![
        DayFunc{name: "first_try", func : first_try},
    ]
}

pub fn solution() -> u64 {
    109665
}


pub fn first_try() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();
    let mut sum = 0;

    let line_length = input_string.lines().next().unwrap().len();
    let mut space_list : Vec<VecDeque<usize>> = vec![VecDeque::new(); line_length];
    let line_height = input_string.lines().count();

    for (y, line) in input_string.lines().map(|x| x.as_bytes()).enumerate() {
        for (x, c) in line.iter().enumerate() {
            match *c {
                b'.' => {
                    space_list[x].push_back(y);
                },
                b'#' => {
                    space_list[x].clear();
                },
                b'O' => {
                    if let Some(empty_y) = space_list[x].pop_front() {
                        sum += line_height - empty_y;
                        space_list[x].push_back(y);
                    } else {
                        sum += line_height - y;
                    }
                }
                _ => (),
            }
        }
    }

    sum as u64
}

