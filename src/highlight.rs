use colored::{Colorize,ColoredString};
use std::io::{self,Write};
use crate::process_command;

enum Input {
    ValidCommand(String),
    InvalidCommand(String),
    Arguments(String),
    Quotes(String),
}

impl Input {
    fn highlight(&self) -> ColoredString {
        match self {
            Input::ValidCommand(k) => format!("{}", k).blue(),
            Input::InvalidCommand(k) => format!("{}", k).red(),
            Input::Arguments(k) => format!("{}", k).cyan(),
            Input::Quotes(k) => format!("{}", k).yellow(),
        }
    }
}

pub fn highlighter(raw_input: &String, command_list: &Vec<String>) -> String {
    let mut arg: Vec<Input> = vec![];
    let mut return_string: String = String::from("");
    let (command, args) = process_command::comm(&raw_input);
    let input: Input;
    if command_list.contains(&command) {
        input = Input::ValidCommand(String::from(&command));
    } else {
        input = Input::InvalidCommand(String::from(&command));
    }
    for i in args.iter() {
        if i.as_bytes()[0] == b'"' {
            arg.push(Input::Arguments(i.to_string()));
        } else {
            arg.push(Input::Quotes(i.to_string()));
        }
    }
    return_string = format!("{}{}",return_string,input.highlight());
    for i in arg.iter() {
        return_string = format!("{} ",return_string);
        return_string = format!("{}{}",return_string,i.highlight());
    }
    return_string
}
