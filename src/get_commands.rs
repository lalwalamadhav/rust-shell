mod trie3;
use std::env;
use std::process::Command;
use trie3::Node;

pub fn construct_tree() -> Node {
    let mut Dictionary: Node = Node::new();
    let mut y: Vec<String> = vec![];
    let mut t: Vec<u8> = vec![];
    let x = env::var("PATH").unwrap();
    let mut k = 0;
    for i in 0..x.len() {
        if x.chars().nth(i).unwrap() == ':' {
            y.push(x[k..i].to_string());
            k = i + 1;
        }
    }
    for i in 0..y.len() {
        let mut a = Command::new(&String::from("ls"))
            .arg(&y[i])
            .output()
            .expect("Wrong directory on PATH");
        for j in 0..a.stdout.len() {
            if a.stdout[j] != 10 {
                t.push(a.stdout[j]);
            } else {
                Dictionary.add(&String::from_utf8(t).unwrap());
                t = vec![];
            }
        }
    }
    Dictionary
}
