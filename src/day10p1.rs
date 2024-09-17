use std::fs;
use cgmath::{self, Vector2};

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day10/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 10,
        part_nr: 1,
        day_funcs: get_function_list(),
        solution: solution(),
    }
}

pub fn get_function_list() -> Vec<DayFunc> {
    vec![
        DayFunc{name: "first_try", func : first_try},
        DayFunc{name: "timvisee(not mine)", func : timvisee},
    ]
}

pub fn solution() -> u64 {
    6890
}

pub fn get_dirs_from_map(map: &Vec<&str>,p: Vector2<isize>) -> [Vector2<isize>; 2] {
    get_dirs_from_pipe(map[p.y as usize].as_bytes()[p.x as usize])
}

pub fn get_dirs_from_pipe(pipe: u8) -> [Vector2<isize>; 2] {
    match pipe {
        b'|' => [Vector2::new(0, 1), Vector2::new(0, -1)],
        b'-' => [Vector2::new(1, 0), Vector2::new(-1, 0)],
        b'L' => [Vector2::new(0, -1), Vector2::new(1, 0)],
        b'J' => [Vector2::new(0, -1), Vector2::new(-1, 0)],
        b'7' => [Vector2::new(0, 1), Vector2::new(-1, 0)],
        b'F' => [Vector2::new(0, 1), Vector2::new(1, 0)],

        _ => [Vector2::new(0, 0), Vector2::new(0, 0)],
    }
}


pub fn first_try() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();

    let mut steps = 1;
    let map = input_string.lines().filter(|x| x.len() > 0).collect::<Vec<&str>>();

    let start_position = map.iter().enumerate().find_map(|(y, line)| 
        match line.find('S') {
            Some(x) => Some(Vector2::new(x as isize, y as isize)),
            None => None,
        }  
    ).unwrap();

    let possible_next_positions = [start_position + Vector2::new(1, 0), start_position + Vector2::new(-1, 0), start_position + Vector2::new(0, 1), start_position + Vector2::new(0, -1)];
    let possible_next_positions = possible_next_positions.into_iter().filter(|v| (v.x >= 0 && v.y >= 0 && v.y < map.len() as isize && v.x < map[v.y as usize].as_bytes().len() as isize)).collect::<Vec<Vector2<isize>>>();
    let mut possible_next_positions = possible_next_positions.into_iter().filter(|v| (get_dirs_from_map(&map, *v).contains(&(start_position - *v)))).collect::<Vec<Vector2<isize>>>();
    let mut current_position = [start_position, start_position];

    while possible_next_positions[0] != possible_next_positions[1] {
        for i in 0..2 {
            let mut dirs = get_dirs_from_map(&map, possible_next_positions[i]).to_vec();
            let in_dir = current_position[i] - possible_next_positions[i];
            dirs.retain(|&x| x != in_dir);
            current_position[i] = possible_next_positions[i];
            possible_next_positions[i] = *dirs.first().unwrap() + possible_next_positions[i];
        }
        steps += 1;
    }
    steps
}


pub fn timvisee() -> u64 {
    let input_str = fs::read_to_string(INPUT_PATH).unwrap();
    let map = input_str.as_bytes();
    let width = map.iter().position(|&b| b == b'\n').unwrap();
    let start = map.iter().position(|&b| b == b'S').unwrap();

    let (mut pos, mut dir) = {
        if matches!(map[start - width - 1], b'|' | b'7' | b'F') {
            (start - width - 1, Dir::Up)
        } else if matches!(map[start + width + 1], b'|' | b'L' | b'J') {
            (start + width + 1, Dir::Down)
        } else {
            (start - 1, Dir::Left)
        }
    };

    ((1 + std::iter::repeat(())
        .position(|_| unsafe {
            match (map.get_unchecked(pos), dir) {
                (b'|', Dir::Down) => pos += width + 1,
                (b'|', Dir::Up) => pos -= width + 1,
                (b'-', Dir::Left) => pos -= 1,
                (b'-', Dir::Right) => pos += 1,
                (b'L', Dir::Down) | (b'F', Dir::Up) => {
                    pos += 1;
                    dir = Dir::Right;
                }
                (b'L', Dir::Left) | (b'J', Dir::Right) => {
                    pos -= width + 1;
                    dir = Dir::Up;
                }
                (b'7', Dir::Up) | (b'J', Dir::Down) => {
                    pos -= 1;
                    dir = Dir::Left;
                }
                (b'7', Dir::Right) | (b'F', Dir::Left) => {
                    pos += width + 1;
                    dir = Dir::Down;
                }
                (b'S', _) => return true,
                (_, _) => unreachable!(),
            }
            false
        })
        .unwrap())
        / 2) as u64
}

#[derive(Debug, Clone, Copy)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}