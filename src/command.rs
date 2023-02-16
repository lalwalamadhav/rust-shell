pub struct command {
    command: String,
    args: Vec<String>,
}

impl command {
    fn new(input: &String) -> command {
        let mut dummy_string: String = String::from("");
        let mut command_string: String = String::from("");
        let mut args: Vec<String> = vec![];
        let mut done = false;
        for i in 0..input.len() + 1 {
            if i == input.len() || input.chars().nth(i).unwrap() == ' ' {
                if done == false {
                    command_string = dummy_string;
                    done = true;
                } else if dummy_string != String::from("") {
                    args.push(dummy_string);
                }
                dummy_string = String::from("");
            } else {
                dummy_string.push(input.chars().nth(i).unwrap());
            }
        }
        command {
            command: command_string,
            args: args,
        }
    }
}
