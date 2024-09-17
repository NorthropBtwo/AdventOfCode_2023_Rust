use std::fs;

use cgmath::{Vector2, Zero};

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day18/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 18,
        part_nr: 2,
        day_funcs: get_function_list(),
        solution: solution(),
    }
}

pub fn get_function_list() -> Vec<DayFunc> {
    vec![
        DayFunc{name: "firts_try", func : firts_try},
        DayFunc{name: "ranjeethmahankali(not mine)", func : ranjeethmahankali},
    ]
}

pub fn solution() -> u64 {
    60612092439765
}


pub struct Instruction {
    direction : Vector2<i64>,
    length : u64,
}

impl Instruction {
    pub fn from_str(str: &str) -> Instruction {
        let hex_bytes = str.split(" ").skip(2).next().unwrap().as_bytes();
        let direction = match hex_bytes[7] {
            b'0' => Vector2::unit_x(),
            b'1' => Vector2::unit_y(),
            b'2' => -Vector2::unit_x(),
            b'3' => -Vector2::unit_y(),
            _ => Vector2::zero(),
        };
        let mut length = 0;
        for i in 2..=6  {
            let mut num = hex_bytes[i] - b'0';
            if num > 9 {
                num = hex_bytes[i] - b'a' + 10;
            }
            length = length * 16 + num as u64;
        }
        Instruction {direction, length}
    }
    /*pub fn from_str(str: &str) -> Instruction {
        let mut parts = str.split(" ");

        let direction = match parts.next().unwrap() {
            "R" => Vector2::unit_x(),
            "D" => Vector2::unit_y(),
            "L" => -Vector2::unit_x(),
            "U" => -Vector2::unit_y(),
            _ => Vector2::zero(),
        };
        let length = parts.next().unwrap_or("0").parse().unwrap_or(0);
        Instruction {direction, length}
    }*/
}

pub struct Area {
    min : Vector2<i64>,
    max : Vector2<i64>,
}

impl Area {
    pub fn increase_area(&mut self, new_point: Vector2<i64>) {
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

#[derive(Eq, Clone)]
pub struct DigLine {
    start : Vector2<i64>,
    end : Vector2<i64>,
    dir : Vector2<i64>,
}

impl Ord for DigLine {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.x.cmp(&other.start.x)
    }
}

impl PartialOrd for DigLine {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.start.x.cmp(&other.start.x))
    }
}

impl PartialEq for DigLine {
    fn eq(&self, other: &Self) -> bool {
        self.start.x == other.start.x
    }
}


