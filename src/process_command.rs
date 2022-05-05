use std::env;

enum Separator {
    Space(usize),
    Slash(usize),
    EnvVar(usize),
    Quote(usize),
}

pub fn comm(command: &String) -> (String, Vec<String>) {
    let mut sep_vec: Vec<Separator> = vec![];
    let mut args: Vec<String> = vec![];
    let command_bytes = command.trim().as_bytes();
    let mut only_com: String = String::from("");
    for (k, &i) in command_bytes.iter().enumerate() {
        if i == b'$' {
            sep_vec.push(Separator::EnvVar(k));
        } else if i == b' ' {
            sep_vec.push(Separator::Space(k));
        } else if i == b'/' {
            sep_vec.push(Separator::Slash(k));
        } else if i == b'"' {
            sep_vec.push(Separator::Quote(k));
	}
    }
    if sep_vec.len() > 0 {
        if let Separator::Space(num) = sep_vec[0] {
            only_com.push_str(&command[..num]);
        }
        let mut arg_new: String = String::from("");
	let mut quote_open: bool = false;
	let mut quote_begin: usize = 0;
        for i in 0..sep_vec.len() {
	    if let Separator::Quote(_) = sep_vec[i] {
	    } else {
               if quote_open {
                   continue;
	       }
	    }
            match sep_vec[i] {
                Separator::EnvVar(j) => {
                    if i != (sep_vec.len() - 1) {
                        match sep_vec[i + 1] {
                            Separator::Slash(k) => {
                                arg_new.push_str(
                                    &env::var(&command[(j + 1)..k])
                                        .expect("Environment variable not set!"),
                                );
                            }
                            Separator::Space(k) => args.push(
                                env::var(&command[(j + 1)..k]).expect("Environment variable not set!"),
                            ),
                            Separator::EnvVar(_) => (),
			    Separator::Quote(_) => (),
                        };
                    } else {
                        args.push(
                            env::var(&command[(j + 1)..]).expect("Environment variable not set!"),
                        );
                    }
                }
                Separator::Space(j) => {
                    if i != (sep_vec.len() - 1) {
                        match sep_vec[i + 1] {
                            Separator::Space(k) => args.push((&command[(j + 1)..k]).to_string()),
			    Separator::Slash(k) => arg_new.push_str(&command[(j + 1)..k]),
			    Separator::EnvVar(_) => (),
			    Separator::Quote(_) => (),
                        };
                    } else {
                        args.push((&command[(j + 1)..]).to_string());
                    }
                }
                Separator::Slash(j) => {
                    if i != (sep_vec.len() - 1) {
                        match sep_vec[i + 1] {
                            Separator::Slash(k) => {
                                arg_new.push_str("/");
                                arg_new.push_str(&command[(j + 1)..k]);
                            }
                            Separator::Space(k) => {
                                arg_new.push_str(&command[j..k]);
                                args.push(String::from(&arg_new));
                            }
                            Separator::EnvVar(_) => (),
			    Separator::Quote(_) => (),
                        }
                    } else {
                        arg_new.push_str(&command[j..]);
                        args.push(String::from(&arg_new));
                    }
                }
		Separator::Quote(j) => {
                    quote_open = !quote_open;
		    if !quote_open {
                        args.push(String::from(&command[(quote_begin + 1)..j]));
		    } else {
                        quote_begin = j;
		    }
		},
            };
        }
    } else {
        only_com.push_str(command);
    }
    args.retain(|x| x != &String::from(""));
    (only_com, args)
}
