use core::str;
use std::{collections::HashMap, fs};

use cgmath::{Vector2, Vector3};

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day22/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 22,
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
    459
}

#[derive(Debug, Clone, PartialEq)]
pub struct Brick{
    pub p1: Vector3<u32>,
    pub p2: Vector3<u32>,
}

impl Brick {
    
    pub fn from_strings(s1 : &str, s2 : &str) -> Brick {
       Brick{p1: Self::str2point(s1).unwrap(), p2: Self::str2point(s2).unwrap()}
    }

    fn str2point(s: &str) -> Option<Vector3<u32>> {
        let parts: Vec<_> = s.split(',').collect();
        if parts.len() == 3 {
            Some(Vector3::new(parts[0].parse::<u32>().unwrap(), parts[1].parse::<u32>().unwrap(), parts[2].parse::<u32>().unwrap()))
        } else {
            None
        }
    }

}

impl Default for Brick {
    fn default() -> Self {
        Self { p1: Vector3{ x: 0, y: 0, z: 0 }, p2: Vector3{ x: 0, y: 0, z: 0 } }
    }
}

pub struct BrickNode {
    pub value: Brick,
    pub children: Vec<usize>,
    pub num_parents: usize
}

pub struct BrickTree {
    pub nodes: Vec<BrickNode>,
}

impl BrickTree {
    pub fn new(ground: Brick) -> BrickTree {
        BrickTree{nodes: vec![BrickNode{value: ground, children: vec![], num_parents: 0}]}
    }

    pub fn push(&mut self, brick_to_add : Brick, num_parents: usize) -> usize {
        self.nodes.push(BrickNode { value: brick_to_add, children: vec![], num_parents});
        self.nodes.len()-1
    }

    pub fn search(&self, brick_to_find : &Brick) -> Option<usize> {
        for brick in self.nodes.iter().enumerate() {
            if brick.1.value == *brick_to_find {
                return Some(brick.0);
            }
        }
        None
    } 
}


pub fn first_try() -> u64 {
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();

    let ground = Brick::default();
    let mut bricks = vec![];

    for line in  input_string.lines() {
        let line_parts = line.split('~').collect::<Vec<_>>();
        if line_parts.len() == 2 {
            let temp =Brick::from_strings(line_parts[0], line_parts[1]);
            if temp.p1.x > temp.p2.x || temp.p1.y > temp.p2.y || temp.p1.z > temp.p2.z {
                panic!("crash and burn");
            }
            bricks.push(Brick::from_strings(line_parts[0], line_parts[1]))
        }
    }

    bricks.sort_by(|a, b| a.p1.z.cmp(&b.p1.z));

    //println!("{:?}", bricks[0]);

    let mut brick_fall_area: HashMap<Vector2<u32>, (Brick, usize)> = HashMap::new();
    let mut brick_tree = BrickTree::new(ground.clone());

    for brick in bricks.into_iter() {
        //search bricks below
        let mut fall_height = 0;
        let brick_height = (brick.p2.z - brick.p1.z + 1) as usize;
        let mut bricks_below = vec![ground.clone()];
        for x in brick.p1.x..=brick.p2.x {
            for y in brick.p1.y..=brick.p2.y {
                match brick_fall_area.get(&Vector2::new(x, y)) {
                    Some(brick_found) => {
                        if brick_found.1 == fall_height {
                            if !bricks_below.contains(&brick_found.0) {
                                bricks_below.push(brick_found.0.clone());
                            }
                        } else if brick_found.1 > fall_height {
                            fall_height = brick_found.1;
                            bricks_below = vec![brick_found.0.clone()];
                        }
                    },
                    None => (),
                } 
            }
        }
        //update brick_fall_area
        for x in brick.p1.x..=brick.p2.x {
            for y in brick.p1.y..=brick.p2.y {
                brick_fall_area.entry(Vector2::new(x, y)).and_modify(|e| {e.0 = brick.clone(); e.1 = fall_height + brick_height}).or_insert((brick.clone(), fall_height + brick_height));
            }
        }


        //add new brick into tree
        let brick_id = brick_tree.push(brick, bricks_below.len());
        for brick_below in bricks_below {
            let brick_below_id = brick_tree.search(&brick_below).unwrap();
            brick_tree.nodes[brick_below_id].children.push(brick_id);
        }

    }

    //count how many node ha no child or all childreen have multiple parents
    let mut counter = 0;
    for brick_node in brick_tree.nodes.iter().skip(1) {
        if brick_node.children.iter().map(|child_id| &brick_tree.nodes[*child_id]).filter(|x| x.num_parents == 1).count() == 0 {
            counter += 1;
        }
    }



    counter as u64 //682 is too hight
}
