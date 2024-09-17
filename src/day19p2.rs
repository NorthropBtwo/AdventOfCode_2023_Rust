use core::str;
use std::{collections::{HashMap, VecDeque}, fs, ops::Index, u32};

use cgmath::{Vector2, Zero};

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day19/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 19,
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
    129249871135292
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

        let mut con_parts= vec![];
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


#[derive(Default, Clone, Copy)]
struct PartCategoryRange {
    pub start : u32,
    pub length : u32,
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
    let part_range = [PartCategoryRange{start: 1, length: 4000}; 4];
    let active_workflow = "in";

    sum = process_workflow(part_range, active_workflow, &workflows);

    sum as u64
}

fn process_workflow(part_range: [PartCategoryRange; 4], workflow_name :&str, workflows: &HashMap<&str, Vec<Rule>>) -> u64 {

    let mut part_range = part_range;
    let mut sum = 0;

    if workflow_name == "A" {
        return part_range[0].length as u64 * part_range[1].length as u64 * part_range[2].length as u64 * part_range[3].length as u64;
    } else if workflow_name == "R"  {
        return 0;
    }

    for rule in &workflows[workflow_name] {
        if let Some(condition) = &rule.condition {
            let mut category_range = part_range[condition.category];
            if condition.bigger {
                if category_range.start > condition.value {
                    sum += process_workflow(part_range, &rule.next_workflow, workflows);
                    break;
                } else if (category_range.start + category_range.length - 1) > condition.value {
                    let new_category_range = PartCategoryRange{start: condition.value+1, length: category_range.start + category_range.length - 1 - condition.value};
                    let mut new_part_range = part_range;
                    new_part_range[condition.category] = new_category_range;
                    sum += process_workflow(new_part_range, &rule.next_workflow, workflows);

                    category_range.length = condition.value - category_range.start + 1;
                    part_range[condition.category] = category_range;
                }
            } else { /* !condition.bigger */
                if category_range.start + category_range.length -1 < condition.value {
                    sum += process_workflow(part_range, &rule.next_workflow, workflows);
                    break;
                } else if category_range.start < condition.value {
                    let new_category_range = PartCategoryRange{start: category_range.start, length: condition.value - category_range.start};
                    let mut new_part_range = part_range;
                    new_part_range[condition.category] = new_category_range;
                    sum += process_workflow(new_part_range, &rule.next_workflow, workflows);

                    category_range.length = category_range.start + category_range.length - condition.value;
                    category_range.start = condition.value;
                    part_range[condition.category] = category_range;
                }
            }
        } else {
            sum += process_workflow(part_range, &rule.next_workflow, workflows);
        }
    }
    

    sum
}