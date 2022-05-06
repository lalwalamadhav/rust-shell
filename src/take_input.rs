mod process_command;
mod highlight;
use std::{io::{self,Write},thread,time};
use termion::{self,input::TermRead,raw::IntoRawMode};

fn input_command() -> String {
    let mut stdout = io::stdout()
        .into_raw_mode()
	.unwrap();
    let mut stdin = termion::async_stdin().keys();
    let mut r = 3;

    loop {
        let input = stdin.next();
	
	if let Some(Ok(key)) = input {
            match key {
                termion::event::Key::Backspace => {
                    char_vec.remove(r);
		    write!("{}{}",highlight::highlighter(char_vec.into_iter().collect()),termion::cursor::Goto((r - 1),1);
		},
		termion::event::Key::Left => write!("{}",termion::cursor::Goto(r-1, 1),
		termion::event::Key::Right => {
                    if r < char_vec.len() {
                        write!("{}",termion::cursor::Goto(r + 1, 1);
		    }
		},
		termion::event::Key::Char('\n') => break;
		termion::event::Key::Char(k) => {
                    char_vec.insert(r,k);
		    write!("{}{}",highlight::highlighter(char_vec.into_iter().collect()),termion::cursor::Goto((r + 1),1);
		}
		_ => (),
	    }
	}
    }
    char_vec.into_iter().collect()
}
