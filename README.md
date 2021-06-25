# is-emoji

A small CLI to check if a certain input is an emoji.

Detecting an emoji in bash is almost impossible without having a full set of emojis or using another language, like Python or JavaScript, which would result in slow methods.

## Usage

You can either give the input as an argument: `is-emoji 🥸`

Or pipe the input into it: `echo 🥸 | is-emoji`

Just like a lot of tools like this (like zsh's [`is-at-least` function](https://scriptingosx.com/2019/11/comparing-version-strings-in-zsh/)), this CLI uses exit codes to express its result.

It can exit with:

- `0` if the input is valid (a string containing only a single emoji)
- `1` if the input is invalid
- `128` if the usage is incorrect

Another possible exit code is `101`, which is the code Rust sets when a program panicked. But the program is short and defensive enough to make this exit code improbable.

## How it works?

It's just a small CLI written in Rust, which uses a regular expression with the `\p{Extended_Pictographic}` class.

## Why?

My only use of it is to detect a single emoji character in my clipboard and do an action with it ([the script is here](https://github.com/Drarig29/auto-paste-emoji)). So this tool only detects a string with a single character: an emoji.
