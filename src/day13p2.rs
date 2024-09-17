use std::{collections::HashMap, fs};

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day13/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 13,
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
    28235
}


pub fn first_try() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();
    let mut sum = 0;

    let mut puzzle = Vec::new();

    for line in input_string.lines().map(|x| x.as_bytes()) {
        
        if line.len() > 0 {
            puzzle.push(line);
        } else {
            if let Some(position) = find_horizontal_mirror(&puzzle) {
                sum += 100 * position;
            } else if let Some(position) = find_vertical_mirror(&puzzle) {
                sum += position
            }
            puzzle.clear();
        }


    }

    sum as u64
}

fn find_horizontal_mirror(puzzle: &Vec<&[u8]>) -> Option<usize> {

    for i in 1..puzzle.len() {
        if check_valid_mirror_position_horizontal(puzzle, i) {
            return Some(i);
        }
    }

    None
}

fn check_valid_mirror_position_horizontal(puzzle: &Vec<&[u8]>, pos : usize) -> bool {
    let mut num_of_errors = 0;
    let above = pos;
    let below = puzzle.len() - pos;
    for j in 0..above.min(below) {
        for i in 0..puzzle[0].len() {
            if puzzle[pos-1-j][i] != puzzle[pos+j][i] {
                if num_of_errors == 0 {
                    num_of_errors = 1;
                } else {
                    return false;
                }
            }
        }
    }
    num_of_errors == 1
}


fn find_vertical_mirror(puzzle: &Vec<&[u8]>) -> Option<usize> {

    for i in 1..puzzle[0].len() {
        if check_valid_mirror_position_vertical(puzzle, i) {
            return Some(i);
        }
    }

    None
}

fn check_valid_mirror_position_vertical(puzzle: &Vec<&[u8]>, pos : usize) -> bool {
    let mut num_of_errors = 0;
    let left = pos;
    let right = puzzle[0].len() - pos;
    for j in 0..left.min(right) {
        for i in 0..puzzle.len() {
            if puzzle[i][pos-1-j] != puzzle[i][pos+j] {
                if num_of_errors == 0 {
                    num_of_errors = 1;
                } else {
                    return false;
                }
            }
        }
    }
    num_of_errors == 1
}