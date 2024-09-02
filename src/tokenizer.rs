use std::process::exit;

use crate::token::next_token;

pub fn tokenize(file_contents: &str) {
    let mut exit_code = 0;
    let mut characters = file_contents.chars().peekable();
    let mut curr_line = 1;
    loop {
        let token = next_token(&mut characters);
        match token {
            Ok((token_type, representation, something)) => {
                println!("{} {} {}", token_type, representation, something);
            }
            Err(some_err) => match &some_err[..] {
                "NEWLINE" => {
                    curr_line += 1;
                }
                "UNTERMINATED_STRING" => {
                    eprintln!("[line {}] Error: Unterminated string.", curr_line);
                    exit_code = 65;
                }
                "EOF" => {
                    println!("EOF  null");
                    break;
                }
                e => {
                    let u = &e[9..];
                    eprintln!("[line {}] Error: Unexpected character: {}", curr_line, u);
                    exit_code = 65;
                    // println!("Something unexpected happened!!!");
                }
            },
        }
    }
    exit(exit_code);
}
