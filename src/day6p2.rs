use std::fs;

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day6/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 6,
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
    46173809
}


pub fn first_try() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();
    let mut product = 1;
    let mut line_iter = input_string.lines();

    let time = line_iter.next().unwrap_or(":").split(':').skip(1).next().unwrap_or_default().split_ascii_whitespace().collect::<String>();
    let distance = line_iter.next().unwrap_or(":").split(':').skip(1).next().unwrap_or_default().split_ascii_whitespace().collect::<String>();

    if let(Ok(time), Ok(distance)) = (time.parse::<f64>(), distance.parse::<f64>()) {
        let x1 = (-time + (time.powf(2.) - 4. * distance).sqrt()) / (-2.);
        let x2 = (-time - (time.powf(2.) - 4. * distance).sqrt()) / (-2.);
        let min = x1 as u32;
        let max = (x2 - 10.*f64::EPSILON) as u32;
        let num_of_wins = max - min;
        product *= num_of_wins;
    }

    product as u64
}