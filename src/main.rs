use regex::Regex;
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    let re = Regex::new(r"^\p{Extended_Pictographic}$").unwrap();

    if args.len() != 2 {
        println!("Usage: is-emoji <arg>");
        exit(128);
    }

    let is_emoji = re.is_match(&args[1]);
    if !is_emoji {
        exit(1);
    }
}
