use std::fs;


use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day9/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 9,
        part_nr: 2,
        day_funcs: get_function_list(),
        solution: solution(),
    }
}

pub fn get_function_list() -> Vec<DayFunc> {
    vec![
        DayFunc{name: "from_part_1", func : from_part_1},
    ]
}

pub fn solution() -> u64 {
    919
}


pub fn from_part_1() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();
    let mut sum = 0;

    for line in input_string.lines() {
        let first_sequence = line.split_ascii_whitespace().filter_map(|x| x.parse::<i32>().ok()).collect::<Vec<i32>>();
        if first_sequence.len() > 0 {
            sum += process_sequence_line2(first_sequence);
        }
    }

    sum as u64
}

fn process_sequence_line2(line : Vec<i32>) -> i32 {
    if line.iter().sum::<i32>() == 0 {
        0
    } else {
        let mut line_blow = vec![];
        for i in 1..line.len() {
            line_blow.push(line[i]-line[i-1]);
        }
        *line.first().unwrap() - process_sequence_line2(line_blow)
    }
}