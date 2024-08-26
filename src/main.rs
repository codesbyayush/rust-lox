use std::env;
use std::fs;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!( "Usage: {} tokenize <filename>", args[0]);
        return;
    }

    let command = &args[1];
    let filename = &args[2];

    match command.as_str() {
        "tokenize" => {
            // You can use print statements as follows for debugging, they'll be visible when running tests.
            eprintln!("Logs from your program will appear here!");

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!( "Failed to read file {}", filename);
                String::new()
            });

            // Uncomment this block to pass the first stage
            // if !file_contents.is_empty() {
            //     panic!("Scanner not implemented");
            // } else {
            //     println!("EOF  null"); // Placeholder, remove this line when implementing the scanner
            // }

            tokenize(&file_contents);
            // }
        }
        _ => {
            eprintln!( "Unknown command: {}", command);
            return;
        }
    }
}

fn tokenize(file_contents: &str) {
    let mut code = 0;
    let length = file_contents.len();
    let characters = file_contents.chars();
    let mut last = ' ';
    for (i, c) in characters.enumerate() {
        if last == '=' && c != '=' {
            println!("EQUAL = null");
            last = ' ';
        } else if last == '!' && c != '=' {
            println!("BANG ! null");
            last = ' ';
        }
        match c {
            '(' =>     println!("LEFT_PAREN ( null"),        
            '{' =>    println!("LEFT_BRACE {{ null"),
            '*' =>    println!("STAR * null"),
            '.' =>    println!("DOT . null"),
            ',' =>    println!("COMMA , null"),
            '+' =>    println!("PLUS + null"),
            '-' =>    println!("MINUS - null"),
            '}' =>    println!("RIGHT_BRACE }} null"),
            ')' =>    println!("RIGHT_PAREN ) null"),
            ';' =>    println!("SEMICOLON ; null"), 
            '=' =>    {
                match last {
                    '=' => {
                        println!("EQUAL_EQUAL == null");
                        last = ' ';
                    },
                    '!' => {
                        println!("BANG_EQUAL != null");
                        last = ' ';
                    },
                    _ => last = '='
                }
            },
            '!' =>    last = '!',
            u => {
                eprintln!("[line 1] Error: Unexpected character: {}", u);
                code = 65;
            }
        };
    }
    if last == '=' {
        println!("EQUAL = null");
    }
    println!("EOF  null");
    exit(code);
}
