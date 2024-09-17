use std::{collections::HashMap, fs};
use rand::Rng;

use crate::{DayFunc, DayRiddle};

const INPUT_PATH: &str = "src/day25/input.txt";

pub fn get_riddle() -> DayRiddle {
    DayRiddle{
        day_nr: 25,
        part_nr: 1,
        day_funcs: get_function_list(),
        solution: solution(),
    }
}

pub fn get_function_list() -> Vec<DayFunc> {
    vec![
        DayFunc{name: "Karger's algorithm", func : first_try},
    ]
}

pub fn solution() -> u64 {
    602151
}


pub fn first_try() -> u64 {
    let mut graph;
    let input_string = fs::read_to_string(INPUT_PATH).unwrap();
    let lines = input_string.lines().collect::<Vec<_>>();

    //create graph
    graph = Graph::default();
    for line in lines {
        let line_parts = line.split(':').collect::<Vec<_>>();
        if line_parts.len() == 2 {
            let startpoint = line_parts[0];
            let endpoints = line_parts[1].trim().split(' ').collect::<Vec<_>>();
            for endpoint in endpoints {
                graph.insert_connection(startpoint, endpoint);
            }

        }

    }

    let graph_master_copy = graph;

    //loop until a valid solution is found
    loop {
        graph = graph_master_copy.clone();
        //Karger's algorithm
        while graph.active_connections_indeces.len() > 3 {
            graph.merge_two_nodes_at_random();
        }
        //println!("graph.active_connections_indeces.len: {}", graph.active_connections_indeces.len());
        if graph.active_connections_indeces.len() == 3 {
            break;
        }
    }

    let [final_node1, final_node2] = graph.connections[graph.active_connections_indeces[0]].node_indeces;
    graph.nodes[final_node1].cluster_node_count*graph.nodes[final_node2].cluster_node_count
}

impl Graph {
    fn insert_connection(&mut self,startpoint: &str, endpoint: &str) {
        
        let node_index_1 = match self.node_map.get(startpoint) {
            Some(node_index) => *node_index,
            None => {
                let node_index = self.nodes.len();
                self.nodes.push(Node{name: startpoint.to_string(), connection_indeces: vec![], cluster_node_count: 1});
                self.node_map.insert(startpoint.to_string(), node_index);
                node_index
            },
        };

        let node_index_2 = match self.node_map.get(endpoint) {
            Some(node_index) => *node_index,
            None => {
                let node_index = self.nodes.len();
                self.nodes.push(Node{name: endpoint.to_string(), connection_indeces: vec![], cluster_node_count: 1});
                self.node_map.insert(endpoint.to_string(), node_index);
                node_index
            },
        };

        let connection_index = self.connections.len();
        self.connections.push(Connection { node_indeces: [node_index_1, node_index_2] });
        self.active_connections_indeces.push(connection_index);

        self.nodes[node_index_1].connection_indeces.push(connection_index);
        self.nodes[node_index_2].connection_indeces.push(connection_index);
    }

    fn merge_two_nodes_at_random(&mut self) {
        //get a random connection
        let mut rng = rand::thread_rng();
        let mut connection_indeces_to_remove = vec![];

        let active_connection_index =  rng.gen_range(0..self.active_connections_indeces.len());
        let connection_index = self.active_connections_indeces[active_connection_index];
        let [old_node_idx, new_node_idx] =  self.connections[connection_index].node_indeces;
        let connection_indeces_to_change = self.nodes[old_node_idx].connection_indeces.clone();
        for connection_index in connection_indeces_to_change {
            let connection = &mut self.connections[connection_index];
            if connection.node_indeces[0] == old_node_idx {
                connection.node_indeces[0] = new_node_idx;
            }
            if connection.node_indeces[1] == old_node_idx {
                connection.node_indeces[1] = new_node_idx;
            }
            if connection.node_indeces[0] == connection.node_indeces[1] {
                connection_indeces_to_remove.push((connection_index,connection.node_indeces));
            }
            self.nodes[new_node_idx].connection_indeces.push(connection_index);
        }
        //remove selfloop connections
        for (connection_index,node_indeces) in connection_indeces_to_remove {

            let connection_indeces = &mut self.nodes[node_indeces[0]].connection_indeces;
            let remove_indx = connection_indeces.iter().position(|x| *x == connection_index).unwrap();
            connection_indeces.remove(remove_indx);
            let connection_indeces = &mut self.nodes[node_indeces[1]].connection_indeces;
            connection_indeces.remove(connection_indeces.iter().position(|x| *x == connection_index).unwrap());

            self.active_connections_indeces.remove(self.active_connections_indeces.iter().position(|x| *x == connection_index).unwrap());
        }

        //increase node merge counter
        self.nodes[new_node_idx].cluster_node_count += self.nodes[old_node_idx].cluster_node_count;

    }

}

#[derive(Default, Clone)]
struct Graph{
    pub connections: Vec<Connection>,
    pub node_map: HashMap<String, usize>,
    pub nodes: Vec<Node>,
    pub active_connections_indeces: Vec<usize>,
}

#[derive(Clone)]
struct Connection {
    pub node_indeces: [usize; 2],
}

#[derive(Clone)]
struct Node {
    pub name: String,
    pub connection_indeces: Vec<usize>,
    pub cluster_node_count: u64,
}