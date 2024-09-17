use core::str;
use std::{collections::HashMap, fs, u32};

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day19/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 19,
        part_nr: 1,
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
    421983
}

fn index_of_category(category: &str) -> usize {
    match category {
        "x" => 0,
        "m" => 1,
        "a" => 2,
        "s" => 3,
        _ => panic!("invalid category"),
    }
}

struct Condition {
    pub category : usize,
    pub bigger : bool,
    pub value : u32,
}

impl Condition {
    pub fn from_str(s: &str) -> Option<Condition> {

        let con_parts;
        let mut bigger = false;

        if s.contains('>') {
            con_parts = s.split('>').collect::<Vec<_>>();
            bigger = true;
        } else {
            con_parts = s.split('<').collect::<Vec<_>>();
        }

        if con_parts.len() == 2 {
            let category = index_of_category(con_parts[0]);
            let value = con_parts[1].parse::<u32>().unwrap();
            Some(Condition{ category, bigger, value })
        } else {
            None
        }
    }
}

#[derive(Default)]
struct Rule {
    pub condition : Option<Condition>,
    pub next_workflow : String,
}

impl Rule {
    pub fn from_str(s: &str) -> Rule {
        let mut rule = Rule::default();

        let rule_parts = s.split(':').collect::<Vec<_>>();
        if rule_parts.len() == 1 {
            rule.next_workflow = rule_parts[0].to_string();
        } else if rule_parts.len() == 2 {
            rule.condition = Condition::from_str(rule_parts[0]);
            rule.next_workflow = rule_parts[1].to_string();
        }

        rule
    }
}

pub fn first_try() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();

    let mut workflows = HashMap::new();
    let mut sum = 0;

    let mut line_iter = input_string.lines();
    //process workflows
    while let Some(line) = line_iter.next() {
        let mut rules = vec![];
        
        let line_parts = line.split('{').collect::<Vec<_>>();
        if line_parts.len() != 2 {
            break;
        }
        let workflow_name = line_parts[0];
        let rules_str = line_parts[1].split('}').next().unwrap().split(',');
        for rule_str in rules_str {
            rules.push(Rule::from_str(rule_str))
        }

        workflows.insert(workflow_name, rules);
    }

    //process parts
    for line in line_iter {
        let line = line.trim_matches(['{', '}']);
        let categories = line.split(',').map(|x| x.trim_matches(|c: char| !c.is_ascii_digit())).map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>();

        let mut active_workflow = "in";
        while active_workflow != "R" && active_workflow != "A" {
            let workflow = &workflows[&active_workflow];
            for rule in workflow {
                if let Some(condition) = &rule.condition {
                    if condition.bigger {
                        if categories[condition.category] > condition.value {
                            active_workflow = &rule.next_workflow;
                            break;
                        }
                    } else {
                        if categories[condition.category] < condition.value {
                            active_workflow = &rule.next_workflow;
                            break;
                        }
                    }
                } else {
                    active_workflow = &rule.next_workflow;
                    break;
                }
            }

        }

        if active_workflow == "A" {
            sum += categories.iter().sum::<u32>();
        }

    }

    sum as u64
}

