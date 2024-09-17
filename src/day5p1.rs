use std::{fs, str::FromStr, num::ParseIntError};

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day5/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 5,
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

pub fn solution() -> u64 {
    31599214
}

struct MapElement {
    destination_start: u32,
    source_start: u32,
    length: u32,
}


impl FromStr for MapElement {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut word_iter = s.split(' ');

        let destination_start = word_iter.next().unwrap_or_default().parse::<u32>()?;
        let source_start = word_iter.next().unwrap_or_default().parse::<u32>()?;
        let length = word_iter.next().unwrap_or_default().parse::<u32>()?;

        Ok(MapElement {destination_start, source_start, length})
    }
}


pub fn first_try() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();
    let mut seeds = vec![];

    let mut line_iter = input_string.lines();

    for seed in line_iter.next().unwrap().split_ascii_whitespace().skip(1) {
        if let Ok(seed) = seed.parse::<u32>() {
            seeds.push(seed);
        }
    }

    let mut line_iter = line_iter.skip(1);

    while let Some(_) = line_iter.next() {

        let mut map_elements = vec![];
        while let Ok(map_element) = MapElement::from_str(line_iter.next().unwrap_or_default()) {
            map_elements.push(map_element)
        }

        for seed in seeds.iter_mut() {
            for map_element in map_elements.iter() {
                if *seed >= map_element.source_start && *seed <= map_element.source_start + (map_element.length-1) {
                    *seed = seed.wrapping_sub(map_element.source_start) + map_element.destination_start;
                    break;
                }
            }
        }


    }

    seeds.into_iter().min().unwrap_or_default() as u64
}
