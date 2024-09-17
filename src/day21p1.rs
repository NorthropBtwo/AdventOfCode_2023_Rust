use core::str;
use std::{collections::{HashMap, VecDeque}, fs, ops::Index, u32};

use cgmath::{Vector2, Zero};

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day21/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 21,
        part_nr: 1,
        day_funcs: get_function_list(),
        solution: solution(),
    }
}

pub fn get_function_list() -> Vec<DayFunc> {
    vec![
        DayFunc{name: "first_try", func : first_try},
        DayFunc{name: "no_back_step", func : no_back_step},
    ]
}

pub fn solution() -> u64 {
    3740
}


pub fn first_try() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();

    let layout = input_string.lines().map(|x| x.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
    let y_max = layout.len() as isize;
    let x_max = layout[0].len() as isize;
    let mut positions = vec![find_start(&layout)];

    for _ in 0..64 {
        let mut new_positions = vec![];
        for position in positions {
            
            let next_steps = [position + Vector2::unit_x(), position - Vector2::unit_x(), position + Vector2::unit_y(), position - Vector2::unit_y()];
            for next_step in next_steps {
                if next_step.x >= 0 && next_step.x < x_max && next_step.y >= 0 && next_step.y < y_max {
                    let tile = layout[next_step.y as usize][next_step.x as usize];
                    if (tile == b'.' || tile == b'S') && !new_positions.contains(&next_step) {
                        new_positions.push(next_step)
                    }
                }
            }

        }

        positions = new_positions;
    }


    positions.len() as u64
}

fn find_start(layout : &Vec<Vec<u8>>)-> Vector2<isize> {
    for y in 1..layout.len() {
        for x in 1..layout[0].len() {
            if layout[y][x] == b'S' {
                return Vector2::new(x as isize, y as isize);
            }
        }
    }

    panic!("start not found");
}

pub fn no_back_step() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();

    let layout = input_string.lines().map(|x| x.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
    let y_max = layout.len() as isize;
    let x_max = layout[0].len() as isize;
    let mut positions = vec![find_start(&layout)];

    let mut all_uneven_positions = vec![];
    let mut all_even_positions = vec![positions[0]];

    for _ in 0..(64/2) {
        let mut new_positions = vec![];
        for position in positions {
            
            let next_steps = [position + Vector2::unit_x(), position - Vector2::unit_x(), position + Vector2::unit_y(), position - Vector2::unit_y()];
            for next_step in next_steps {
                if next_step.x >= 0 && next_step.x < x_max && next_step.y >= 0 && next_step.y < y_max {
                    let tile = layout[next_step.y as usize][next_step.x as usize];
                    if (tile == b'.' || tile == b'S') && !new_positions.contains(&next_step) && !all_uneven_positions.contains(&next_step) {
                        new_positions.push(next_step);
                        all_uneven_positions.push(next_step);
                    }
                }
            }

        }
        positions = new_positions;


        let mut new_positions = vec![];
        for position in positions {
            
            let next_steps = [position + Vector2::unit_x(), position - Vector2::unit_x(), position + Vector2::unit_y(), position - Vector2::unit_y()];
            for next_step in next_steps {
                if next_step.x >= 0 && next_step.x < x_max && next_step.y >= 0 && next_step.y < y_max {
                    let tile = layout[next_step.y as usize][next_step.x as usize];
                    if (tile == b'.' || tile == b'S') && !new_positions.contains(&next_step) && !all_even_positions.contains(&next_step) {
                        new_positions.push(next_step);
                        all_even_positions.push(next_step);
                    }
                }
            }

        }
        positions = new_positions;       
    }

    all_even_positions.len() as u64
}
