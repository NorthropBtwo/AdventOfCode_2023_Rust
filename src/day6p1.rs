use std::fs;

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day6/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 6,
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

pub fn solution() -> u64 {
    608902
}


pub fn first_try() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();
    let mut product = 1;
    let mut line_iter = input_string.lines();

    let mut times = vec![];
    for time in  line_iter.next().unwrap_or(":").split(':').skip(1).next().unwrap_or_default().split_ascii_whitespace() {
        if let Ok(time) = time.parse::<u32>() {
            times.push(time);
        }
    }

    let mut distances = vec![];
    for time in  line_iter.next().unwrap_or(":").split(':').skip(1).next().unwrap_or_default().split_ascii_whitespace() {
        if let Ok(time) = time.parse::<u32>() {
            distances.push(time);
        }
    }

    for (time, distance) in times.into_iter().zip(distances.into_iter()) {
        let x1 = (-(time as f32) + (time.pow(2) as f32 - 4. * distance as f32).sqrt()) / (-2.);
        let x2 = (-(time as f32) - (time.pow(2) as f32 - 4. * distance as f32).sqrt()) / (-2.);
        let min = x1 as u32;
        let max = (x2 - 10.*f32::EPSILON) as u32;
        let num_of_wins = max - min;
        product *= num_of_wins;
    }

    product as u64
}