use std::fs;

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day1/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 1,
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
    55413
}

pub fn first_try() -> u64 {
    let contents = fs::read_to_string(INPUT_PATH);
    match contents {
        Ok(contents) => {
            let mut sum = 0;
            for line in contents.lines() {
                let mut first = None;
                let mut last = None;
                for ic in 0..line.len() {
                    if let Some(value) = line_position_to_int(line, ic) {
                        if first == None {
                            first = Some(value);
                        }
                        last = Some(value);
                    } 
                }
                sum += (first.unwrap_or(0) * 10 + last.unwrap_or(0)) as u32;
            }
            return sum as u64;
        },
        Err(error) => println!("{}", error),
    }
    0
}

fn line_position_to_int(line: &str, idx: usize) -> Option<u8> {
    let mut retval = None;
    if line.as_bytes()[idx].is_ascii_digit() {
        retval = Some(line.as_bytes()[idx] - 48);
    } else {
        let line_slice = &line[idx..line.len()];
        if line_slice.starts_with("one") {
            retval = Some(1);
        } else if line_slice.starts_with("two") {
            retval = Some(2);
        } else if line_slice.starts_with("three") {
            retval = Some(3);
        } else if line_slice.starts_with("four") {
            retval = Some(4);
        } else if line_slice.starts_with("five") {
            retval = Some(5);
        } else if line_slice.starts_with("six") {
            retval = Some(6);
        } else if line_slice.starts_with("seven") {
            retval = Some(7);
        } else if line_slice.starts_with("eight") {
            retval = Some(8);
        } else if line_slice.starts_with("nine") {
            retval = Some(9);
        }
    }
    retval
}