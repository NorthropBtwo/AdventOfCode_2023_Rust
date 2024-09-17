use std::{fs, collections::HashMap};

use num_integer::Integer;

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day8/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 8,
        part_nr: 2,
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
    13385272668829
}


pub fn first_try() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();
    let mut input_string_lines = input_string.lines();

    let directions = input_string_lines.next().unwrap();
    let input_string_lines = input_string_lines.skip(1);

    let mut branch_table = HashMap::new();
    let mut start_positions = vec![];
    let mut factors_positions = vec![];

    for line in input_string_lines {
        let line = line.as_bytes();
        let position = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];
        branch_table.insert(position, (left,right));
        if position[2] == b'A' {
            start_positions.push(position);
        }
    }

    start_positions = start_positions;

    for mut position in start_positions {
        let mut num_of_setps = 0;
        let mut at_goal = false;
        while !at_goal {
            for direction in directions.as_bytes() {
                match direction {
                    b'L' => {
                        position = branch_table.get(position).unwrap().0;
                    },
                    b'R' => {
                        position = branch_table.get(position).unwrap().1;
                    },
                    _ => (),
                }
                num_of_setps += 1;

                if position[2] == b'Z' {
                    at_goal = true;
                    break;
                }

            }
        }
        factors_positions.push(num_of_setps as u64)
    }

    factors_positions.into_iter().fold(1 as u64, |acc, x| x.lcm(&acc))
}
