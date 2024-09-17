use std::{fs, str::FromStr, num::ParseIntError};

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day5/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 5,
        part_nr: 2,
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
    20358599
}

#[derive(Eq, Default, Debug)]
struct SeedRange {
    start: u64,
    length: u64,
}

impl Ord for SeedRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for SeedRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for SeedRange {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start
    }
}

#[derive(Debug)]
struct MapElement {
    destination_start: u64,
    source_start: u64,
    length: u64,
}


impl FromStr for MapElement {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut word_iter = s.split(' ');

        let destination_start = word_iter.next().unwrap_or_default().parse::<u64>()?;
        let source_start = word_iter.next().unwrap_or_default().parse::<u64>()?;
        let length = word_iter.next().unwrap_or_default().parse::<u64>()?;

        Ok(MapElement {destination_start, source_start, length})
    }
}


pub fn first_try() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();
    let mut seed_ranges = vec![];

    let mut line_iter = input_string.lines();

    let mut seed_iter = line_iter.next().unwrap().split_ascii_whitespace().skip(1);

    while let (Some(seed_start), Some(seed_length)) = (seed_iter.next(), seed_iter.next()) {
        if let (Ok(seed_start), Ok(seed_length)) = (seed_start.parse::<u64>(), seed_length.parse::<u64>()) {
            seed_ranges.push(SeedRange{start: seed_start, length: seed_length});
        }
    }

    let mut line_iter = line_iter.skip(1);

    while let Some(_) = line_iter.next() {

        let mut map_elements = vec![];
        while let Ok(map_element) = MapElement::from_str(line_iter.next().unwrap_or_default()) {
            map_elements.push(map_element)
        }

        let mut new_seed_ranges: Vec<SeedRange> = vec![];
        for map_element in map_elements.iter() {
            let mut i = 0;
            let mut not_converted_ranges: Vec<SeedRange> = vec![];
            while i < seed_ranges.len() {
                /* case 1: map_element matches lower part of seed_ranges */
                if map_element.source_start <= seed_ranges[i].start && (map_element.source_start as u64 + map_element.length as u64) < (seed_ranges[i].start as u64 + seed_ranges[i].length as u64) && map_element.source_start + map_element.length > seed_ranges[i].start {
                    let new_seed_range_length =  map_element.source_start + map_element.length - seed_ranges[i].start;
                    new_seed_ranges.push(convert_range(SeedRange { start: seed_ranges[i].start, length: new_seed_range_length }, map_element));
                    not_converted_ranges.push(SeedRange { start: seed_ranges[i].start + new_seed_range_length, length: seed_ranges[i].length - new_seed_range_length})
                /* case 2: map_element inside seed_ranges */
                } else if map_element.source_start > seed_ranges[i].start && (map_element.source_start as u64 + map_element.length as u64) < (seed_ranges[i].start as u64 + seed_ranges[i].length as u64) {
                    let new_seed_range_length =  map_element.length;
                    new_seed_ranges.push(convert_range(SeedRange { start: map_element.source_start, length: new_seed_range_length }, map_element));
                    not_converted_ranges.push(SeedRange { start: seed_ranges[i].start, length: map_element.source_start - seed_ranges[i].start});
                    //not_converted_ranges.push(SeedRange { start: map_element.source_start + map_element.length, length: seed_ranges[i].start - seed_ranges[i].length - (map_element.source_start + map_element.length)})
                    not_converted_ranges.push(SeedRange { start: map_element.source_start + map_element.length, length: seed_ranges[i].start + seed_ranges[i].length - (map_element.source_start + map_element.length)})
                /* case 4: map_element contains seed_ranges */
                } else if map_element.source_start <= seed_ranges[i].start && map_element.source_start + map_element.length >= seed_ranges[i].start + seed_ranges[i].length {
                    new_seed_ranges.push(convert_range(SeedRange { start: seed_ranges[i].start, length: seed_ranges[i].length }, map_element));
                /* case 3: map_element matches upper part of seed_ranges */
                } else if map_element.source_start < seed_ranges[i].start + seed_ranges[i].length && map_element.source_start > seed_ranges[i].start {
                    let new_seed_range_length = seed_ranges[i].start + seed_ranges[i].length - map_element.source_start;
                    new_seed_ranges.push(convert_range(SeedRange { start: map_element.source_start, length: new_seed_range_length }, map_element));
                    not_converted_ranges.push(SeedRange { start: seed_ranges[i].start, length: seed_ranges[i].length - new_seed_range_length})
                /* ranges dont touch each others */
                } else {
                    not_converted_ranges.push(SeedRange { start: seed_ranges[i].start, length: seed_ranges[i].length });
                }
                i = i + 1;
            }
            seed_ranges = not_converted_ranges;
        }


        new_seed_ranges.append(&mut seed_ranges);
        seed_ranges = new_seed_ranges;


    }

    seed_ranges.into_iter().min().unwrap_or_default().start as u64
}

fn convert_range(range : SeedRange, map_element: &MapElement) -> SeedRange {
    SeedRange { start: range.start.wrapping_sub(map_element.source_start) + map_element.destination_start, length: range.length}
}