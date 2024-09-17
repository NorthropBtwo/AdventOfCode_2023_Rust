use core::str;
use std::fs;
use cgmath::Vector2;

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day24/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 24,
        part_nr: 1,
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
    21679
}


#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub position: Vector2<i64>,
    pub direction: Vector2<i64>,
}

impl Line {
    fn from_str(line_str: &str) -> Self {

        let str_parts = line_str.split('@').collect::<Vec<_>>();
        if str_parts.len() == 2 {
            let position = str_parts[0].split(',').collect::<Vec<_>>();
            let position = position.iter().map(|x| x.trim().parse::<i64>().unwrap()).collect::<Vec<_>>();
            let direction = str_parts[1].split(',').collect::<Vec<_>>();
            let direction = direction.iter().map(|x| x.trim().parse::<i64>().unwrap()).collect::<Vec<_>>();
            if position.len() == 3 && direction.len() == 3 {
                return Line{position: Vector2::new(position[0], position[1]), direction: Vector2::new(direction[0], direction[1])};
            }
        }

        panic!("str is not a line");
    }
}


pub fn first_try() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();
    let lines = input_string.lines().map(|x| Line::from_str(x)).collect::<Vec<_>>();

    let mut intersection_in_boundary = 0;

    for i in 0..lines.len() {
        for j in (i+1)..lines.len() {
            let point = ray_intersect(&lines[i], &lines[j]);
            if let Some(point) = point {
                if point.x >= 200000000000000. && point.y >= 200000000000000. && point.x <= 400000000000000. && point.y <= 400000000000000. {
                    intersection_in_boundary += 1;
                }
            }
        }
    }

    intersection_in_boundary
}

//ray intersection https://www.youtube.com/watch?v=c065KoXooSw&t=920s

fn ray_intersect(l1: &Line, l2: &Line) -> Option<Vector2<f32>> {
    let d = l1.direction.x * l2.direction.y - l1.direction.y * l2.direction.x;
    //lines are parallel -> no intersection
    if d == 0 {
        None
    } else {
        let u = ((l2.position.x - l1.position.x) * l1.direction.y - (l2.position.y - l1.position.y) * l1.direction.x) as f32 / d as f32;
        let t = ((l2.position.x - l1.position.x) * l2.direction.y - (l2.position.y - l1.position.y) * l2.direction.x) as f32 / d as f32;
        if u >= 0. && t >= 0. {
            Some(Vector2::new(l2.position.x as f32 + l2.direction.x as f32 * u, l2.position.y as f32 + l2.direction.y as f32 * u))
        } else {
            None
        }
    }
}

