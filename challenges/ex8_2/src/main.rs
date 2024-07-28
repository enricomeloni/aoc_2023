use std::{
    collections::{HashMap, HashSet},
};

use num::integer::lcm;

use itertools::Itertools;
use utils::{get_input_path, read_lines};

struct Node {
    id: String,
    left: String,
    right: String,
}

struct Map {
    directions: Vec<char>,
    nodes: HashMap<String, Node>,
}

fn is_nodes_set_goal(nodes: &Vec<&Node>) -> bool {
    nodes.iter().all(|node| node.id.ends_with("Z"))
}

impl Map {
    fn get_starting_nodes(&self) -> Vec<&Node> {
        self.nodes
            .iter()
            .filter_map(|(node_id, node)| {
                if node_id.ends_with("A") {
                    Some(node)
                } else {
                    None
                }
            })
            .collect_vec()
    }

    fn count_steps(&self) -> u64 {
        let mut starting_nodes = self.get_starting_nodes();

        let cycle_sizes = starting_nodes
            .iter()
            .map(|start_node| {
                let mut curr_node = *start_node;
                let mut steps: usize = 0;

                while !curr_node.id.ends_with("Z") {
                    let direction = self.directions[steps % self.directions.len()];

                    let next_node_id = match direction {
                        'L' => &curr_node.left,
                        'R' => &curr_node.right,
                        _ => panic!("Should not happen"),
                    };

                    curr_node = &self.nodes[next_node_id];
                    steps += 1;
                }

                steps as u64
            })
            .collect_vec();

        println!("Cycle sizes {:?}, {}", cycle_sizes, u64::MAX);

        cycle_sizes.into_iter().reduce(|a, b| lcm(a, b)).unwrap()
    }

    // fn count_steps(&self) -> u32 {
    //     let mut curr_nodes = self.get_starting_nodes();
    //     let mut steps = 0;

    //     // let visited_nodes = vec![HashSet::<String>::new(); curr_nodes.len()];
    //     let visited_nodes: Vec<HashSet<String>> = curr_nodes.iter().map(|node| {
    //         let mut hash_set = HashSet::<String>::new();
    //         hash_set.insert(node.id.clone());
    //         hash_set
    //     }).collect_vec();

    //     while !is_nodes_set_goal(&curr_nodes) {
    //         let direction = self.directions[steps % self.directions.len()];

    //         let mut next_nodes = curr_nodes.iter().map(|node| {
    //             let next_node_id = match direction {
    //                 'L' => &node.left,
    //                 'R' => &node.right,
    //                 _ => panic!("Should not happen"),
    //             };

    //             return &self.nodes[next_node_id];
    //         }).collect_vec();

    //         for (lane, node) in next_nodes.iter().enumerate() {
    //             if node.id.ends_with("Z") {
    //                 println!("Lane {}, reached goal ({}) at step {} (idx {} of directions)", lane+1, node.id, steps+1, steps % self.directions.len())
    //             }
    //         }

    //         // next_nodes.sort_by_key(|node| &node.id);

    //         // let prev_len = next_nodes.len();
    //         // next_nodes.dedup_by_key(|node| &node.id);
    //         // let next_len = next_nodes.len();

    //         // if prev_len != next_len {
    //         //     println!("filtered")
    //         // }

    //         curr_nodes = next_nodes;
    //         steps += 1;
    //     }

    //     steps as u32
    // }
}

fn main() {
    let input_path = get_input_path(8, Some("input.txt"));

    if let Ok(lines) = read_lines(&input_path) {
        let lines = lines.map(|line| line.unwrap()).collect_vec();

        let directions = lines[0].chars().collect_vec();

        let nodes: HashMap<String, Node> = lines[2..]
            .iter()
            .map(|line| {
                let mut iter = line.split(" = ");
                let key = iter.next().unwrap().to_string();

                let (left_node, right_node): (String, String) = iter
                    .next()
                    .unwrap()
                    .replace("(", "")
                    .replace(")", "")
                    .split(", ")
                    .map(|piece| piece.to_string())
                    .collect_tuple()
                    .unwrap();

                let node = Node {
                    id: key.clone(),
                    left: left_node,
                    right: right_node,
                };

                (key, node)
            })
            .collect();

        let map = Map { directions, nodes };

        let steps = map.count_steps();

        println!("Steps made: {}", steps)
    }
}
