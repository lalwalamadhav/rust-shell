struct command {
    command: String,
    args: Vec<String>,
}

impl command {
    fn new(input: &String) -> command {
        let mut dummy_string: String = String::from("");
        let command_string: String = String::from("");
        let mut args: Vec<String> = vec![];
        for i in 0..input.len() {
            if input[i] == ' ' {
                if command_string == String::from("") {
                    command_string = dummy_string;
                } else if dummy_string != String::from("") {
                    args.push(command_string);
                }
                dummy_string = String::from("");
            } else {
                dummy_string += input[i];
            }
        }
        command {command: command_string, args: args}
    }
}
