use std::fs;

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day15/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 15,
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
    513158
}


pub fn first_try() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();
    let mut sum = 0;

    let input = input_string.as_bytes();
    for step in input.split(|&c| c == b',') {
        let mut value = 0;
        for c in step {
            value += *c as u64;
            value *= 17;
            value &= 0xFF
        }
        sum += value as u64
    }

    
    sum
}

