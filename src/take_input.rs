use std::io;
mod highlight;

pub fn take_input() {
    let mut input: String = String::from("");
    loop {
        let stdin = io::stdin().take(1).expect("Input failed!");
	input.push_str(stdin);
        if stdin == "\n" {
            break;
	} else {
            highlight::highlighter(input);
	}
    }
}
