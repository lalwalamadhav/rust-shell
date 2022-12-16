use colored::Colorize;
use std::env;
use std::io::{self, Write};
use std::process::Command;
mod process_command;
mod highlight;
mod take_input;

fn main() {
    loop {
        let add = Command::new("pwd")
            .output()
            .expect("Getting the path failed");
        let prompt: &str = "-> ";
        print!(
            "\n{}",
            format!(
                "{}{}",
                String::from_utf8(add.stdout).expect("invalid"),
                &prompt
            )
            .red()
        );
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Command input failed");
        let command = String::from(command.trim());
        let (command, args) = process_command::comm(&command);

        if command == String::from("exit") {
            break;
        } else if command == String::from("") {
            continue;
        } else if command == String::from("cd") {
            if args.len() == 1 {
                match env::set_current_dir(&args[0]).is_ok() {
                    true => continue,
                    false => {
                        println!("Incorrect path!");
                        continue;
                    }
                };
            } else if args.len() == 0 {
                assert!(env::set_current_dir(
                    env::var("HOME").expect("HOME environment variable not set!")
                )
                .is_ok());
            } else {
                println!("Expected only one argument, more than one given!");
            }
        } else if command == String::from("setvar"){
            if args.len() < 2 {
                println!("Error: Expected more arguments!");
            } else if args.len() == 2 {
                env::set_var(&args[0], &args[1]);
            }
        } else {
            let mut a = match Command::new(&command).args(&args).spawn() {
                Ok(k) => k,
                Err(_) => {
                    println!("Command not found!");
                    continue;
                }
            };

            match a.wait().is_ok() {
                true => continue,
                false => {
                    println!("Command execution failed!");
                    continue;
                }
            };
        }
    }
}
