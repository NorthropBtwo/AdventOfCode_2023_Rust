use std::{collections::HashMap, fs};

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day12/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 12,
        part_nr: 2,
        day_funcs: get_function_list(),
        solution: solution(),
    }
}

pub fn get_function_list() -> Vec<DayFunc> {
    vec![
        DayFunc{name: "dynamic_programming", func : dynamic_programming},
        DayFunc{name: "dynamic_programming2", func : dynamic_programming2},
    ]
}

pub fn solution() -> u64 {
    204640299929836
}


#[inline(always)]
fn insert_and_return(map: &mut HashMap<usize,u64>, key : usize, value : u64) -> u64 {
    map.insert(key, value);
    value
}

fn process_next_step_lt(springs: &[u8], spring_idx: usize, group_sizes: &[usize], group_size_idx: usize, map: &mut HashMap<usize,u64>) -> u64 {


    let key = (spring_idx << 32) + group_size_idx;
    if let Some(result) = map.get(&key) {
        return *result;
    }
    
    let fin1 = spring_idx >= springs.len();
    let fin2 = group_size_idx >= group_sizes.len();

    if fin1 && fin2 {
        return 1;
    } else if fin2 && springs[spring_idx] != b'#' {
        let result = process_next_step_lt(springs, spring_idx+1, group_sizes, group_size_idx, map);
        return insert_and_return(map, key, result);
    } else if fin1 || fin2 {
        return 0;
    }

    match springs[spring_idx] {
        b'.' => {
            let result = process_next_step_lt(springs, spring_idx+1, group_sizes, group_size_idx, map);
            insert_and_return(map, key, result)
        },
        b'#' => {
            let grou_size = group_sizes[group_size_idx];

            if spring_idx + grou_size -1 >= springs.len() {
                return 0;
            }

            for i in spring_idx..(spring_idx + grou_size) {
                if springs[i] == b'.' {
                    return 0;
                }
            }

            if spring_idx + grou_size < springs.len() && springs[spring_idx + grou_size] == b'#' {
                return 0;
            }
            let result = process_next_step_lt(springs, spring_idx + grou_size + 1, group_sizes, group_size_idx + 1, map);
            insert_and_return(map, key, result)
        },
        b'?' => {
            let grou_size = group_sizes[group_size_idx];

            if spring_idx + grou_size -1 >= springs.len() {
                let result = process_next_step_lt(springs, spring_idx+1, group_sizes, group_size_idx, map);
                return insert_and_return(map, key, result);
            }

            for i in spring_idx..(spring_idx + grou_size) {
                if springs[i] == b'.' {
                    let result = process_next_step_lt(springs, spring_idx+1, group_sizes, group_size_idx, map);
                    return insert_and_return(map, key,result);
                }
            }
            
            if spring_idx + grou_size < springs.len() && springs[spring_idx + grou_size] == b'#' {
                let result = process_next_step_lt(springs, spring_idx+1, group_sizes, group_size_idx, map);
                return insert_and_return(map, key,result);
            }
            let mut result = process_next_step_lt(springs, spring_idx + grou_size + 1, group_sizes, group_size_idx + 1, map);
            result += process_next_step_lt(springs, spring_idx+1, group_sizes, group_size_idx, map);
            insert_and_return(map, key, result)
        },
        _ => 0
    }
}


pub fn dynamic_programming() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();
    let mut possibilities = 0;

    
    for line in input_string.lines().map(|x| x.as_bytes()) {
        let mut line_parts = line.split(|&x| x == b' ');
        let springs = line_parts.next().unwrap();
        let springs = [springs, &[b'?'], springs, &[b'?'], springs, &[b'?'], springs, &[b'?'], springs].concat();
        let spring_group_sizes = line_parts.next().unwrap().split(|&x| x == b',').collect::<Vec<&[u8]>>();
        let spring_group_sizes = spring_group_sizes.into_iter().map(|x| atoi::atoi::<usize>(x).unwrap()).collect::<Vec<usize>>();
        let spring_group_sizes = [spring_group_sizes.clone(), spring_group_sizes.clone(), spring_group_sizes.clone(), spring_group_sizes.clone(), spring_group_sizes].concat();

        let local_possiblities = process_next_step_lt(&springs, 0, &spring_group_sizes, 0, &mut HashMap::new());

        possibilities += local_possiblities;
    }

   

    return possibilities;
}


