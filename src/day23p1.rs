use core::str;
use std::{fs, usize};

use cgmath::Vector2;

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day23/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 23,
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
    2110
}


#[derive(Debug, Clone, PartialEq)]
pub struct Step{
    pub position: Vector2<usize>,
}

impl Default for Step {
    fn default() -> Self {
        Self { position: Vector2::new(0, 0) }
    }
}

impl Step {

}

#[derive(Default)]
pub struct PathNode {
    pub value: Step,
    pub children: Vec<usize>,
    pub parent: usize,
}

#[derive(Default)]
pub struct PathTree {
    pub nodes: Vec<PathNode>,
}

impl PathTree {


    pub fn new(start: Vector2<usize>) -> PathTree {
        PathTree{nodes: vec![PathNode{value: Step{position: start}, ..Default::default()}]}
    }

    pub fn push(&mut self, node_to_add : Step, parent: usize) -> usize {
        self.nodes.push(PathNode { value: node_to_add, children: vec![], parent });
        self.nodes.len()-1
    }

    pub fn is_parent(&mut self, child_index: usize ,search_pos : &Vector2<usize>) -> bool {
        let mut node = &self.nodes[child_index];
        while node.parent != 0 {
            if node.value.position == *search_pos {
                return true;
            }
            node = &self.nodes[node.parent];
        }

        node.value.position == *search_pos || self.nodes[node.parent].value.position == *search_pos
    }

    pub fn count_parents(&mut self, child_index: usize) -> usize {
        let mut count = 0;
        let mut node = &self.nodes[child_index];
        while node.parent != 0 {
            count += 1;
            node = &self.nodes[node.parent];
        }

        count
    }

   /*  pub fn search(&self, brick_to_find : &Brick) -> Option<usize> {
        for brick in self.nodes.iter().enumerate() {
            if brick.1.value == *brick_to_find {
                return Some(brick.0);
            }
        }
        None
    }*/ 
}

pub fn first_try() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();

    let layout = input_string.lines().map(|x| x.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();
    
    let start_x = layout[0].iter().position(|&x| x == b'.').unwrap();
    
    let mut path_tree = PathTree::new(Vector2 { x: start_x, y: 0 });
    path_tree.push(Step { position: Vector2 { x: start_x, y: 1 } }, 0);

    let mut active_indices = vec![1 as usize];
    let mut max_num_parents = 0; 

    while active_indices.len() > 0 {
        
        let cur_idx = active_indices.pop().unwrap();
        let cur_pos = path_tree.nodes[cur_idx].value.position;
        
        let next_positions = match layout[cur_pos.y][cur_pos.x] {
            b'.' => vec![Vector2::new(cur_pos.x+1, cur_pos.y),Vector2::new(cur_pos.x, cur_pos.y+1),Vector2::new(cur_pos.x-1, cur_pos.y),Vector2::new(cur_pos.x, cur_pos.y-1)],
            b'v' => vec![Vector2::new(cur_pos.x, cur_pos.y+1)],
            b'>' => vec![Vector2::new(cur_pos.x+1, cur_pos.y)],
            b'<' => vec![Vector2::new(cur_pos.x-1, cur_pos.y)],
            _ => panic!("unknown input character"),
        };

        for position in next_positions {
            if position.y == layout.len() {
                max_num_parents = max_num_parents.max(path_tree.count_parents(cur_idx));
            } else {
                if layout[position.y][position.x] != b'#' {
                    if !path_tree.is_parent(cur_idx, &position) {
                        let new_idx = path_tree.push(Step {position}, cur_idx);
                        active_indices.push(new_idx);
                    }
                }                
            }

        }


    }


    max_num_parents as u64 + 1
}
