use std::io;
mod process_command;
mod highlight;

fn main() {
    let mut input: String = String::from("");
    let mut track_input: String = String::new();
    loop {
        let stdin = io::stdin()
	     .take(1)
	     .expect("Input failed!");
	stdin.read_line(&mut track_input);
	input.push_str(track_input);
        if stdin == "\n" {
            break;
	} else {
            highlight::highlighter(input);
	}
    }
}
