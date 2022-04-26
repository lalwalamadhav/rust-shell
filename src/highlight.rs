use colored::Colorize;
use std::io;
mod process_command

enum Input {
    ValidCommand(str),
    InvalidCommand(str),
    Arguments(str),
    Quotes(str),
}

impl Input {
    fn highlight(&self) {
        match self {
            Input::ValidCommand(k) => print!("{}",format!("{}", k).expect("Error Highlighting").blue()),
            Input::InvalidCommand(k) => print!("{}",format!("{}", k).expect("Error Highlighting").red()),
            Input::Arguments(k) => print!("{}",format!("{}", k).expect("Error Highlighting").cyan()),
            Input::Quotes(k) => print!("{}",format!("{}", k).expect("Error Highlighting").yellow()),
        };
        io::stdout().flush().unwrap();
    }
}

pub fn highlighter(stdin: &String) {
    let input: Input;
    let arg: Vec<Input> = vec![];
    for line in stdin.lock().lines() {
        if line == "\n" {
            break;
        } else {
            let (command, args) = process_command::com(&line);
            if command_list.contains(&command) {
                let input = Input::ValidCommand(&String::from(&command));
            } else {
                let input = Input::ValidCommand(&String::from(&command));
            }
        }
        for i in args.iter() {
            if &i[0].as_bytes() == b'"' {
                arg.push(Input::Arguments(i));
            } else {
                arg.push(Input::Quotes(i));
            }
        }
    }
    input.highlight();
    for i in arg.iter() {
        i.highlight();
    }
}
