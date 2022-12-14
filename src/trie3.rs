use std::collections::HashMap;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

pub struct Node {
    connections: HashMap<char, Node>,
    end: bool,
}

impl Node {
    pub fn new() -> Node {
        Node {connections: HashMap::new(), end: false}
    }


    pub fn add(&mut self, word: &String) {
        let mut current_node = self;
        let mut new: Node;
        for i in 0..word.len() {
            current_node.connections.entry(word.chars().nth(i).unwrap()).or_insert(Node {connections: HashMap::new(), end: false});
            current_node = current_node.connections.get_mut(&word.chars().nth(i).unwrap()).unwrap();
        }
        current_node.end = true;
    }

    pub fn search(&self, word: &String) -> bool {
        let mut current_node = self;
        for i in 0..word.len() {
            if let None = current_node.connections.get(&word.chars().nth(i).unwrap()) {
                return false;
            };
            current_node = current_node.connections.get(&word.chars().nth(i).unwrap()).unwrap();
        }
        if current_node.end == true {
            true
        } else {
            false
        }
    }
}
