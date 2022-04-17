use colored::Colorize;
use std::env;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    loop {
        let add = Command::new("pwd")
            .output()
            .expect("Getting the path failed");
        let prompt: &str = "|> ";
        print!(
            "\n{}",
            format!(
                "{}{}",
                String::from_utf8(add.stdout).expect("invalid"),
                &prompt
            )
            .blue()
        );
        io::stdout().flush().unwrap();
        let mut command = String::new();
        let mut processed_command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("Command input failed");
        let command = command.trim();
        let mut space: Vec<usize> = vec![];
        let mut args: Vec<&str> = vec![];
        let mut env_vars: Vec<usize> = vec![];
        let command_bytes = command.as_bytes();
        for (k, &i) in command_bytes.iter().enumerate() {
            if i == b' ' {
                space.push(k);
            } else if i == b'$' {
                env_vars.push(k);
            }
        }
        let command = if env_vars.len() > 0 && space.len() > 0 {
            processed_command.push_str(&command[..space[0]]);
            processed_command.push_str(" ");
            for i in 0..space.len() {
                if i != (space.len() - 1) {
                    if env_vars.contains(&(&space[i] + 1)) {
                        processed_command.push_str(
                            &env::var(&command[(space[i] + 2)..space[i + 1]].trim())
                                .expect("Environment variable not found!"),
                        );
                        processed_command.push_str(" ");
                    } else {
                        processed_command.push_str(&command[(space[i] + 1)..space[i]].trim());
                        processed_command.push_str(" ");
                    }
                } else {
                    if env_vars.contains(&(&space[i] + 1)) {
                        processed_command.push_str(
                            &env::var(&command[(space[i] + 2)..].trim())
                                .expect("Environment variable not found!"),
                        );
                        processed_command.push_str(" ");
                    } else {
                        processed_command.push_str(&command[(space[i] + 1)..].trim());
                        processed_command.push_str(" ");
                    }
                }
            }
            &processed_command
        } else {
            command
        };
        if command == String::from("exit") {
            break;
        } else if command == String::from("") {
            continue;
        } else if command.len() == 1 {
            println!("Command not found!");
            continue;
        } else if &command[..2] == String::from("cd") {
            if space.len() != 0 {
                match env::set_current_dir(&command[(space[0] + 1)..].trim()).is_ok() {
                    true => continue,
                    false => {
                        println!("Incorrect path!");
                        continue;
                    }
                };
            } else {
                assert!(env::set_current_dir(
                    env::var("HOME").expect("HOME environment variable not set!")
                )
                .is_ok());
            }
        } else {
            for i in 0..space.len() {
                if i != (space.len() - 1) {
                    if space[i + 1] > (space[i] + 1) {
                        args.push(&command[(space[i] + 1)..space[i + 1]].trim());
                    }
                } else {
                    args.push(&command[(space[i] + 1)..].trim());
                }
            }
            let new_com = if args.len() != 0 {
                &command[..space[0]]
            } else {
                &command[..]
            };
            let mut a = match Command::new(&new_com[..]).args(&args).spawn() {
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
