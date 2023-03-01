mod intermediate;
mod command;
mod trie3;
use crate::highlight;
use intermediate::coloredString;
use std::{
    io::{self, Write},
    thread, time,
};
use command::command;
use termion::{self, input::TermRead, raw::IntoRawMode};
use trie3::Node;
use intermediate::coloredString;
use std::convert::TryInto;

pub fn input_command(dictionary: Node) -> command {
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut stdin = termion::async_stdin().keys();
    let mut char_vec: Vec<char> = vec![];
    let mut r: usize = 0;
    let commands = vec![];
    loop {
        let input = stdin.next();
        if let Some(Ok(key)) = input {
            match key {
                termion::event::Key::Left => {
                    if usize::from(r) > 0 {
                        write!(&mut stdout, "{}", termion::cursor::Left(1)).unwrap();
                        stdout.flush().unwrap();
                        r -= 1;
                    } else {
                        continue;
                    }
                }
                termion::event::Key::Right => {
                    if usize::from(r) < char_vec.len() {
                        write!(&mut stdout, "{}", termion::cursor::Right(1)).unwrap();
                        stdout.flush().unwrap();
                        r += 1;
                    } else {
                        continue;
                    }
                }
                termion::event::Key::Backspace => {
                    if usize::from(r) >= 1 {
                        char_vec.remove(usize::from(r));
                        write!(
                            &mut stdout,
                            "{}{}{}{}",
                            termion::cursor::Left(r.try_into().unwrap()),
                            termion::clear::AfterCursor,
                            coloredString(char_vec, dictionary),
                            termion::cursor::Right((r - 1).try_into().unwrap());
                        )
                        .unwrap();
                        stdout.flush().unwrap();
                        r -= 1;
                    } else if usize::from(r) == 0 {
                        continue;
                    }
                }
                termion::event::Key::Char('\n') => break,

                termion::event::Key::Char(k) => {
                    println!("{:?}", k);
                    if char_vec.len() > usize::from(r) {
                        char_vec.insert(usize::from(r) + 1, k);
                    } else {
                        char_vec.push(k);
                    }
                    r += 1;
                    write!(
                        &mut stdout,
                        "{}{}{}{}",
                        termion::cursor::Left((r - 1).try_into().unwrap()),
                        termion::clear::AfterCursor,
                        coloredString(char_vec, dictionary),
                        termion::cursor::Right(r.try_into().unwrap())
                    )
                    .unwrap();
                    stdout.flush().unwrap();
                }

                _ => (),
            }
        }
        thread::sleep(time::Duration::from_millis(50));
    }
    return command::new(char_vec.into_iter().collect());
}
