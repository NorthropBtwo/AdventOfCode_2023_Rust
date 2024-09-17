use std::{fs, u32};

use cgmath::{Vector2, Zero};

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day17/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 17,
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
    1294
}

#[derive(Clone)]
struct PathCell {
    up : [u32; 10],
    down : [u32; 10],
    right : [u32; 10],
    left : [u32; 10],
}

impl Default for PathCell {
    fn default() -> Self {
        Self { up: [u32::MAX; 10], down: [u32::MAX; 10], right: [u32::MAX; 10], left: [u32::MAX; 10] }
    }
}

impl PathCell {
    pub fn is_best_cost(&mut self, dir : Vector2<i32>, steps_in_same_dir : usize, cost: u32) -> bool {
        let dir_list = match dir {
            Vector2 {x: 1,y: 0 } => &mut self.right,
            Vector2 {x: -1,y: 0 } => &mut self.left,
            Vector2 {x: 0,y: 1 } => &mut self.down,
            Vector2 {x: 0,y: -1 } => &mut self.up,
            _ => return false,
        };

        if cost < dir_list[steps_in_same_dir] {
            dir_list[steps_in_same_dir] = cost;
            true
        } else {
            false
        }
    }

    pub fn min_costs(&self) -> u32 {
        *self.right.iter().skip(3).chain(self.left.iter().skip(3)).chain(self.up.iter().skip(3)).chain(self.down.iter().skip(3)).min().unwrap()
    }
}


#[derive(Clone)]
struct State {
    pos : Vector2<i32>,
    dir : Vector2<i32>,
    steps_in_same_dir : usize,
    cost : u32,
}

pub fn first_try() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();


    let mut layout = input_string.lines().map(|x| x.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();

    let mut cells = vec![vec![PathCell::default(); layout[0].len()] ; layout.len()];

    let mut active_states = vec![
        State{dir: Vector2::unit_x(), steps_in_same_dir: 0, cost: 0, pos : Vector2::zero()},
        State{dir: Vector2::unit_y(), steps_in_same_dir: 0, cost: 0, pos : Vector2::zero()},
        ];

    let grid_size = Vector2::new(layout[0].len(), layout.len());

    for x in 0..grid_size.x {
        for y in 0..grid_size.y {
            layout[y][x] = layout[y][x] - b'0';
        }
    }

    while active_states.len() > 0 {

        let mut next_states = vec![];
        for state in active_states {

            let pos = state.pos + state.dir;
            if pos.x < 0 || pos.x >= grid_size.x as i32 || pos.y < 0 || pos.y >= grid_size.y as i32 {
                continue;
            }

            let cost = state.cost + layout[pos.y as usize][pos.x as usize] as u32;

            if cells[pos.y as usize][pos.x as usize].is_best_cost(state.dir, state.steps_in_same_dir, cost) {
                let steps_in_same_dir = state.steps_in_same_dir + 1;

                if steps_in_same_dir >= 4 {
                    let dir = Vector2::new(state.dir.y, state.dir.x);
                    next_states.push(State{pos, dir, cost, steps_in_same_dir: 0});
                    let dir = Vector2::new(-state.dir.y, -state.dir.x);
                    next_states.push(State{pos, dir, cost, steps_in_same_dir: 0});
                    let dir = -dir;
                    next_states.push(State{pos, dir, cost, steps_in_same_dir: 0});
                }
                if steps_in_same_dir < 10 {
                    let dir = state.dir;
                    next_states.push(State{pos, dir, cost, steps_in_same_dir});
                }
            }

        }

        active_states = next_states;

    }

    cells[grid_size.y-1][grid_size.x-1].min_costs().into()

}
