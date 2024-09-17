use std::fs;
use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day14/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 14,
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
    96061
}

enum TiltDir {
    North,
    West,
    South,
    East,
}


pub fn first_try() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();
    let mut sum = 0;

    

    let line_length = input_string.lines().next().unwrap().len();
    let input_string_line_iter = input_string.lines();
    let line_height = input_string_line_iter.count();

    let mut rock_field : Vec<Vec<bool>> = vec![vec![false; line_height+2]; line_length+2];
    let mut obstacle_field : Vec<Vec<bool>> = vec![vec![false; line_height+2]; line_length+2];
    
    for x in 0..obstacle_field.len() {
        obstacle_field[x][0] = true;
        obstacle_field[x][line_height+1] = true;
    }
    for y in 0..obstacle_field[0].len() {
        obstacle_field[0][y] = true;
        obstacle_field[line_length+1][y] = true;
    }

    for (y, line) in input_string.lines().map(|x| x.as_bytes()).enumerate() {
        for (x, c) in line.iter().enumerate() {
            match *c {
                b'#' => obstacle_field[x+1][y+1] = true,
                b'O' => rock_field[x+1][y+1] = true,
                _ =>  (),
            }
        }
    }
    
    let mut past_cycles : Vec<Vec<Vec<bool>>> = vec![];

    for cycle in 1..1000000000 {
        let mut rock_field_temp;
        rock_field_temp = tilt_platform(&rock_field, &obstacle_field, TiltDir::North);
        past_cycles.push(rock_field);
        rock_field_temp = tilt_platform(&rock_field_temp, &obstacle_field, TiltDir::West);
        rock_field_temp = tilt_platform(&rock_field_temp, &obstacle_field, TiltDir::South);
        rock_field_temp = tilt_platform(&rock_field_temp, &obstacle_field, TiltDir::East);
        rock_field = rock_field_temp;

        if let Some(position) = past_cycles.iter().position(|p| p == &rock_field) {
            let repeats_after = cycle - position;
            let requested_index = (1000000000 - position) % repeats_after + position;
            sum = calculate_support(&past_cycles[requested_index]);
            break;
        }
    }

    sum as u64
}

fn tilt_platform(rocks: &Vec<Vec<bool>>, obstacles: &Vec<Vec<bool>>, tilt_dir: TiltDir) -> Vec<Vec<bool>> {

    let x_max = rocks.len();
    let y_max = rocks[0].len();
    let mut rocks2 : Vec<Vec<bool>> = vec![vec![false; y_max]; x_max];

    match tilt_dir {
        TiltDir::North => {
            for x in 0..x_max {
                let mut group_size = 0;
                for y in (0..y_max).rev() {
                    if rocks[x][y] {
                        group_size += 1;
                    } else if obstacles[x][y] {
                        for y in (y+1)..(y+1+group_size) {
                            rocks2[x][y] = true;
                        }
                        group_size = 0;
                    }
                }
            }
        },
        TiltDir::West => {
            for y in 0..y_max {
                let mut group_size = 0;
                for x in (0..x_max).rev() {
                    if rocks[x][y] {
                        group_size += 1;
                    } else if obstacles[x][y] {
                        for x in (x+1)..(x+1+group_size) {
                            rocks2[x][y] = true;
                        }
                        group_size = 0;
                    }
                }
            }
        },
        TiltDir::South => {
            for x in 0..x_max {
                let mut group_size = 0;
                for y in 0..y_max {
                    if rocks[x][y] {
                        group_size += 1;
                    } else if obstacles[x][y] {
                        for y in (y-group_size)..(y) {
                            rocks2[x][y] = true;
                        }
                        group_size = 0;
                    }
                }
            }
        },
        TiltDir::East => {
            for y in 0..y_max {
                let mut group_size = 0;
                for x in (0..x_max) {
                    if rocks[x][y] {
                        group_size += 1;
                    } else if obstacles[x][y] {
                        for x in (x-group_size)..(x) {
                            rocks2[x][y] = true;
                        }
                        group_size = 0;
                    }
                }
            }
        },
    }
    
    rocks2
}

fn calculate_support(rocks_grid: &Vec<Vec<bool>>) -> usize {
    let mut sum = 0;
    for rocks_row in rocks_grid {
        for (weight ,rock) in rocks_row.iter().rev().enumerate() {
            if *rock {
                sum += weight
            }
        }
    }

    sum
}


fn print_field(rocks: &Vec<Vec<bool>>, obstacles: &Vec<Vec<bool>>) {

    let x_max = rocks.len();
    let y_max = rocks[0].len();

    for y in 0..y_max{
        for x in 0..x_max {
            if rocks[x][y] {
                print!("O");
            } else if obstacles[x][y] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    
}