use std::fs::File;
use std::io::Write;
use std::io::{Error, ErrorKind};
use std::process::{Command, Stdio};
use std::{env, io};

#[derive(Debug, Clone)]
enum Token {
    Blank,
    Redir(i32),
    Delim(bool),
    Normal(char),
}

fn token(x: Option<&char>) -> Token {
    match x {
        Some(&c) => match c {
            '|' => Token::Delim(true),
            '<' => Token::Redir(0),
            '>' => Token::Redir(1),
            ' ' | '\t' | '\n' | '\r' => Token::Blank,
            _ => Token::Normal(c),
        },
        None => Token::Delim(false),
    }
}

fn run<I>(mut it: std::iter::Peekable<I>, output: Stdio) -> Result<(), Error>
where
    I: Iterator<Item = char>,
{
    let mut is_pipe = false;
    let mut args = Vec::<String>::new();
    let mut io_in = Stdio::inherit();
    let mut io_out = output;
    while let Some(tok) = it.next() {
        match token(Some(&tok)) {
            Token::Delim(p) => {
                is_pipe = p;
                break;
            }
            Token::Normal(c) => {
                let mut word = String::new();
                word.push(c);
                while let Token::Normal(c) = token(it.peek()) {
                    it.next();
                    word.push(c);
                }
                args.push(word.chars().rev().collect::<String>());
            }
            Token::Redir(fd) => match args.pop() {
                Some(path) => match fd {
                    0 => io_in = Stdio::from(File::open(path)?),
                    1 => io_out = Stdio::from(File::create(path)?),
                    _ => return Err(Error::new(ErrorKind::Other, "bad redirection fd")),
                },
                None => {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "redirection filename expected",
                    ));
                }
            },
            Token::Blank => {}
        }
    }
    if args.is_empty() {
        return Ok(());
    }
    args.reverse();
    if args[0] == "cd" {
        let path = args
            .into_iter()
            .nth(1)
            .unwrap_or(env::var("HOME").unwrap_or("/".to_string()));
        env::set_current_dir(path)?;
        return Ok(());
    }
    if is_pipe {
        io_in = Stdio::piped()
    }
    let mut args_iter = args.iter();
    let pathname = args_iter.next().unwrap();
    let mut child = Command::new(pathname)
        .args(args_iter)
        .stdin(io_in)
        .stdout(io_out)
        .spawn()?;
    if is_pipe {
        run(it, Stdio::from(child.stdin.take().unwrap()))?;
    }
    child.wait()?;
    Ok(())
}

fn main() {
    loop {
        print!("$ ");
        let _ = io::stdout().flush(); // continue even if flush fails
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => return,
            Ok(_) => match run(input.chars().rev().peekable(), Stdio::inherit()) {
                Err(e) => println!("error: {}", e),
                _ => {}
            },
            Err(e) => {
                println!("io error: {}", e);
                return;
            }
        }
    }
}
