use std::{fs, collections::HashMap, usize};

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day3/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 3,
        part_nr: 2,
        day_funcs: get_function_list(),
        solution: solution(),
    }
}

pub fn get_function_list() -> Vec<DayFunc> {
    vec![
        DayFunc{name: "expanded_part_1", func : expanded_part_1},
        DayFunc{name: "timvisee(not mine)", func : timvisee},
        DayFunc{name: "go_for_stars_first", func : go_for_stars_first},
    ]
}

pub fn solution() -> u32 {
    82818007
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct XY<T> {
    pub x: T,
    pub y: T,
}

#[derive(Clone, Copy, PartialEq)]
enum Field {
    Dot,
    Digit(u8),
    Symbol(char),
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
            _  => Field::Symbol(c),
        }
    }

    pub fn is_symbol(&self) -> bool {
        match self {
            Field::Symbol(_) => true,
            _ => false,
        }
    }

    pub fn is_digit(&self) -> bool {
        match self {
            Field::Digit(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
struct Star {
    pub adjacent_parts : Vec<u32>,
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
        self.get_field(x.wrapping_add(1), y).is_symbol() || 
        self.get_field(x.wrapping_sub(1), y).is_symbol() || 
        self.get_field(x, y.wrapping_add(1)).is_symbol() || 
        self.get_field(x, y.wrapping_sub(1)).is_symbol() || 
        self.get_field(x.wrapping_add(1), y.wrapping_add(1)).is_symbol() || 
        self.get_field(x.wrapping_sub(1), y.wrapping_add(1)).is_symbol() || 
        self.get_field(x.wrapping_add(1), y.wrapping_sub(1)).is_symbol() || 
        self.get_field(x.wrapping_sub(1), y.wrapping_sub(1)).is_symbol()
    }

    fn find_adjacent_star_positions(&self,x: usize, y: usize, length: usize) -> Vec<XY<usize>> {
        let mut star_positions = vec![];

        for y in y.saturating_sub(1)..=y.saturating_add(1) {
            for x in x.saturating_sub(length-1)..=x {
                if self.get_field(x, y) == Field::Symbol('*') {
                    star_positions.push(XY{x,y})
                }
            }
        }

        star_positions
    }

    pub fn calculate_gear_ratio_sum(&self) -> u32 {

        let mut symbol_found = false;
        let mut part_len = 0;
        let mut part_nr = 0;
        let mut gear_sum = 0;
        let mut star_map :HashMap<XY<usize>, Star> = HashMap::new();

        for y in 0..self.lines.len() {
            let line = &self.lines[y];
            for x in 0..=line.len() {
                let cur_field = self.get_field(x, y);
                match cur_field {
                    Field::Digit(digit) => {
                        part_len += 1;
                        part_nr = part_nr * 10 + digit as u32;
                        if !symbol_found {
                            symbol_found = self.is_symbol_adjascent(x, y);
                        }
                    },
                    _ => {
                        if symbol_found {
                            let star_positions = self.find_adjacent_star_positions(x, y, part_len + 2);
                            for star_position in star_positions {
                                match star_map.get_mut(&star_position) {
                                    Some(star) => {
                                        star.adjacent_parts.push(part_nr);
                                    },
                                    None => {
                                        star_map.insert(star_position, Star{adjacent_parts: vec![part_nr]});
                                    },
                                }
                            }
                            
                        }
                        part_len = 0;
                        part_nr = 0;
                        symbol_found = false;
                    }
                }
            }
        }

        for star in star_map.into_values() {
            if star.adjacent_parts.len() == 2 {
                gear_sum += star.adjacent_parts[0] * star.adjacent_parts[1];
            }
        }

        gear_sum
    }

}

pub fn expanded_part_1() -> u32 {
    let content = fs::read_to_string(INPUT_PATH);
    match content {
        Ok(content) => {
            let schematic = Schematic::new(&content);
            return schematic.calculate_gear_ratio_sum();
        },
        Err(error) => println!("{}", error),
    }
    0
}


fn timvisee() -> u32 {
    let inputs_string = fs::read_to_string(INPUT_PATH).unwrap();
    let map = inputs_string.as_bytes();
    let width = map.iter().position(|b| b == &b'\n').unwrap() as isize;
    let mut nums = vec![];
    (0..map.len() - 2)
        .filter(|i| map[*i] == b'*')
        .filter_map(|i| {
            nums.clear();
            nums.extend(
                (-width - 2..=-width)
                    .chain([-1, 1])
                    .chain(width..=width + 2)
                    .map(|pos| (i as isize + pos) as usize)
                    .filter(|pos| map[*pos].is_ascii_digit())
                    .flat_map(|pos| {
                        (pos.saturating_sub(2)..=pos)
                            .rev()
                            .take_while(|i| map[*i].is_ascii_digit())
                            .last()
                    }),
            );
            nums.dedup();
            (nums.len() == 2).then(|| {
                nums.iter()
                    .map(|i| atoi::atoi::<usize>(&map[*i..*i + 3]).unwrap())
                    .product::<usize>()
            })
        })
        .sum::<usize>() as u32
    
}



pub fn go_for_stars_first() -> u32 {
    let content: Result<String, std::io::Error> = fs::read_to_string(INPUT_PATH);
    match content {
        Ok(content) => {
            let schematic = Schematic::new(&content);
            return schematic.calculate_gear_ratio_sum2();
        },
        Err(error) => println!("{}", error),
    }
    0
}


impl Schematic {

    pub fn calculate_gear_ratio_sum2(&self) -> u32 {
        let mut gear_sum = 0;
        for y in 0..self.lines.len() {
            let line = &self.lines[y];
            for x in 0..=line.len() {
                let cur_field = self.get_field(x, y);

                if cur_field == Field::Symbol('*') {

                    let bool_array = [
                        self.get_field(x.wrapping_sub(1), y).is_digit(), /* left */
                        self.get_field(x.wrapping_add(1), y).is_digit(), /* right */
                        self.get_field(x, y.wrapping_add(1)).is_digit() || self.get_field(x.wrapping_add(1), y.wrapping_add(1)).is_digit(), /* down */
                        !self.get_field(x, y.wrapping_add(1)).is_digit() && self.get_field(x.wrapping_sub(1), y.wrapping_add(1)).is_digit(), /* down */
                        self.get_field(x, y.wrapping_sub(1)).is_digit() || self.get_field(x.wrapping_add(1), y.wrapping_sub(1)).is_digit(), /* up */
                        !self.get_field(x, y.wrapping_sub(1)).is_digit() && self.get_field(x.wrapping_sub(1), y.wrapping_sub(1)).is_digit(), /* up */
                    ];
                    let nr_of_adjacent_parts = bool_array.into_iter().filter(|x| *x).count();
                    if nr_of_adjacent_parts == 2 { /* its a gear */
                        
                        /* right */
                        let mut digits = vec![];
                        let mut part_nrs = vec![];
                        for i in 1..usize::MAX {
                            match self.get_field(x.wrapping_add(i),y) {
                                Field::Digit(digit) => {
                                    digits.push(digit as u32);
                                },
                                _ => {
                                    part_nrs.push(digits);
                                    break;
                                }
                            }
                        }
                        /* left */
                        let mut digits = vec![];
                        for i in 1..usize::MAX {
                            match self.get_field(x.wrapping_sub(i),y) {
                                Field::Digit(digit) => {
                                    digits.insert(0, digit as u32)
                                },
                                _ => {
                                    part_nrs.push(digits);
                                    break;
                                }
                            }
                        }
                        /* down */
                        let mut digits = vec![];
                        if self.get_field(x, y.wrapping_add(1)).is_digit() {
                            /* only 1 digit below but in 2 direction possible*/
                            for i in 0..usize::MAX {
                                match self.get_field(x.wrapping_add(i),y.wrapping_add(1)) {
                                    Field::Digit(digit) => {
                                        digits.push(digit as u32);
                                    },
                                    _ => { 
                                        break;
                                    }
                                }
                            }
                            for i in 1..usize::MAX {
                                match self.get_field(x.wrapping_sub(i),y.wrapping_add(1)) {
                                    Field::Digit(digit) => {
                                        digits.insert(0,digit as u32);
                                    },
                                    _ => { 
                                        part_nrs.push(digits);
                                        break;
                                    }
                                }
                            }
                        } else {
                            /* only 2 digits possible*/
                            let mut digits = vec![];
                            for i in 1..usize::MAX {
                                match self.get_field(x.wrapping_add(i),y.wrapping_add(1)) {
                                    Field::Digit(digit) => {
                                        digits.push(digit as u32);
                                    },
                                    _ => {
                                        part_nrs.push(digits);
                                        break;
                                    }
                                }
                            }
                            let mut digits = vec![];
                            for i in 1..usize::MAX {
                                match self.get_field(x.wrapping_sub(i),y.wrapping_add(1)) {
                                    Field::Digit(digit) => {
                                        digits.insert(0, digit as u32)
                                    },
                                    _ => {
                                        part_nrs.push(digits);
                                        break;
                                    }
                                }
                            }
                        }
                        /* up */
                        let mut digits = vec![];
                        if self.get_field(x, y.wrapping_sub(1)).is_digit() {
                            /* only 1 digit below but in 2 direction possible*/
                            for i in 0..usize::MAX {
                                match self.get_field(x.wrapping_add(i),y.wrapping_sub(1)) {
                                    Field::Digit(digit) => {
                                        digits.push(digit as u32);
                                    },
                                    _ => { 
                                        break;
                                    }
                                }
                            }
                            for i in 1..usize::MAX {
                                match self.get_field(x.wrapping_sub(i),y.wrapping_sub(1)) {
                                    Field::Digit(digit) => {
                                        digits.insert(0,digit as u32);
                                    },
                                    _ => { 
                                        part_nrs.push(digits);
                                        break;
                                    }
                                }
                            }
                        } else {
                            /* only 2 digits possible*/
                            let mut digits = vec![];
                            for i in 1..usize::MAX {
                                match self.get_field(x.wrapping_add(i),y.wrapping_sub(1)) {
                                    Field::Digit(digit) => {
                                        digits.push(digit as u32);
                                    },
                                    _ => {
                                        part_nrs.push(digits);
                                        break;
                                    }
                                }
                            }
                            let mut digits = vec![];
                            for i in 1..usize::MAX {
                                match self.get_field(x.wrapping_sub(i),y.wrapping_sub(1)) {
                                    Field::Digit(digit) => {
                                        digits.insert(0, digit as u32)
                                    },
                                    _ => {
                                        part_nrs.push(digits);
                                        break;
                                    }
                                }
                            }
                        }

                        let mut part_nr_values = vec![];
                        for part_nr_vec in part_nrs {
                            let mut nr = 0;
                            for digit in part_nr_vec {
                                nr = nr * 10 + digit;
                            }
                            if nr != 0 {
                                part_nr_values.push(nr);
                            }
                        }

                        gear_sum += part_nr_values.into_iter().product::<u32>();
                    }
                }
            }
        }

        gear_sum
    }

}