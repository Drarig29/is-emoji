use regex::Regex;
use std::env;
use std::io::{self, BufRead};
use std::process::exit;

fn get_input() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        let stdin = io::stdin();
        let line = stdin.lock().lines().next().unwrap().unwrap();
        return Some(line);
    }

    Some(args[1].clone())
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