pub fn firts_try() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();

    //input file to list of holes
    let instructions = input_string.lines().map(|x| Instruction::from_str(x)).collect::<Vec<Instruction>>();

    let mut dig_pos = Vector2::zero();
    let mut area = Area::default();
    let mut lava_area_size = 0;

    let mut upwards_dig_line = vec![];
    let mut downwards_dig_line = vec![];

    for instruction in instructions {
        
        let mew_dig_pos = dig_pos + instruction.direction * instruction.length as i64;
        if instruction.direction.y > 0 {
            downwards_dig_line.push(DigLine{start : dig_pos, end: mew_dig_pos, dir: instruction.direction});
        } else if instruction.direction.y < 0 {
            upwards_dig_line.push(DigLine{start : dig_pos, end: mew_dig_pos, dir: instruction.direction});
        }

        area.increase_area(mew_dig_pos);
        dig_pos = mew_dig_pos;
    }

    downwards_dig_line.sort_unstable_by(|a, b| a.start.y.cmp(&b.start.y)); //downwads by start position
    upwards_dig_line.sort_unstable_by(|a, b| a.end.y.cmp(&b.end.y)); //upwards by end position

    let mut downwards = downwards_dig_line.into_iter().peekable();
    let mut upwards = upwards_dig_line.into_iter().peekable();

    let mut finish = false;
    let mut cur_y_event = downwards.peek().unwrap().start.y;
    let mut next_y_event;

    let mut active_by_x: Vec<DigLine> = vec![];
    let mut active_downwards: Vec<DigLine> = vec![];
    let mut active_upwards: Vec<DigLine> = vec![];

    while !finish {
        
        next_y_event = i64::MAX;
        let mut lava_line_size = 0;

        while let Some(down_line) = downwards.next_if(|line| line.start.y == cur_y_event) {
            let pos = active_by_x.binary_search_by(|probe| probe.start.x.cmp(&down_line.start.x)).unwrap_or_else(|e| e);
            active_by_x.insert(pos, down_line.clone());
            let pos = active_downwards.binary_search_by(|probe| probe.end.y.cmp(&down_line.end.y)).unwrap_or_else(|e| e);
            active_downwards.insert(pos, down_line);
            next_y_event = cur_y_event + 1;
        }
        if let Some(line_next) = downwards.peek() {
            next_y_event = next_y_event.min(line_next.start.y);
        }

        while let Some(up_line) = upwards.next_if(|line| line.end.y == cur_y_event) {
            let pos = active_by_x.binary_search_by(|probe| probe.start.x.cmp(&up_line.start.x)).unwrap_or_else(|e| e);
            active_by_x.insert(pos, up_line.clone());
            let pos = active_upwards.binary_search_by(|probe| probe.start.y.cmp(&up_line.start.y)).unwrap_or_else(|e| e);
            active_upwards.insert(pos, up_line);
            next_y_event = cur_y_event + 1;
        }
        if let Some(line_next) = upwards.peek() {
            next_y_event = next_y_event.min(line_next.end.y);
        }

        let mut hole_start_x = None;
        let mut inside = false;
        let mut on_edge = None;
        for line in &active_by_x {
            
            if let Some(dir) = on_edge {

                if dir == line.dir { //next line goes into same direction, we change from outsie to inside and vice versa
                    inside = !inside;
                }

                if !inside { //no longer inside
                    //finish here
                    lava_line_size += line.start.x - hole_start_x.unwrap() + 1;
                    hole_start_x = None;
                }
               
                on_edge = None;

            } else {
                if line.start.y == cur_y_event || line.end.y == cur_y_event {
                    on_edge = Some(line.dir);
                    hole_start_x = hole_start_x.or(Some(line.start.x));
                } else if hole_start_x.is_some() {
                    //finish here
                    lava_line_size += line.start.x - hole_start_x.unwrap() + 1;
                    inside = false;
                    hole_start_x = None;
                } else {
                    hole_start_x = Some(line.start.x);
                    inside = true;
                }
            }
            
        }
        
        //remove lines which are no longer used
        while active_downwards.first().is_some_and(|f| f.end.y == cur_y_event) {
            let removed = active_downwards.remove(0); //todo: use VecDeque
            active_by_x.retain(|x| x.start != removed.start && x.end != removed.end);
            next_y_event = cur_y_event + 1; //removed a line, nect line will be different
        }
        if let Some(line_next) = active_downwards.first() {
            next_y_event = next_y_event.min(line_next.end.y);
        }

        while active_upwards.first().is_some_and(|f| f.start.y == cur_y_event) {
            let removed = active_upwards.remove(0); //todo: use VecDeque
            active_by_x.retain(|x| x.start != removed.start && x.end != removed.end);
            next_y_event = cur_y_event + 1; //removed a line, nect line will be different
        }
        if let Some(line_next) = active_upwards.first() {
            next_y_event = next_y_event.min(line_next.start.y);
        }

        lava_area_size += lava_line_size * (next_y_event - cur_y_event);
        cur_y_event = next_y_event;

        finish = active_by_x.is_empty();
    }

    lava_area_size as u64
}





//----------------------------------------------------------------------------------------------------------------------------------------------------------------------------

use itertools::Itertools;

/*Everytime we go right, we add the infinite area to the south, everytime we go left, we subtract the infinite area to the south. 
In addition to this general idea, because the boundary is also included in the area, we need to add up the blocks when traversing downwards (NOT upwards). 
And finally add 1 because we never counted the starting square.  */

pub fn ranjeethmahankali() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();

    part_2(&input_string) as u64
}


fn part_2(input: &str) -> isize {
    // Same as part 1 with different way to parse inputs.
    1 + input
        .trim()
        .lines()
        .fold((0isize, 0isize), |(area, lat), line| {
            let (_, _, hex) = line.split(' ').collect_tuple().unwrap();
            assert_eq!(hex.len(), 9);
            let dist = isize::from_str_radix(&hex[2..7], 16).unwrap();
            let dir = isize::from_str_radix(&hex[7..8], 16).unwrap();
            match dir {
                0 => (area - dist * (lat), lat),
                1 => (area + dist, lat + dist),
                2 => (area + dist * (lat + 1), lat),
                3 => (area, lat - dist),
                _ => (area, lat),
            }
        })
        .0
}