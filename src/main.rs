use atty::Stream;
use regex::Regex;
use std::env;
use std::io::{self, BufRead};
use std::process::exit;

fn get_input() -> Option<String> {
    if atty::is(Stream::Stdin) {
        let args: Vec<String> = env::args().collect();
        if args.len() == 2 {
            return Some(args[1].clone());
        }
    } else {
        let stdin = io::stdin();
        let first_line = stdin.lock().lines().next()?;
        if let Ok(first_line) = first_line {
            return Some(first_line);
        }
    }

    None
}

fn main() {
    if let Some(input) = get_input() {
        let re = Regex::new(r"^\p{Extended_Pictographic}$").unwrap();
        let is_emoji = re.is_match(&input);
        exit(!is_emoji as i32)
    }

    println!("Usage: is-emoji <arg>");
    exit(128);
}
