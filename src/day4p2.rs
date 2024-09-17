use std::{fs, collections::HashMap};

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day4/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 4,
        part_nr: 2,
        day_funcs: get_function_list(),
        solution: solution(),
    }
}

pub fn get_function_list() -> Vec<DayFunc> {
    vec![
        DayFunc{name: "timvisee(not mine)", func : timvisee},
        DayFunc{name: "based_on_p1", func : based_on_p1},
        DayFunc{name: "with_hashmap", func : with_hashmap},
    ]
}

pub fn solution() -> u64 {
    8805731
}

pub fn timvisee() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();
    let input = input_string.as_bytes();
    let col = input.iter().position(|&b| b == b':').unwrap();
    let sep = input.iter().position(|&b| b == b'|').unwrap();
    let mut factors = [1usize; 256];
    input
        .split(|&b| b == b'\n')
        .enumerate()
        .map(|(i, game)| {
            let factor = factors[i];
            let win_seq = &game[col + 1..sep];
            let win_count = game[sep + 1..]
                .chunks_exact(3)
                .map(|n| &n[1..])
                .filter(|n| win_seq.chunks_exact(3).map(|n| &n[1..]).any(|c| &c == n))
                .count();
            (i..i + win_count).for_each(|i| factors[i + 1] += factor);
            factor * win_count + 1
        })
        .sum::<usize>() as u64
}

pub fn based_on_p1() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();
    let mut nr_of_copies = vec![0; input_string.lines().count() + 20];
    let mut sum = 0;

    for (line_idx, line) in input_string.lines().enumerate() {
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

            let nr_of_self = nr_of_copies[line_idx] + 1;
            sum += nr_of_self as u32;
            for i in (line_idx+1)..=(line_idx+matching) {
                nr_of_copies[i] += nr_of_self;
            }

        }

    }
    sum as u64
}


pub fn with_hashmap() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();
    let mut nr_of_copies: HashMap<usize, u32> = HashMap::new();
    let mut sum = 0;

    for (line_idx, line) in input_string.lines().enumerate() {
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

            //let nr_of_self = nr_of_copies.get(&line_idx).unwrap_or(&0) + 1;
            let nr_of_self = *nr_of_copies.entry(line_idx).or_default() + 1;
            sum += nr_of_self as u32;
            for i in (line_idx+1)..=(line_idx+matching) {
                *nr_of_copies.entry(i).or_default() += nr_of_self;
            }

        }

    }
    sum as u64
}
