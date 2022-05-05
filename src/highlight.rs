use colored::Colorize;
use std::io::{self,Write};
mod process_command;

enum Input {
    ValidCommand(String),
    InvalidCommand(String),
    Arguments(String),
    Quotes(String),
}

impl Input {
    fn highlight(&self) {
        match self {
            Input::ValidCommand(k) => print!("{}",format!("{}", k).blue()),
            Input::InvalidCommand(k) => print!("{}",format!("{}", k).red()),
            Input::Arguments(k) => print!(" {}",format!("{}", k).cyan()),
            Input::Quotes(k) => print!("{}",format!("{}", k).yellow()),
        };
        io::stdout().flush().unwrap();
    }
}

pub fn highlighter(raw_input: &String, command_list: Vec<String>) {
    let mut arg: Vec<Input> = vec![];
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
    input.highlight();
    for i in arg.iter() {
        print!(" ");
	io::stdout().flush().unwrap();
        i.highlight();
    }
}
