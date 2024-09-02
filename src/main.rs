use crate::parser::parse;
use crate::tokenizer::tokenize;
use std::env;
use std::fs;

mod parser;
mod token;
mod tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        eprintln!("Failed to read file {}", filename);
        String::new()
    });

    match command.as_str() {
        "tokenize" => {
            eprintln!("Logs from your program will appear here!");
            tokenize(&file_contents);
        }
        "parse" => {
            parse(&file_contents);
        }
        _ => {
            eprintln!("Unknown command: {}", command);
            return;
        }
    }
}