fn process_next_step_lt2(springs: &[u8], spring_idx: usize, group_sizes: &[usize], group_size_idx: usize, map: &mut HashMap<usize,u64>) -> u64 {

    let key = (spring_idx << 32) + group_size_idx;
    if let Some(result) = map.get(&key) {
        return *result;
    }
    
    let fin1 = spring_idx >= springs.len();
    let fin2 = group_size_idx >= group_sizes.len();

    if fin1 && fin2 {
        return insert_and_return(map, key, 1);
    }
    if fin2 && springs[spring_idx] != b'#' {
        let result = process_next_step_lt2(springs, spring_idx+1, group_sizes, group_size_idx, map);
        return insert_and_return(map, key, result);
    }
    if fin1 || fin2 {
        return insert_and_return(map, key,0);
    }

    match springs[spring_idx] {
        b'.' => {
            let result = process_next_step_lt2(springs, spring_idx+1, group_sizes, group_size_idx, map);
            insert_and_return(map, key, result)
        }
        b'#' => {
            let grou_size = group_sizes[group_size_idx];

            if spring_idx + grou_size -1 >= springs.len() {
                return insert_and_return(map, key, 0);
            }

            for i in spring_idx..(spring_idx + grou_size) {
                if springs[i] == b'.' {
                    return insert_and_return(map, key,0);
                }
            }

            if spring_idx + grou_size < springs.len() && springs[spring_idx + grou_size] == b'#' {
                return insert_and_return(map, key,0);
            }
            let result = process_next_step_lt2(springs, spring_idx + grou_size + 1, group_sizes, group_size_idx + 1, map);
            insert_and_return(map, key, result)
        },
        b'?' => {
            let grou_size = group_sizes[group_size_idx];

            if spring_idx + grou_size -1 >= springs.len() {
                let result = process_next_step_lt2(springs, spring_idx+1, group_sizes, group_size_idx, map);
                return insert_and_return(map, key, result);
            }

            for i in spring_idx..(spring_idx + grou_size) {
                if springs[i] == b'.' {
                    let result = process_next_step_lt2(springs, spring_idx+1, group_sizes, group_size_idx, map);
                    return insert_and_return(map, key, result);
                }
            }
            
            if spring_idx + grou_size < springs.len() && springs[spring_idx + grou_size] == b'#' {
                let result = process_next_step_lt2(springs, spring_idx+1, group_sizes, group_size_idx, map);
                return insert_and_return(map, key, result);
            }
            let mut result = process_next_step_lt2(springs, spring_idx + grou_size + 1, group_sizes, group_size_idx + 1, map);
            result += process_next_step_lt2(springs, spring_idx+1, group_sizes, group_size_idx, map);
            insert_and_return(map, key, result)
        },
        _ => insert_and_return(map, key,0)
    }
}

pub fn dynamic_programming2() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();
    let mut possibilities = 0;

    
    for line in input_string.lines().map(|x| x.as_bytes()) {
        let mut line_parts = line.split(|&x| x == b' ');
        let springs = line_parts.next().unwrap();
        let springs = [springs, &[b'?'], springs, &[b'?'], springs, &[b'?'], springs, &[b'?'], springs].concat();
        let spring_group_sizes = line_parts.next().unwrap().split(|&x| x == b',').collect::<Vec<&[u8]>>();
        let spring_group_sizes = spring_group_sizes.into_iter().map(|x| atoi::atoi::<usize>(x).unwrap()).collect::<Vec<usize>>();
        let spring_group_sizes = [spring_group_sizes.clone(), spring_group_sizes.clone(), spring_group_sizes.clone(), spring_group_sizes.clone(), spring_group_sizes].concat();

        let local_possiblities = process_next_step_lt2(&springs, 0, &spring_group_sizes, 0, &mut HashMap::new());


        possibilities += local_possiblities;
    }

   

    return possibilities;
}