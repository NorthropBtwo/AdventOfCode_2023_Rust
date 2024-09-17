use std::{fs, thread::{Builder, JoinHandle}};

use cgmath::Vector2;

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day16/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 16,
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
    7943
}

#[derive(Default, Clone)]
struct BeamCell {
    up : bool,
    down : bool,
    right : bool,
    left : bool,
}

struct Configuration {
    start : Vector2<i32>,
    dir : Vector2<i32>,
}

impl std::fmt::Debug for BeamCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", (self.up as u8 + self.down as u8 + self.left as u8 + self.right as u8))
    }
}

impl BeamCell {
    pub fn energize(&mut self, dir: Vector2<i32>) -> bool {
        let mut already_energized = false;
        match dir {
            Vector2{x: 1, y: 0} => {
                already_energized = self.right;
                self.right = true;
            }
            Vector2{x: -1, y: 0} => {
                already_energized = self.left;
                self.left = true;
            }
            Vector2{x: 0, y: 1} => {
                already_energized = self.down;
                self.down = true;
            }
            Vector2{x: 0, y: -1} => {
                already_energized = self.up;
                self.up = true;
            }
            _ => {}
        }

        already_energized
    }
}

pub fn first_try() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();

    let layout = input_string.lines().map(|x| x.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();

    let beam_field = vec![vec![BeamCell::default(); layout[0].len()] ; layout.len()];

    let configs = get_all_conficurations(Vector2::new(layout[0].len() as i32, layout.len() as i32));
    let mut handles = vec![];

    for config in configs {
        let layout = layout.clone();
        let mut beam_field = beam_field.clone();
        let handle = calculate_with_bigger_stack_size(move || {
            calculate_beam(config.start, config.dir, &layout, &mut beam_field);
            beam_field.into_iter().flatten().filter(|cell| cell.right || cell.left || cell.up || cell.down).count() as u64
        }).unwrap();

        handles.push(handle);

    }

    let mut max_sum = 0;

    for handler in handles {
        let sum = handler.join().unwrap();
        max_sum = max_sum.max(sum);
    }


    max_sum
}



fn get_all_conficurations (grid_size : Vector2<i32>) -> Vec<Configuration> {

    let top_rows = (0..grid_size.x).map( |x| Configuration{start: Vector2::new(x, 0), dir: Vector2::unit_y()});
    let bottom_rows = (0..grid_size.x).map( |x| Configuration{start: Vector2::new(x, grid_size.y-1), dir: -Vector2::unit_y()});
    let left_column = (0..grid_size.y).map( |y| Configuration{start: Vector2::new(0, y), dir: Vector2::unit_x()});
    let right_column = (0..grid_size.y).map( |y| Configuration{start: Vector2::new(grid_size.x-1, y), dir: -Vector2::unit_x()});

    top_rows.chain(bottom_rows).chain(left_column).chain(right_column).collect()
} 

fn calculate_with_bigger_stack_size<F, T>(f: F) -> std::io::Result<JoinHandle<T>> 
where
    F: FnOnce() -> T + Send + 'static, 
    T: Send + 'static, 
{
    let builder = Builder::new().stack_size(32 * 1024* 128);
    builder.spawn(f)
}

fn calculate_beam(pos : Vector2<i32>, dir : Vector2<i32>,layout : &Vec<Vec<u8>> , beam_field : &mut Vec<Vec<BeamCell>>) {


    if pos.x >= 0 && pos.y >= 0 &&  pos.y < layout.len() as i32 && pos.x < layout[pos.y as usize].len() as i32 {

        match layout[pos.y as usize][pos.x as usize] {
            b'.' => {
                if !beam_field[pos.y as usize][pos.x as usize].energize(dir) {
                    calculate_beam(pos + dir, dir, layout, beam_field);
                }
            }
            b'\\' => {
                let dir = Vector2::new(dir.y, dir.x);
                if !beam_field[pos.y as usize][pos.x as usize].energize(dir) {
                    calculate_beam(pos + dir, dir, layout, beam_field);
                }
            }
            b'/' => {
                let dir = Vector2::new(-dir.y, -dir.x);
                if !beam_field[pos.y as usize][pos.x as usize].energize(dir) {
                    calculate_beam(pos + dir, dir, layout, beam_field);
                }
            }
            b'|' => {
                if dir.x != 0 {
                    if !beam_field[pos.y as usize][pos.x as usize].energize(dir) {
                        for dir in vec![Vector2::unit_y(), -Vector2::unit_y()] {
                            calculate_beam(pos + dir, dir, layout, beam_field);
                        }
                    }
                } else {
                    if !beam_field[pos.y as usize][pos.x as usize].energize(dir) {
                        calculate_beam(pos + dir, dir, layout, beam_field);
                    }
                }
            }
            b'-' => {
                if dir.y != 0 {
                    if !beam_field[pos.y as usize][pos.x as usize].energize(dir) {
                        for dir in vec![Vector2::unit_x(), -Vector2::unit_x()] {
                            calculate_beam(pos + dir, dir, layout, beam_field);
                        }
                    }
                } else {
                    if !beam_field[pos.y as usize][pos.x as usize].energize(dir) {
                        calculate_beam(pos + dir, dir, layout, beam_field);
                    }
                }
            }
            _ => {}
        }

    }

}