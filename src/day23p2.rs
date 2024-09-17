use core::str;
use std::{fs, usize};

use cgmath::Vector2;

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day23/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 23,
        part_nr: 2,
        day_funcs: get_function_list(),
        solution: solution(),
    }
}

pub fn get_function_list() -> Vec<DayFunc> {
    vec![
        //DayFunc{name: "first_try", func : first_try},
        DayFunc{name: "make_a_pre_map", func : make_a_pre_map},
    ]
}

pub fn solution() -> u64 {
    6514
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
    pub steps: u64,
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

    pub fn push(&mut self, node_to_add : Step,steps : u64 , parent: usize) -> usize {
        self.nodes.push(PathNode { value: node_to_add, steps: steps, children: vec![], parent });
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

}


pub struct MapConnection {
    pub steps: u64,
    pub map_positions_indeces: [Vector2<usize>; 2],
}


pub struct MapPosition {
    pub position: Vector2<usize>,
    pub connection_indeces: Vec<usize>,
}

#[derive(Default)]
struct Map {
    pub positions: Vec<Vec<MapPosition>>,
    pub connections : Vec<MapConnection>,
}

impl Map {
    /*pub fn push(&mut self, position : Vector2<usize>, neighbour_index: usize) -> usize {
        let new_pos_idx = self.push_position(position);
        let new_con_idx = self.push_connection([new_pos_idx, neighbour_index]);
        self.positions[new_pos_idx].connection_indeces.push(new_con_idx);
        new_pos_idx
    }

    pub fn push_position(&mut self, position :  Vector2<usize>) -> usize {
        self.positions.push(MapPosition { position: position, connection_indeces: vec![] });
        self.positions.len()-1
    }

    pub fn push_connection(&mut self, position_indeces : [usize; 2]) -> usize {
        self.connections.push(MapConnections { steps: 1, map_positions_indeces: position_indeces});
        self.positions.len()-1
    }*/
}


pub fn make_a_pre_map() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();

    let layout = input_string.lines().map(|x| x.as_bytes().to_vec()).collect::<Vec<Vec<u8>>>();

    let start_x = layout[0].iter().position(|&x| x == b'.').unwrap();

    let mut map = Map::default();

    //add positions
    for y in 0..layout.len() {
        let mut pos_nodes = vec![];
        for x in 0..layout[0].len() {
            pos_nodes.push(MapPosition{position: Vector2 { x, y}, connection_indeces: vec![]})
        }
        map.positions.push(pos_nodes);
    }

    //add connections in y directions
    for y in 1..layout.len() {
        for x in 0..layout[0].len() {
            if layout[y-1][x] != b'#' && layout[y][x] != b'#' {
                map.connections.push(MapConnection { steps: 1, map_positions_indeces: [Vector2::new(x, y-1), Vector2::new(x, y)] });
                map.positions[y-1][x].connection_indeces.push(map.connections.len() - 1);
                map.positions[y][x].connection_indeces.push(map.connections.len() - 1);
            }
        }
    }

    //add connections in x directions
    for x in 1..layout[0].len() {
        for y in 0..layout.len() {
            if layout[y][x-1] != b'#' && layout[y][x] != b'#' {
                map.connections.push(MapConnection { steps: 1, map_positions_indeces: [Vector2::new(x-1, y), Vector2::new(x, y)] });
                map.positions[y][x-1].connection_indeces.push(map.connections.len() - 1);
                map.positions[y][x].connection_indeces.push(map.connections.len() - 1);
            }
        }
    }

    //simplify connections
    for y in 0..layout.len() {
        for x in 0..layout[0].len() {

            let connection_indeces =  map.positions[y][x].connection_indeces.clone();
            if connection_indeces.len() == 2 {
                let pos1 = map.connections[connection_indeces[0]].map_positions_indeces.clone().into_iter().filter(|&v| v != Vector2{x, y}).collect::<Vec<_>>();
                let pos2 = map.connections[connection_indeces[1]].map_positions_indeces.clone().into_iter().filter(|&v| v != Vector2{x, y}).collect::<Vec<_>>();
                if pos1.len() != 1 || pos2.len() != 1 {
                    panic!("pos1 or pos2 length != 1");
                }
                map.connections[connection_indeces[0]].steps += map.connections[connection_indeces[1]].steps;
                map.connections[connection_indeces[0]].map_positions_indeces = [pos1[0], pos2[0]];

                //cleanup
                map.connections[connection_indeces[1]].steps = 0;
                map.positions[y][x].connection_indeces = vec![];
                for positions in [pos1, pos2] {
                    for position in positions {
                        for con in &mut map.positions[position.y][position.x].connection_indeces {
                            if *con == connection_indeces[1] {
                                *con = connection_indeces[0];
                            }
                        }
                    }
                }
            }
        }
    }

    //run alogrithm ver simplifed map

    let mut active_pos = vec![Vector2::new(start_x, 0)];

    let cur_pos = active_pos.pop().unwrap();
    let mut path_tree = PathTree::new(Vector2 { x: cur_pos.x, y: cur_pos.y });

    let mut active_indices = vec![0 as usize];
    let mut max_num_steps = 0;



    while active_indices.len() > 0 {
    
        let cur_idx = active_indices.pop().unwrap();
        let cur_pos = path_tree.nodes[cur_idx].value.position;
        let cur_num_steps = path_tree.nodes[cur_idx].steps;

        let next_connections_indeces = map.positions[cur_pos.y][cur_pos.x].connection_indeces.clone();
        let mut next_positions = vec![];
        for next_connections in next_connections_indeces.iter().map(|x| &map.connections[*x]) {
            next_positions.push((next_connections.map_positions_indeces[0], next_connections.steps));
            next_positions.push((next_connections.map_positions_indeces[1], next_connections.steps));
        }


        for (position, steps) in next_positions {
            let next_num_steps = cur_num_steps + steps;
            if position.y == (layout.len()-1) {
                max_num_steps = max_num_steps.max(next_num_steps);
                break;
            } else {
                if !path_tree.is_parent(cur_idx, &position) {
                    let new_idx = path_tree.push(Step {position},next_num_steps, cur_idx);
                    active_indices.push(new_idx);
                }             
            }

        }



    }




    max_num_steps as u64
}