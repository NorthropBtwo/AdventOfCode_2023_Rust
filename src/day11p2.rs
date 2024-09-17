use std::fs;
use cgmath::{self, Vector2};

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day11/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 11,
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
    363293506944
}


pub fn first_try() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();
    let mut total_path_length = 0;

    let lines = input_string.lines().collect::<Vec<&str>>();
    let mut y_has_galaxie =  vec![false; lines.len()];
    let mut x_has_galaxie = vec![false; lines[0].chars().count()];

    let mut galaxy_positions = vec![];

    for (y,line) in lines.into_iter().enumerate() {
        for (x,c) in line.chars().enumerate() {
            if c == '#' {
                y_has_galaxie[y] = true;
                x_has_galaxie[x] = true;
                galaxy_positions.push(Vector2{x, y})
            }
        }
    }

    for idx1 in 0..galaxy_positions.len() {
        for idx2 in (idx1+1)..galaxy_positions.len() {
            for x in get_range_iter_inclusive(galaxy_positions[idx1].x, galaxy_positions[idx2].x) {
                if x_has_galaxie[x] {
                    total_path_length += 1;
                } else {
                    total_path_length += 1000000;
                }
            }
            total_path_length -= 1; /* we counted 1 too much */
            for y in get_range_iter_inclusive(galaxy_positions[idx1].y, galaxy_positions[idx2].y) {
                if y_has_galaxie[y] {
                    total_path_length += 1;
                } else {
                    total_path_length += 1000000;
                }
            }
            total_path_length -= 1; /* we counted 1 too much */
        }
    }


    return total_path_length;
}



fn get_range_iter_inclusive(a: usize, b: usize) -> impl Iterator<Item = usize> {
    if b > a {
        let vec: Vec<usize> = (a..=b).collect();
        vec.into_iter()
    } else {
        let vec: Vec<usize> = (b..=a).rev().collect();
        vec.into_iter()
    }
}