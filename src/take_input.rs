use crate::process_command;
use crate::highlight;
use std::{io::{self,Write},thread,time};
use termion::{self,input::TermRead,raw::IntoRawMode};

pub fn input_command() -> String {
    let mut stdout = io::stdout()
        .into_raw_mode()
	.unwrap();
    let mut stdin = termion::async_stdin().keys();
    let mut char_vec: Vec<char> = vec![];
    let mut r: u16 = 0;
    let commands = vec![String::from("This")];

    loop {
        let input = stdin.next();
	
	if let Some(Ok(key)) = input {
            match key {
		termion::event::Key::Left => {
		    if usize::from(r) > 0 {
                        write!(&mut stdout,"{}",termion::cursor::Goto(r-1 , 1)).unwrap();
		        stdout.flush().unwrap();
		        r -= 1;
	            } else {
                        continue;
		    }
		},
		termion::event::Key::Right => {
                    if usize::from(r) <= char_vec.len() {
                        write!(&mut stdout,"{}",termion::cursor::Goto(r+1 , 1)).unwrap();
			stdout.flush().unwrap();
			r += 1;
		    } else {
                        continue;
		    }
		},
                termion::event::Key::Backspace => {
		    if usize::from(r) >= char_vec.len() {
                        char_vec.pop();
		        write!(&mut stdout,"{}{}{}{}",termion::clear::All,termion::cursor::Goto(1,1),highlight::highlighter(&((&char_vec).into_iter().collect()),&commands),termion::cursor::Goto(r,1)).unwrap();
                        stdout.flush().unwrap();
		        r -= 1;

		    } else {
                        char_vec.remove(usize::from(r) - 1);
		        write!(&mut stdout,"{}{}{}{}",termion::clear::All,termion::cursor::Goto(1,1),highlight::highlighter(&((&char_vec).into_iter().collect()),&commands),termion::cursor::Goto(r,1)).unwrap();
                        stdout.flush().unwrap();
		        r -= 1;
		    }
		},
		termion::event::Key::Char('\n') => break,
		
		termion::event::Key::Char(k) => {
	            println!("{:?}",k);
		    if char_vec.len() > usize::from(r) {
                        char_vec.insert(usize::from(r) + 1,k);
                    } else {
                        char_vec.push(k);
		    }
		    r += 1;
		    write!(&mut stdout,"{}{}{}{}",termion::clear::All,termion::cursor::Goto(1,1),highlight::highlighter(&((&char_vec).into_iter().collect()),&commands),termion::cursor::Goto((r + 1),1)).unwrap();
		    stdout.flush().unwrap();
		},

                
		_ => (),
	    }
	}
	thread::sleep(time::Duration::from_millis(50));
    }
    char_vec.into_iter().collect()
}
