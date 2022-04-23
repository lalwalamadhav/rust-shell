use colored::Colorize;

enum Input {
    ValidCommand(&String),
    InvalidCommand(&String),
    Arguments(&String),
    Quotes(&String),
}

impl Input {
    fn highlight(&self) {
        match self {
            Input::ValidCommand(k) => format!(k).blue(),
            Input::InvalidCommand(k) => format!(k).red(),
	    Input::Arguments(k) => format!(k).cyan(),
	    Input::Quotes(k) => format!(k).yellow(),
        }
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
	} for i in args.iter() {
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
