use std::fs;

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day3/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 3,
        part_nr: 1,
        day_funcs: get_function_list(),
        solution: solution(),
    }
}

pub fn get_function_list() -> Vec<DayFunc> {
    vec![
        DayFunc{name: "First Try", func : first_try},
    ]
}

pub fn solution() -> u32 {
    539637
}

#[derive(Clone, Copy, PartialEq)]
enum Field {
    Dot,
    Digit(u8),
    Symbol,
}

impl Field {
    pub fn from_char(c : char) -> Field {
        match c {
            '.' => Field::Dot,
            '0' => Field::Digit(0),
            '1' => Field::Digit(1),
            '2' => Field::Digit(2),
            '3' => Field::Digit(3),
            '4' => Field::Digit(4),
            '5' => Field::Digit(5),
            '6' => Field::Digit(6),
            '7' => Field::Digit(7),
            '8' => Field::Digit(8),
            '9' => Field::Digit(9),
            _  => Field::Symbol,
        }
    }
}

struct Schematic {
    lines : Vec<Vec<Field>>,
}

impl Schematic {
    pub fn new(lines : &str) -> Schematic {
        let mut field_lines = vec![];
        for line in lines.lines() {
            let mut field_line = vec![];
            for c in line.chars() {
                field_line.push(Field::from_char(c))
            }
            field_lines.push(field_line);
            
        }
        Schematic{lines: field_lines}
    }

    pub fn get_field(&self, x: usize, y: usize) -> Field {
        match self.lines.get(y) {
            Some(line) => {
                match line.get(x) {
                    Some(field) => *field,
                    None =>  Field::Dot,
                }
            },
            None => Field::Dot,
        }
    }

    fn is_symbol_adjascent(&self,x: usize, y: usize) -> bool {
        self.get_field(x.wrapping_add(1), y) == Field::Symbol || 
        self.get_field(x.wrapping_sub(1), y) == Field::Symbol || 
        self.get_field(x, y.wrapping_add(1)) == Field::Symbol || 
        self.get_field(x, y.wrapping_sub(1)) == Field::Symbol || 
        self.get_field(x.wrapping_add(1), y.wrapping_add(1)) == Field::Symbol || 
        self.get_field(x.wrapping_sub(1), y.wrapping_add(1)) == Field::Symbol || 
        self.get_field(x.wrapping_add(1), y.wrapping_sub(1)) == Field::Symbol || 
        self.get_field(x.wrapping_sub(1), y.wrapping_sub(1)) == Field::Symbol
    }

    pub fn calculate_sum_near_symbol(&self) -> u32 {

        let mut symbol_found = false;
        let mut part_nr = 0;
        let mut part_sum = 0;

        for y in 0..self.lines.len() {
            let line = &self.lines[y];
            for x in 0..=line.len() {
                let cur_field = self.get_field(x, y);
                match cur_field {
                    Field::Digit(digit) => {
                        part_nr = part_nr * 10 + digit as u32;
                        if !symbol_found {
                            symbol_found = self.is_symbol_adjascent(x, y);
                        }
                    },
                    _ => {
                        if symbol_found {
                            part_sum += part_nr;
                        }
                        part_nr = 0;
                        symbol_found = false;
                    }
                }
            }
        }

        part_sum
    }

}

pub fn first_try() -> u32 {
    let content = fs::read_to_string(INPUT_PATH);
    match content {
        Ok(content) => {
            let schematic = Schematic::new(&content);
            return schematic.calculate_sum_near_symbol();
        },
        Err(error) => println!("{}", error),
    }
    0
}
