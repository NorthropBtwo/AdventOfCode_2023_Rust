use std::fs;

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day4/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 4,
        part_nr: 1,
        day_funcs: get_function_list(),
        solution: solution(),
    }
}

pub fn get_function_list() -> Vec<DayFunc> {
    vec![
        DayFunc{name: "First Try", func : first_try},
        DayFunc{name: "timvisee(not mine)", func : timvisee},
    ]
}

pub fn solution() -> u64 {
    25571
}


pub fn first_try() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();
    let mut points = 0;

    for line in input_string.lines() {
        if let Some(number_str) = line.split(':').nth(1) {
            let mut number_str_iter = number_str.split('|');
            let winning_numbers = number_str_iter.next().unwrap();
            let my_numbers = number_str_iter.next().unwrap();

            let winning_numbers = winning_numbers.split_ascii_whitespace().map(|x| x.parse::<u32>());
            let mut winning_number_vec = vec![];
            for number in winning_numbers {
                if let Ok(number) = number {
                    winning_number_vec.push(number);
                }
            }

            let mut matching = 0;
            let my_numbers = my_numbers.split_ascii_whitespace().map(|x| x.parse::<u32>());
            for number in my_numbers {
                if let Ok(number) = number {
                    if winning_number_vec.contains(&number) {
                        matching += 1;
                    }
                }
            }

            if matching > 0 {
                points += 2_u32.pow(matching-1)
            }

        }

    }
    points as u64
}


pub fn timvisee() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();
    let input = input_string.as_bytes();
    let col = input.iter().position(|&b| b == b':').unwrap();
    let sep = input.iter().position(|&b| b == b'|').unwrap();
    input
        .split(|&b| b == b'\n')
        .map(|game| {
            let win_seq = &game[col + 1..sep];
            let win_count = game[sep + 1..]
                .chunks_exact(3)
                .map(|n| &n[1..])
                .filter(|n| win_seq.chunks_exact(3).map(|n| &n[1..]).any(|c| &c == n))
                .count() as u32;
            2usize.pow(win_count) >> 1
        })
        .sum::<usize>() as u64
}