use std::{fs, u32};

use cgmath::{Vector2, Zero};

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day18/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 18,
        part_nr: 1,
        day_funcs: get_function_list(),
        solution: solution(),
    }
}

pub fn get_function_list() -> Vec<DayFunc> {
    vec![
        //DayFunc{name: "first_try", func : first_try}, //this code is to slow
        DayFunc{name: "list_to_2D_conversion", func : list_to_2_d_conversion},
    ]
}

pub fn solution() -> u64 {
    52035
}


pub struct Instruction {
    direction : String,
    length : u32,
}

pub struct Area {
    min : Vector2<i32>,
    max : Vector2<i32>,
}

impl Area {
    pub fn increase_area(&mut self, new_point: Vector2<i32>) {
        self.min.x = self.min.x.min(new_point.x);
        self.min.y = self.min.y.min(new_point.y);
        self.max.x = self.max.x.max(new_point.x);
        self.max.y = self.max.y.max(new_point.y);
    }
}

impl Default for Area {
    fn default() -> Self {
        Self { min: Vector2::zero(), max: Vector2::zero() }
    }
}

#[allow(dead_code)]
pub fn first_try() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();


    let instructions = input_string.lines().map(|x| {let mut parts = x.split(' '); Instruction{direction: parts.next().unwrap_or("").to_string(),length: parts.next().unwrap_or("0").parse().unwrap_or(0)}}).collect::<Vec<Instruction>>();

    let mut holes = vec![];
    let mut dig_pos = Vector2::zero();
    let mut area = Area::default();


    for instruction in instructions {
        let dir =  match instruction.direction.chars().next().unwrap() {
            'R' => {
                Vector2::unit_x()
            }
            'L' => {
                -Vector2::unit_x()
            }
            'U' => {
                -Vector2::unit_y()
            }
            'D' => {
                Vector2::unit_y()
            }
            _ => Vector2::zero()
        };

        for _ in 0..instruction.length {
            dig_pos += dir;
            holes.push(dig_pos);
            area.increase_area(dig_pos)
        }

    }


    let mut flood_fill_points = vec![];
    //find start of floodFill
    for x in area.min.x..=area.max.x {
        let y = area.min.y;
        if holes.contains(&Vector2{x,y}) && !holes.contains(&Vector2{x, y: y+1}) {
            flood_fill_points.push(Vector2{x,y:y+1});
            holes.push(Vector2{x,y:y+1});
        }
    }

    while flood_fill_points.len() > 0 {
        let point = flood_fill_points.pop().unwrap();
        let new_points = [point + Vector2::unit_x(), point - Vector2::unit_x(),point + Vector2::unit_y(), point - Vector2::unit_y()];
        for new_point in new_points {
            if !holes.contains(&new_point) {
                flood_fill_points.push(new_point);
                holes.push(new_point);
            }
        }
    }


    holes.len() as u64

}


pub fn list_to_2_d_conversion() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();

    //input file to list of holes
    let instructions = input_string.lines().map(|x| {let mut parts = x.split(' '); Instruction{direction: parts.next().unwrap_or("").to_string(),length: parts.next().unwrap_or("0").parse().unwrap_or(0)}}).collect::<Vec<Instruction>>();

    let mut holes = vec![];
    let mut dig_pos = Vector2::zero();
    let mut area = Area::default();
    let mut lava_area_size = 0;


    for instruction in instructions {
        let dir =  match instruction.direction.chars().next().unwrap() {
            'R' => {
                Vector2::unit_x()
            }
            'L' => {
                -Vector2::unit_x()
            }
            'U' => {
                -Vector2::unit_y()
            }
            'D' => {
                Vector2::unit_y()
            }
            _ => Vector2::zero()
        };

        for _ in 0..instruction.length {
            dig_pos += dir;
            holes.push(dig_pos);
            area.increase_area(dig_pos)
        }

    }

    //convert list of holes to 2d map
    let mut field = vec![vec![false; (area.max.x - area.min.x + 1) as usize]; (area.max.y - area.min.y + 1) as usize];
    for hole in &holes {
        field[(hole.y - area.min.y) as usize][(hole.x - area.min.x) as usize] = true;
        lava_area_size += 1;
    }

    //flood fill algorithm
    let mut flood_fill_points = vec![];
    for x in 0..field[0].len() {
        let y = 0 as usize;
        if field[y][x] && !field[y+1][x] {
            flood_fill_points.push(Vector2{x,y:y + 1});
        }
    }

    while flood_fill_points.len() > 0 {
        let point = flood_fill_points.pop().unwrap();
        if !field[point.y as usize][point.x as usize] {
            field[point.y as usize][point.x as usize] = true;
            lava_area_size += 1;

            flood_fill_points.push(point + Vector2::unit_x());
            flood_fill_points.push(point - Vector2::unit_x());
            flood_fill_points.push(point + Vector2::unit_y());
            flood_fill_points.push(point - Vector2::unit_y());
        }

    }


    lava_area_size as u64
}