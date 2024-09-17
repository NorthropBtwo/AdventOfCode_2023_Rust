use std::{fs, io::{self, BufRead}};

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day1/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 1,
        part_nr: 1,
        day_funcs: get_function_list(),
        solution: solution(),
    }
}

pub fn get_function_list() -> Vec<DayFunc> {
    vec![
        DayFunc{name: "First Try", func : first_try},
        DayFunc{name: "read_by_line", func : read_by_line},
        DayFunc{name: "better_char_search", func : better_char_search},
        DayFunc{name: "better_char_conversion", func : better_char_conversion},
        DayFunc{name: "better_alloc", func : better_alloc},
        DayFunc{name: "more_iterator", func : more_iterator},
        DayFunc{name: "more_direct_access", func : more_direct_access},
        DayFunc{name: "experimental", func : experimental},
    ]
}

pub fn solution() -> u64 {
    55712
}

pub fn first_try() -> u64 {
    let contents = fs::read_to_string(INPUT_PATH);
    match contents {
        Ok(contents) => {
            let mut sum = 0;
            for line in contents.lines() {
                let mut first = None;
                let mut last = None;
                for c in line.chars() {
                    if c.is_ascii_digit() {
                        if first == None {
                            first = Some(c)
                        }
                        last = Some(c);
                    }
                }
                sum += first.unwrap_or('0').to_digit(10).unwrap() * 10 + last.unwrap_or('0').to_digit(10).unwrap();
            }
            return sum as u64;
        },
        Err(error) => println!("{}", error),
    }
    0
}

pub fn read_by_line() -> u64 {
    let mut sum = 0;

    let file = fs::File::open(INPUT_PATH).unwrap();
    let reader = io::BufReader::new(file);
    for line in reader.lines().flatten() {
        let mut first = None;
        let mut last = None;
        for c in line.chars() {
            if c.is_ascii_digit() {
                if first == None {
                    first = Some(c)
                }
                last = Some(c);
            }
        }
        sum += first.unwrap_or('0').to_digit(10).unwrap() * 10 + last.unwrap_or('0').to_digit(10).unwrap();
    }
    sum as u64
}

pub fn better_char_search() -> u64 {
    let contents = fs::read_to_string(INPUT_PATH);
    match contents {
        Ok(contents) => {
            let mut sum = 0;
            for line in contents.lines() {
                let mut first = None;
                let mut last = None;
                for c in line.chars() {
                    if c.is_ascii_digit() {
                        first = Some(c);
                        break;
                    }
                }
                for c in line.chars().rev() {
                    if c.is_ascii_digit() {
                        last = Some(c);
                        break;
                    }
                }
                sum += first.unwrap_or('0').to_digit(10).unwrap() * 10 + last.unwrap_or('0').to_digit(10).unwrap();
            }
            return sum as u64;
        },
        Err(error) => println!("{}", error),
    }
    0
}

pub fn better_char_conversion() -> u64 {
    let contents = fs::read_to_string(INPUT_PATH);
    match contents {
        Ok(contents) => {
            let mut sum = 0;
            for line in contents.lines() {
                let mut line_val = 0;
                for c in line.chars() {
                    match c.to_digit(10) {
                        Some(first) => {
                            line_val = first * 10;
                            break;
                        },
                        None => {},
                    }
                }
                for c in line.chars().rev() {
                    match c.to_digit(10) {
                        Some(last) => {
                            line_val += last;
                            break;
                        },
                        None => {},
                    }
                }
                sum += line_val;
            }
            return sum as u64;
        },
        Err(error) => println!("{}", error),
    }
    0
}

pub fn better_alloc() -> u64 {
    let contents = fs::read_to_string(INPUT_PATH);
    match contents {
        Ok(contents) => {
            let mut sum = 0;
            let mut line_val = 0;
            for line in contents.lines() {
                for c in line.chars() {
                    match c.to_digit(10) {
                        Some(first) => {
                            line_val = first * 10;
                            break;
                        },
                        None => {},
                    }
                }
                for c in line.chars().rev() {
                    match c.to_digit(10) {
                        Some(last) => {
                            line_val += last;
                            break;
                        },
                        None => {},
                    }
                }
                sum += line_val;
            }
            return sum as u64;
        },
        Err(error) => println!("{}", error),
    }
    0
}

pub fn more_iterator() -> u64 {
    let contents = fs::read_to_string(INPUT_PATH);
    match contents {
        Ok(contents) => {
            let mut sum = 0;
            for line in contents.lines() {
                let first = line.chars().map(|x| x.to_digit(10)).flatten().next().unwrap_or(0);
                let last = line.chars().rev().map(|x| x.to_digit(10)).flatten().next().unwrap_or(0);
                sum += first*10 + last;
            }
            return sum as u64;
        },
        Err(error) => println!("{}", error),
    }
    0
}

pub fn more_direct_access() -> u64 {
    let contents = fs::read_to_string(INPUT_PATH);
    match contents {
        Ok(contents) => {
            let mut sum: u32 = 0;
            let mut line_val = 0;
            for line in contents.lines() {
                for i in 0..line.len() {
                    match line.as_bytes()[i] {
                        48..=57 => {
                            line_val = (line.as_bytes()[i] - 48) * 10;
                            break;
                        },
                        _ => {},
                    }
                }

                for i in (0..line.len()).rev() {
                    match line.as_bytes()[i] {
                        48..=57 => {
                            line_val += line.as_bytes()[i] - 48;
                            break;
                        },
                        _ => {},
                    }
                }

                sum += line_val as u32;
            }
            return sum as u64;
        },
        Err(error) => println!("{}", error),
    }
    0
}

const FILE2: &[u8; 22422] = include_bytes!("./day1/input.txt");
pub fn experimental() -> u64 {
    let mut sum: u32 = 0;
    for line in FILE2.split(|b| b == &b'\n') {
        let mut line_val = 0;
        for i in 0..line.len() {
            match line[i] {
                48..=57 => {
                    line_val = (line[i] - 48) * 10;
                    break;
                },
                _ => {},
            }
        }

        for i in (0..line.len()).rev() {
            match line[i] {
                48..=57 => {
                    line_val += line[i] - 48;
                    break;
                },
                _ => {},
            }
        }
        sum += line_val as u32;
    }
    sum as u64
}