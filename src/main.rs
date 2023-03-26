use atty::Stream;
use regex::Regex;
use std::env;
use std::io::{self, BufRead};
use std::process::exit;

const EMOJI_REGEX: &str = r"^\p{Extended_Pictographic}$";

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

fn is_emoji(input: &str) -> bool {
    let re = Regex::new(EMOJI_REGEX).unwrap();
    re.is_match(&input)
}

fn main() {
    if let Some(input) = get_input() {
        exit(!is_emoji(&input) as i32)
    }

    println!("Usage: is-emoji <arg>");
    exit(128);
}

#[test]
fn test_no_emoji() {
    assert_eq!(is_emoji(""), false);
    assert_eq!(is_emoji("a"), false);
    assert_eq!(is_emoji("hello"), false);
}

#[test]
fn test_ascii_emoji() {
    assert_eq!(is_emoji(":)"), false);
    assert_eq!(is_emoji(":-)"), false);
    assert_eq!(is_emoji(":P"), false);
}

#[test]
fn test_basic_emoji() {
    assert_eq!(is_emoji("😀"), true);
    assert_eq!(is_emoji("🥸"), true);
    assert_eq!(is_emoji("🔥"), true);
}
