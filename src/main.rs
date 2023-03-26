use atty::Stream;
use regex::Regex;
use std::env;
use std::io::{self, BufRead};
use std::process::exit;

// Note: https://unicode.org/reports/tr51/#Emoji_Properties

// Taken from https://unicode.org/reports/tr51/#EBNF_and_Regex and adapted
// to be future-proof with \p{Extended_Pictographic} instead of \p{Emoji}
const EMOJI_REGEX: &str = r"^\p{Extended_Pictographic}(\p{EMod}|\x{FE0F}\x{20E3}?|[\x{E0020}-\x{E007E}]+\x{E007F})?(\x{200D}(\p{RI}\p{RI}|\p{Extended_Pictographic}(\p{EMod}|\x{FE0F}\x{20E3}?|[\x{E0020}-\x{E007E}]+\x{E007F})?))*$";

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

#[test]
fn test_emoji_component() {
    assert_eq!(is_emoji("🦲"), true); // bold
    assert_eq!(is_emoji("🦱"), true); // curly hair
    assert_eq!(is_emoji("🦳"), true); // white hair
}

#[test]
fn test_compound_emoji() {
    // https://unicode.org/reports/tr51/#def_emoji_zwj_sequence
    assert_eq!(is_emoji("👨‍🚀"), true); // 👨 + ZWJ + 🚀
    assert_eq!(is_emoji("❤‍🔥"), true); // ❤ + ZWJ + 🔥
    assert_eq!(is_emoji("👨‍🦱"), true); // 👨 + ZWJ + 🦱

    // https://unicode.org/reports/tr51/#Emoji_Variation_Sequences
    assert_eq!(is_emoji("👁‍🗨"), true); // 👁️ + VS16 + ZWJ + 💬 + VS16
}

#[test]
fn test_unicode_versions() {
    // Unicode 12 [2019]
    assert_eq!(is_emoji("🪑"), true);

    // Unicode 13 [2020]
    assert_eq!(is_emoji("🤌"), true);

    // Unicode 14 [2021]
    assert_eq!(is_emoji("🥹"), true);
    assert_eq!(is_emoji("🫡"), true);

    // Unicode 15 [2022] (not supported everywhere)
    assert_eq!(is_emoji("🫨"), true); // shaking face (https://www.emojiall.com/en/code/1FAE8)
}
