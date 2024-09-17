use std::fs;
use cgmath::{self, Vector2};

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day10/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 10,
        part_nr: 2,
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
    453
}

fn get_dirs_from_map(map: &Vec<String>,p: Vector2<isize>) -> [Vector2<isize>; 2] {
    get_dirs_from_pipe(map[p.y as usize].as_bytes()[p.x as usize])
}

fn get_dirs_from_pipe(pipe: u8) -> [Vector2<isize>; 2] {
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

fn replace_start_with_correct_pipe(map: &mut Vec<String>, start_position: &Vector2<isize>, next_positions: &Vec<Vector2<isize>>) {
    let start_dirs = [next_positions[0] - start_position, next_positions[1] - start_position];

    let mut start_pipe = b'$';
    let possible_pipes = [b'|', b'-', b'L', b'J', b'7', b'F'];
    for pipe in possible_pipes {
        let pipe_dirs = get_dirs_from_pipe(pipe);
        if (pipe_dirs[0] == start_dirs[0] && pipe_dirs[1] == start_dirs[1]) || (pipe_dirs[0] == start_dirs[1] && pipe_dirs[1] == start_dirs[0]) {
            start_pipe = pipe;
            break;
        } 
    }

    map[start_position.y as usize].replace_range(start_position.x as usize..=start_position.x as usize, std::str::from_utf8(&[start_pipe]).unwrap());
}

#[derive(PartialEq)]
pub enum PipeDirection {
    NoPipe,
    FromBelow,
    FromAbove,
}

pub fn first_try() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();

    let mut pipe_in_loop = vec![];
    let mut map = input_string.lines().filter(|x| x.len() > 0).map(|x| x.to_owned()).collect::<Vec<String>>();

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

    //replace start with correct pipe:
    replace_start_with_correct_pipe(&mut map, &start_position, &possible_next_positions);

    while possible_next_positions[0] != possible_next_positions[1] {
        pipe_in_loop.push(possible_next_positions[0]);
        pipe_in_loop.push(possible_next_positions[1]);
        for i in 0..2 {
            let mut dirs = get_dirs_from_map(&map, possible_next_positions[i]).to_vec();
            let in_dir = current_position[i] - possible_next_positions[i];
            dirs.retain(|&x| x != in_dir);
            current_position[i] = possible_next_positions[i];
            possible_next_positions[i] = *dirs.first().unwrap() + possible_next_positions[i];
        }
    }
    pipe_in_loop.push(possible_next_positions[0]);
    pipe_in_loop.push(start_position);

    //remove all pipes which are not in the loop
    for (y, line) in map.iter_mut().enumerate() {

        for (x, c) in unsafe { line.as_bytes_mut().iter_mut().enumerate() } { /*unsafe because we must ensure the borrwoed strings stays a valid UTF8 string */
            if !pipe_in_loop.contains(&Vector2::new(x as isize, y as isize)) {
                *c = b'.';
            }
        }
    }


    //point-in-polygon (PIP) algorithm
    let mut inside_count = 0;
    for line in map {
        let mut inside = false;
        let mut cur_dir = PipeDirection::NoPipe;
        
        for pipe in line.chars() {
            if cur_dir == PipeDirection::NoPipe {
                match pipe {
                    'F' => cur_dir = PipeDirection::FromBelow,
                    'L' => cur_dir = PipeDirection::FromAbove,
                    '|' => inside = !inside,
                    '.' => if inside {inside_count += 1},
                    _ => (),
                }
            } else {
                match pipe {
                    '7' => {
                        inside ^= cur_dir == PipeDirection::FromAbove;
                        cur_dir = PipeDirection::NoPipe;
                    },
                    'J' => {
                        inside ^= cur_dir == PipeDirection::FromBelow;
                        cur_dir = PipeDirection::NoPipe;
                    },
                    '|' => inside = !inside,
                    _ => (),
                }
            }
        }

    }

    inside_count
}


pub fn timvisee() -> u64 {
    let input_str = fs::read_to_string(INPUT_PATH).unwrap();
    let map = input_str.as_bytes();
    let width = map.iter().position(|&b| b == b'\n').unwrap();
    let start = map.iter().position(|&b| b == b'S').unwrap();

    let mut pipes = vec![false; map.len()];
    let (mut pos, mut dir) = {
        if matches!(map[start - width - 1], b'|' | b'7' | b'F') {
            (start - width - 1, Dir::Up)
        } else if matches!(map[start + width + 1], b'|' | b'L' | b'J') {
            (start + width + 1, Dir::Down)
        } else {
            (start - 1, Dir::Left)
        }
    };

    std::iter::repeat(())
        .position(|_| unsafe {
            *pipes.get_unchecked_mut(pos) = true;
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
        .unwrap();

    let mut inside = false;
    map.iter()
        .enumerate()
        .filter(|(pos, tile)| {
            let is_pipe = unsafe { *pipes.get_unchecked(*pos) };
            inside &= pos % (width + 1) != 0;
            inside ^= is_pipe && matches!(*tile, b'|' | b'F' | b'7');
            inside && (!is_pipe || **tile == b'.') && (pos % (width + 1) != width)
        })
        .count() as u64
}

#[derive(Debug, Clone, Copy)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}