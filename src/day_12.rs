mod lib;

use std::collections::{HashMap};
use itertools::Itertools;
use crate::lib::{read_lines};

fn main() {
    let result_a = task_a(read_lines("input/day_12.txt"));
    assert_eq!(4167, result_a);
    let result_b =  task_b(read_lines("input/day_12.txt"));
    assert_eq!(98441, result_b);
    println!("task-a: {}, task-b: {}", result_a, result_b);
}

fn task_a(lines: impl Iterator<Item=String>) -> usize {
    let arena = read_input(lines);
    let paths = tail(&arena, vec![arena.index_of("start") ], false);
    paths.len()
}

fn task_b(lines: impl Iterator<Item=String>) -> usize {
    let arena = read_input(lines);
    let paths = tail(&arena, vec![arena.index_of("start") ], true);
    paths.len()
}

fn tail(arena: &Arena, head: Vec<usize>, free_double_visit: bool) -> Vec<Vec<usize>> {
    let last = head.last().expect("no head");
    let node = arena.get(*last);
    let end = arena.index_of("end");
    if node.peers.is_empty() {
        vec![head]
    } else {
        node.peers.iter()
            .filter_map(|p| {
                let is_double_visit = arena.get(*p).is_small && head.contains(p);
                if free_double_visit && is_double_visit {
                    Some((p, false))
                }else if is_double_visit {
                    None
                } else {
                    Some((p, free_double_visit))
                }
            })
            .map(|(p, free_double_visit)| {
                let mut h = head.clone();
                h.push(*p);
                tail(arena, h, free_double_visit)
            })
            .flatten()
            .filter(|p| *p.last().expect("empty path") == end)
            .collect_vec()
    }
}



fn read_input(lines: impl Iterator<Item=String>) -> Arena{
    let mut arena = Arena::new();
    lines.for_each(|s| {
        let (from, to) = s.split("-")
            .map(|s|s.to_string())
            .collect_tuple()
            .expect("parse failed");
        arena.add_path(&from, &to);
    });
    arena
}

#[derive(Debug)]
struct Node {peers: Vec<usize>, is_small: bool}
impl Node {
    fn new(is_small: bool) -> Node {
        Node {peers: vec![], is_small}
    }
}

struct Arena {
    node: Vec<Node>,
    node_to_index: HashMap<String, usize>,
}
impl Arena {
    fn new() -> Arena {
        Arena {node: vec![], node_to_index: HashMap::new()}
    }
    fn add_path(&mut self, from: &String, to: &String) {
        let from_index = self.add(from);
        let to_index = self.add(to);


        match (from.as_str(), to.as_str()) {
            ("start", _ ) | (_, "end" )  => {
                self.node[from_index].peers.push(to_index);
            },
            (_, "start" ) | ("end", _ ) => {
                self.node[to_index].peers.push(from_index);
            },
            _ => {
                self.node[to_index].peers.push(from_index);
                self.node[from_index].peers.push(to_index);
            }
        }
    }
    fn add(&mut self, node: &String) -> usize{
        if let Some(index) = self.node_to_index.get(node) {
            *index
        } else {
            let is_small = node != "start" && node != "end" && &node.to_lowercase() == node;
            self.node.push(Node::new(is_small));
            let index = self.node.len()-1;
            self.node_to_index.insert(node.to_string(), index);
            index
        }
    }
    fn get(&self, index: usize) -> &Node {
        self.node.get(index).expect("index not found")
    }
    fn index_of(&self, node: &str) -> usize {
        *self.node_to_index.get(node).expect("node not found")
    }
}


