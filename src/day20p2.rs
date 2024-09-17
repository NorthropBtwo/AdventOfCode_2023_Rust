use core::str;
use std::{collections::{HashMap, VecDeque}, fs};

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day20/input.txt";
const FLIPFLOP: &str = "%";
const CONJUNCTION: &str = "&";
const BROADCASTER: &str = "b";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 20,
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
    221453937522197
}

struct Module<'a> {
    pub module_type : &'a str,
    pub name : &'a str,
    pub outputs : Vec<Output<'a>>,
    pub state_index : usize,
}

struct Output<'a> {
    pub name : &'a str,
    pub id : usize,
}

struct Signal<'a> {
    pub wire : &'a str,
    pub state : bool,
    pub output_id: usize,
}


pub fn first_try() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();

    let mut modules = HashMap::new();

    //read modules
    for line in input_string.lines() {
        let line_parts = line.split("->").collect::<Vec<_>>();
        if line_parts.len() == 2 {
            let (module_type, module_name) = line_parts[0].trim().split_at(1);
            let mut outputs = vec![];
            for output in line_parts[1].split(',') {
                let output = output.trim();
                outputs.push(Output{name: output, id : 0} );
            }
            modules.insert(module_name, Module{name: module_name, module_type, outputs, state_index : 0});
        }
    }

    //add an id to outputs to a Conjunction which is used later to access the states and to know how many inputs a conjuctions has
    let mut id_counters = HashMap::new();
    for module in modules.values_mut() {
        for output in module.outputs.iter_mut() {
            let next_id = id_counters.entry(output.name).or_insert(0);
            output.id = *next_id;
            *next_id += 1;
        }
    }


    //generate state list
    let mut states = vec![false];
    let mut next_idx = states.len();
    for module in modules.values_mut() {
        match module.module_type {
            FLIPFLOP => {
                module.state_index = next_idx;
                states.push(false);
                next_idx += 1;
            },
            CONJUNCTION => {
                module.state_index = next_idx;
                let num_states = id_counters.get(module.name).unwrap_or(&1);
                states.append(&mut vec![false; *num_states]);
                next_idx += num_states;
            }
            BROADCASTER => {
                module.state_index = 0;
            }
            _ => {
                panic!("Unknown module type");
            }
        }
    }

    let mut cycle_detector = [0; 4];
    for i in 1..5000 {

        //push button to generate a low pulse
        let mut signals = VecDeque::new(); //Queue
        signals.push_back(Signal{state: false, wire: "roadcaster", output_id: 0}); //B was used as the type

        //iterate over signal
        while signals.len() > 0 {
            let mut new_signals = VecDeque::new();
            for signal in signals {
                
                if let Some(module) = modules.get(signal.wire) {
                    match module.module_type {
                        FLIPFLOP => {
                            if !signal.state { //ignore positive pulse
                                let new_state = !states[module.state_index];
                                for output in &module.outputs {
                                    new_signals.push_back(Signal{state: new_state, wire: output.name, output_id: output.id});
                                }
                                states[module.state_index] = new_state;
                            }
                        },
                        CONJUNCTION => {
                            let state_start_idx = module.state_index;
                            let state_length_idx = id_counters[module.name];
                            let states = &mut states[state_start_idx..(state_start_idx + state_length_idx)];
                            states[signal.output_id] = signal.state;
                            let output_state = !states.iter().all(|&x| x == true); //basically a nand gate
                            for output in &module.outputs {
                                new_signals.push_back(Signal{state: output_state, wire: output.name, output_id: output.id});
                            }
                            if module.name == "hf" && signal.state {
                                cycle_detector[signal.output_id] = i;
                            }
                        },
                        BROADCASTER => {
                            for output in &module.outputs {
                                new_signals.push_back(Signal{state: signal.state, wire: output.name, output_id: output.id});
                            }
                        }
                        _ => {
                            panic!("Unknown module type");
                        }
                    }
                }
            }
            signals = new_signals;   
        }

        
    }

    cycle_detector.iter().product::<u64>()
}

