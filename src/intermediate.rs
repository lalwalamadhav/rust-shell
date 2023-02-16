mod trie3;
mod command;
use command;
use colored::{Colorize, ColoredString};
use trie3::Node;

pub fn coloredString(input_vec: Vec<char>, dictionary: Node) -> String {
    let mut input_string = String::from("");
    let mut spaces: Vec<usize> = vec![];
    let mut args: Vec<ColoredString> = vec![];
    for i in 0..input_vec.len() {
        if input_vec[i] == ' ' {
            spaces.push(i);
        }
        input_string.push(input_vec[i]);
    }
    let sep = command::new(input_string);
    let mut command_string: ColoredString;
    let mut new_string: String;
    if sep.command.len() != 0 {
        if dictionary.search(&sep.command) {
            command_string = format!("{}", sep.command).blue();
        } else {
            command_string = format!("{}", sep.command).red();
        }
        for i in 0..sep.args.len() {
            args.push(format!("{}", sep.args[i]).cyan())
        }
        let mut index: usize = 0;
        let mut index2: usize = -1;
        while new_string.len() < input_vec.len() {
            if new_string.len() == spaces[index] {
                new_string.push(' ');
                index += 1;
            } else {
                if index2 == -1 {
                    new_string = new_string + &command_string;
                } else {
                    new_string = new_string + &args[index2];
                }
                index2 += 1;
            }
        }
    }
    return new_string;
}
