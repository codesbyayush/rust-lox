
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

            eprintln!("Logs from your program will appear here!");

            let file_contents = fs::read_to_string(filename).unwrap_or_else(|_| {
                eprintln!( "Failed to read file {}", filename);
                String::new()
            });
            tokenize(&file_contents);
        }
        _ => {
            eprintln!( "Unknown command: {}", command);
            return;
        }
    }
}

fn tokenize(file_contents: &str) {
    let mut exit_code = 0;
    let characters = file_contents.chars();
    let mut curr_line = 1;
    let mut last_word = ' ';
    let mut is_comment = false;
    let mut ongoing_string = false;
    let mut ongoing_number = false;
    let mut ongoing_identifier = false;
    let mut literal_start = 0;
    for (i, c) in characters.enumerate() {

        if is_comment {
            if c == '\n' {
                is_comment = false;
                curr_line += 1;
            }
            continue;
        }

        if last_word == '/' {
            last_word = ' ';
            if c == '/' {
                is_comment = true;
                continue;
            }
            println!("SLASH / null");
        }
        
        if ongoing_string {
            if c == '\n' {
                curr_line += 1;
            } else if c == '"' {
                let string_literal = &file_contents[literal_start..(i)];
                println!("STRING {}\" {}", &string_literal, &string_literal[1..]);
                ongoing_string = false;
            }
            continue;
        }
        if c == '"' {
            ongoing_string = true;
            literal_start = i;
            continue;
        }
        

        if (c.is_ascii_digit() && !ongoing_identifier) || (c == '.' && ongoing_number) {
            if !ongoing_number {
                literal_start = i;
                ongoing_number = true;
            }
            continue;
        }

        if ongoing_number {
            let number_literal = &file_contents[literal_start..i];
            
            let mut formatted_literal = if number_literal.contains('.') {
                number_literal.to_string()
            } else {
                format!("{}.0", number_literal)
            };
            loop {
                let last_char = formatted_literal.pop().unwrap();
                if last_char != '0' {
                    formatted_literal.push(last_char);
                    if last_char == '.' {
                        formatted_literal.push('0');
                    }
                    break;
                }
            }
            println!("NUMBER {} {}", number_literal, formatted_literal);
            ongoing_number = false;
        }


        if c.is_ascii_alphabetic() || c == '_' {
            if !ongoing_identifier {
                literal_start = i;
                ongoing_identifier = true;
            }
            continue;
        }

        if ongoing_identifier {
            if c.is_ascii_alphanumeric() {
                continue;
            }
            println!("IDENTIFIER {} null", &file_contents[literal_start..i]);
            ongoing_identifier = false;
        }


        
        if c != '=' && last_word != ' ' {
            match last_word {
                '=' => println!("EQUAL = null"),
                '!' => println!("BANG ! null"),
                '<' => println!("LESS < null"),
                '>' => println!("GREATER > null"),
                _ => {}
            }
            last_word = ' ';
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
                match last_word {
                    '=' => println!("EQUAL_EQUAL == null"),
                    '!' => println!("BANG_EQUAL != null"),
                    '<' => println!("LESS_EQUAL <= null"),
                    '>' => println!("GREATER_EQUAL >= null"),
                    _ => found = false
                }
                if !found { last_word = '=';}
                else { last_word = ' ';}
            },
            '!' =>    last_word = '!',
            '<' =>    last_word = '<',
            '>' =>    last_word = '>',
            '/' =>    last_word = '/',
            '\n' =>   curr_line += 1,
            '\t' | '\r' | ' ' =>   {},
            u => {
                eprintln!("[line {}] Error: Unexpected character: {}", curr_line, u);
                exit_code = 65;
            }
        };
    }
    match last_word {
        '=' => println!("EQUAL = null"),
        '!' => println!("BANG ! null"),
        '<' => println!("LESS < null"),
        '>' => println!("GREATER > null"),
        '/' => println!("SLASH / null"),
        _ => {}
    }

    if ongoing_string {
        eprintln!("[line {}] Error: Unterminated string.", curr_line);
        exit_code = 65;
    }

    if ongoing_number {
        let number_literal = &file_contents[literal_start..];
        let mut formatted_literal = if number_literal.contains('.') {
            number_literal.to_string()
        } else {
            format!("{}.0", number_literal)
        };
        loop {
            let last_char = formatted_literal.pop().unwrap();
            if last_char != '0' {
                formatted_literal.push(last_char);
                if last_char == '.' {
                    formatted_literal.push('0');
                }
                break;
            }
        }
        println!("NUMBER {} {}", number_literal, formatted_literal);
    }

    if ongoing_identifier {
        println!("IDENTIFIER {} null", &file_contents[literal_start..]);
    }
    
    println!("EOF  null");
    exit(exit_code);
}
