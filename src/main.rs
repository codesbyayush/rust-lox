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
    let characters = file_contents.chars();
    let mut curr_line = 1;
    let mut last = ' ';
    for (_, c) in characters.enumerate() {
        if last == '/' && c == '/' {
            last = ' '; 
            break;
        } else if last == '/' {
            println!("SLASH / null");
            last = ' ';
        }
        if c != '=' && last != ' ' {
            let mut found = true;
            match last {
                '=' => println!("EQUAL = null"),
                '!' => println!("BANG ! null"),
                '<' => println!("LESS < null"),
                '>' => println!("GREATER > null"),
                _ => found = false
            }
            if !found { last = '=';}
            else { last = ' ';}
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
                let mut found = true;
                match last {
                    '=' => println!("EQUAL_EQUAL == null"),
                    '!' => println!("BANG_EQUAL != null"),
                    '<' => println!("LESS_EQUAL <= null"),
                    '>' => println!("GREATER_EQUAL >= null"),
                    _ => found = false
                }
                if !found { last = '=';}
                else { last = ' ';}
            },
            '!' =>    last = '!',
            '<' =>    last = '<',
            '>' =>    last = '>',
            '/' =>    last = '/',
            '\n' =>   curr_line += 1,
            '\t' | '\r' | ' ' =>   {},
            u => {
                eprintln!("[line {}] Error: Unexpected character: {}", curr_line, u);
                code = 65;
            }
        };
    }
    match last {
        '=' => println!("EQUAL = null"),
        '!' => println!("BANG ! null"),
        '<' => println!("LESS < null"),
        '>' => println!("GREATER > null"),
        '/' => println!("SLASH / null"),
        _ => {}
    }
    println!("EOF  null");
    exit(code);
}
