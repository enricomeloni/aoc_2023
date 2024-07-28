use std::collections::HashMap;

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

impl Map {
    fn count_steps(&self) -> u32 {
        let mut curr_node: &Node = &self.nodes["AAA"];
        let mut steps = 0;
        while curr_node.id != "ZZZ" {
            let direction = self.directions[steps % self.directions.len()];

            let next_node_id = match direction {
                'L' => &curr_node.left,
                'R' => &curr_node.right,
                _ => panic!("Should not happen"),
            };

            curr_node = &self.nodes[next_node_id];
            steps += 1;
        }

        steps as u32
    }
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
