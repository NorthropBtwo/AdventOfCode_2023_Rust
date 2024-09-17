use std::fs;

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day15/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 15,
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
    200277
}

struct Item<'a> {
    label: &'a [u8],
    focal_len: u8,
}

pub fn first_try() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();
    let mut sum = 0;

    const ARRAY_REPEAT_VALUE: Vec<Item> = Vec::new();
    let mut boxes = [ARRAY_REPEAT_VALUE; 256];

    let input = input_string.as_bytes();
    for step in input.split(|&c| c == b',') {

        if let Some(op_pos) = step.iter().position(|&x| x == b'-' || x == b'=') {
            let label = &step[0..op_pos];
            let box_pos = hash(label);
            if step[op_pos] == b'=' {
                if let Some(index) = boxes[box_pos as usize].iter().position(|x| x.label == label) {
                    boxes[box_pos as usize][index].focal_len = step[op_pos+1];
                } else {
                    boxes[box_pos as usize].push(Item{label, focal_len : step[op_pos+1]});
                }
            } else { /* step[op_pos] = - */
                if let Some(index) = boxes[box_pos as usize].iter().position(|x| x.label == label) {
                    boxes[box_pos as usize].remove(index);
                }
            }
            
        }

    }

    for (box_i, boxx) in boxes.into_iter().enumerate() {
        for (slot_i, slot) in boxx.into_iter().enumerate() {
            sum += (box_i+1) * (slot_i+1) * (slot.focal_len - b'0') as usize;
        }
    }

    sum as u64
}

fn hash(input: &[u8]) -> u64 {
    
    let mut value = 0;
    for c in input {
        value += *c as u64;
        value *= 17;
        value &= 0xFF
    }

    value
}