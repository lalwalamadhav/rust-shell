use std::env;

enum Separator {
    Space(usize),
    Slash(usize),
    EnvVar(usize),
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
	}
    }
    if let Separator::Space(num) = sep_vec[0] {
        only_com.push_str(&command[..num]); 
    }
    let mut arg_new: String = String::from("");
    for i in 0..sep_vec.len() {
        match sep_vec[i] {
            Separator::EnvVar(j) => {
                if i != (sep_vec.len() - 1) {
                    match sep_vec[i + 1] {
                        Separator::Slash(k) => {
			    arg_new.push_str(&env::var(&command[(j + 1)..k]).expect("Environment variable not set!"));
                        },
			Separator::Space(k) => args.push(env::var(&command[(j + 1)..k]).expect("Environment variable not set!")),
                        Separator::EnvVar(_) => (),
		    };
		} else {
                    args.push(env::var(&command[(j + 1)..]).expect("Environment variable not set!"));
		}
	    },
	    Separator::Space(j) => {
                if i != (sep_vec.len() - 1) {
                    if let Separator::Space(k) = sep_vec[i + 1] {
                        args.push((&command[(j+1)..k]).to_string());
		    }
		} else {
                    args.push((&command[(j+1)..]).to_string());
		}
	    },
	    Separator::Slash(j) => {
                if i != (sep_vec.len() - 1) {
                    match sep_vec[i + 1] {
                        Separator::Slash(k) => {
			    arg_new.push_str("/");
                            arg_new.push_str(&command[(j+1)..k]);
			},
			Separator::Space(k) => {
			    arg_new.push_str(&command[j..k]);
                            args.push(String::from(&arg_new));
			},
                        Separator::EnvVar(_) => (),
		    }
		} else {
		    arg_new.push_str(&command[j..]);
		    args.push(String::from(&arg_new));
		}
	    },
	};
    }
    args.retain(|x| x != &String::from(""));
    (only_com,args)
}
