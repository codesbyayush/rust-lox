use std::collections::HashMap;
use std::env;
use std::fs;
use std::iter::Peekable;
use std::process::exit;
use std::str::Chars;

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

fn parse(file_contents: &str) {
    let mut exit_code = 0;
    let mut characters = file_contents.chars().peekable();
    let mut curr_line = 1;
    loop {
        let token = next_token(&mut characters);
        match token {
            Ok((_token_type, representation, _something)) => {
                println!("{}", representation);
            }
            Err(some_err) => match &some_err[..] {
                "NEWLINE" => {
                    curr_line += 1;
                }
                "UNTERMINATED_STRING" => {
                    // eprintln!("[line {}] Error: Unterminated string.", curr_line);
                    exit_code = 65;
                }
                "EOF" => {
                    // println!("EOF  null");
                    break;
                }
                e => {
                    let u = &e[9..];
                    // eprintln!("[line {}] Error: Unexpected character: {}", curr_line, u);
                    exit_code = 65;
                    // println!("Something unexpected happened!!!");
                }
            },
        }
    }
    exit(exit_code);
}

fn tokenize(file_contents: &str) {
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

fn next_token(token_vec: &mut Peekable<Chars>) -> Result<(String, String, String), String> {
    let c = token_vec.peek();
    if c.is_none() {
        return Err("EOF".to_string());
    }
    let c = c.unwrap();

    if c.is_ascii_digit() {
        return handle_numeral(token_vec);
    } else if c.is_ascii_alphabetic() || c == &'_' {
        return handle_identifier(token_vec);
    }

    let c = token_vec.next().unwrap();
    let next_char = token_vec.peek().unwrap_or(&' ').clone();

    match c {
        '(' => {
            return Ok((
                "LEFT_PAREN".to_string(),
                "(".to_string(),
                "null".to_string(),
            ))
        }
        '{' => {
            return Ok((
                "LEFT_BRACE".to_string(),
                "{".to_string(),
                "null".to_string(),
            ))
        }
        '*' => return Ok(("STAR".to_string(), "*".to_string(), "null".to_string())),
        '.' => return Ok(("DOT".to_string(), ".".to_string(), "null".to_string())),
        ',' => return Ok(("COMMA".to_string(), ",".to_string(), "null".to_string())),
        '+' => return Ok(("PLUS".to_string(), "+".to_string(), "null".to_string())),
        '-' => return Ok(("MINUS".to_string(), "-".to_string(), "null".to_string())),
        '}' => {
            return Ok((
                "RIGHT_BRACE".to_string(),
                "}".to_string(),
                "null".to_string(),
            ))
        }
        ')' => {
            return Ok((
                "RIGHT_PAREN".to_string(),
                ")".to_string(),
                "null".to_string(),
            ))
        }
        ';' => return Ok(("SEMICOLON".to_string(), ";".to_string(), "null".to_string())),
        '"' => return handle_string(token_vec),
        '=' => {
            if next_char == c {
                token_vec.next();
                return Ok((
                    "EQUAL_EQUAL".to_string(),
                    "==".to_string(),
                    "null".to_string(),
                ));
            }
            return Ok(("EQUAL".to_string(), "=".to_string(), "null".to_string()));
        }
        '!' => {
            if next_char == '=' {
                token_vec.next();
                return Ok((
                    "BANG_EQUAL".to_string(),
                    "!=".to_string(),
                    "null".to_string(),
                ));
            }
            return Ok(("BANG".to_string(), "!".to_string(), "null".to_string()));
        }
        '<' => {
            if next_char == '=' {
                token_vec.next();
                return Ok((
                    "LESS_EQUAL".to_string(),
                    "<=".to_string(),
                    "null".to_string(),
                ));
            }
            return Ok(("LESS".to_string(), "<".to_string(), "null".to_string()));
        }
        '>' => {
            if next_char == '=' {
                token_vec.next();
                return Ok((
                    "GREATER_EQUAL".to_string(),
                    ">=".to_string(),
                    "null".to_string(),
                ));
            }
            return Ok(("GREATER".to_string(), ">".to_string(), "null".to_string()));
        }
        '/' => {
            if next_char == '/' {
                while token_vec.peek().unwrap_or(&'\n').clone() != '\n' {
                    token_vec.next();
                }
                return next_token(token_vec);
            }
            return Ok(("SLASH".to_string(), "/".to_string(), "null".to_string()));
        }
        '\n' => return Err("NEWLINE".to_string()),
        '\t' | '\r' | ' ' => return next_token(token_vec),
        u => return Err(format!("NOTFOUND {}", u)),
    };
}

fn handle_string(token_vec: &mut Peekable<Chars>) -> Result<(String, String, String), String> {
    let mut make_string = String::new();
    let mut terminated = false;
    while let Some(curr_char) = token_vec.next() {
        if curr_char != '"' {
            make_string.push(curr_char);
        } else {
            terminated = true;
            break;
        }
    }
    if terminated {
        return Ok((
            "STRING".to_string(),
            format!("\"{}\"", make_string.clone()),
            make_string.clone(),
        ));
    };
    return Err("UNTERMINATED_STRING".to_string());
}

fn handle_numeral(token_vec: &mut Peekable<Chars>) -> Result<(String, String, String), String> {
    let mut make_number = String::new();
    while let Some(curr_char) = token_vec.peek() {
        if curr_char.is_ascii_digit() || curr_char == &'.' {
            make_number.push(token_vec.next().unwrap());
        } else {
            break;
        }
    }

    let mut formatted_literal = if make_number.contains('.') {
        make_number.to_string()
    } else {
        format!("{}.0", make_number)
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
    Ok((
        "NUMBER".to_string(),
        make_number.to_string(),
        formatted_literal.to_string(),
    ))
}

fn handle_identifier(token_vec: &mut Peekable<Chars>) -> Result<(String, String, String), String> {
    let keywords: HashMap<&str, &str> = HashMap::from([
        ("and", "AND"),
        ("class", "CLASS"),
        ("else", "ELSE"),
        ("false", "FALSE"),
        ("for", "FOR"),
        ("fun", "FUN"),
        ("if", "IF"),
        ("nil", "NIL"),
        ("or", "OR"),
        ("print", "PRINT"),
        ("return", "RETURN"),
        ("super", "SUPER"),
        ("this", "THIS"),
        ("true", "TRUE"),
        ("var", "VAR"),
        ("while", "WHILE"),
    ]);
    let mut make_identifier = String::new();
    while let Some(curr_char) = token_vec.peek() {
        if curr_char.is_ascii_alphanumeric() || curr_char == &'_' {
            make_identifier.push(token_vec.next().unwrap());
        } else {
            break;
        }
    }

    let identifier_type = keywords.get(&make_identifier[..]).unwrap_or(&"IDENTIFIER");
    Ok((
        identifier_type.to_string(),
        make_identifier.to_string(),
        "null".to_string(),
    ))
}

// fn tokenize(file_contents: &str) {
//     let keywords: HashMap<&str, &str> = HashMap::from([
//         ("and", "AND"),
//         ("class", "CLASS"),
//         ("else", "ELSE"),
//         ("false", "FALSE"),
//         ("for", "FOR"),
//         ("fun", "FUN"),
//         ("if", "IF"),
//         ("nil", "NIL"),
//         ("or", "OR"),
//         ("print", "PRINT"),
//         ("return", "RETURN"),
//         ("super", "SUPER"),
//         ("this", "THIS"),
//         ("true", "TRUE"),
//         ("var", "VAR"),
//         ("while", "WHILE"),
//     ]);
//     let mut exit_code = 0;
//     let mut characters = file_contents.chars().peekable();
//     let mut curr_line = 1;
//     let mut last_word = ' ';
//     let mut is_comment = false;
//     let mut ongoing_string = false;
//     let mut ongoing_number = false;
//     let mut ongoing_identifier = false;
//     let mut literal_start = 0;
//     for (i, c) in characters.enumerate() {
//         if is_comment {
//             if c == '\n' {
//                 is_comment = false;
//                 curr_line += 1;
//             }
//             continue;
//         }

//         if last_word == '/' {
//             last_word = ' ';
//             if c == '/' {
//                 is_comment = true;
//                 continue;
//             }
//             println!("SLASH / null");
//         }

//         if ongoing_string {
//             if c == '\n' {
//                 curr_line += 1;
//             } else if c == '"' {
//                 let string_literal = &file_contents[literal_start..(i)];
//                 println!("STRING {}\" {}", &string_literal, &string_literal[1..]);
//                 ongoing_string = false;
//             }
//             continue;
//         }
//         if c == '"' {
//             ongoing_string = true;
//             literal_start = i;
//             continue;
//         }

//         if (c.is_ascii_digit() && !ongoing_identifier) || (c == '.' && ongoing_number) {
//             if !ongoing_number {
//                 literal_start = i;
//                 ongoing_number = true;
//             }
//             continue;
//         }

//         if ongoing_number {
//             let number_literal = &file_contents[literal_start..i];

//             let mut formatted_literal = if number_literal.contains('.') {
//                 number_literal.to_string()
//             } else {
//                 format!("{}.0", number_literal)
//             };
//             loop {
//                 let last_char = formatted_literal.pop().unwrap();
//                 if last_char != '0' {
//                     formatted_literal.push(last_char);
//                     if last_char == '.' {
//                         formatted_literal.push('0');
//                     }
//                     break;
//                 }
//             }
//             println!("NUMBER {} {}", number_literal, formatted_literal);
//             ongoing_number = false;
//         }

//         if c.is_ascii_alphabetic() || c == '_' {
//             if !ongoing_identifier {
//                 literal_start = i;
//                 ongoing_identifier = true;
//             }
//             continue;
//         }

//         if ongoing_identifier {
//             if c.is_ascii_alphanumeric() {
//                 continue;
//             }
//             let identifier_type = keywords
//                 .get(&file_contents[literal_start..i])
//                 .unwrap_or(&"IDENTIFIER");
//             println!(
//                 "{} {} null",
//                 identifier_type,
//                 &file_contents[literal_start..i]
//             );
//             ongoing_identifier = false;
//         }

//         if c != '=' && last_word != ' ' {
//             match last_word {
//                 '=' => println!("EQUAL = null"),
//                 '!' => println!("BANG ! null"),
//                 '<' => println!("LESS < null"),
//                 '>' => println!("GREATER > null"),
//                 _ => {}
//             }
//             last_word = ' ';
//         }

//         match c {
//             '(' => println!("LEFT_PAREN ( null"),
//             '{' => println!("LEFT_BRACE {{ null"),
//             '*' => println!("STAR * null"),
//             '.' => println!("DOT . null"),
//             ',' => println!("COMMA , null"),
//             '+' => println!("PLUS + null"),
//             '-' => println!("MINUS - null"),
//             '}' => println!("RIGHT_BRACE }} null"),
//             ')' => println!("RIGHT_PAREN ) null"),
//             ';' => println!("SEMICOLON ; null"),
//             '=' => {
//                 let mut found = true;
//                 match last_word {
//                     '=' => println!("EQUAL_EQUAL == null"),
//                     '!' => println!("BANG_EQUAL != null"),
//                     '<' => println!("LESS_EQUAL <= null"),
//                     '>' => println!("GREATER_EQUAL >= null"),
//                     _ => found = false,
//                 }
//                 if !found {
//                     last_word = '=';
//                 } else {
//                     last_word = ' ';
//                 }
//             }
//             '!' => last_word = '!',
//             '<' => last_word = '<',
//             '>' => last_word = '>',
//             '/' => last_word = '/',
//             '\n' => curr_line += 1,
//             '\t' | '\r' | ' ' => {}
//             u => {
//                 eprintln!("[line {}] Error: Unexpected character: {}", curr_line, u);
//                 exit_code = 65;
//             }
//         };
//     }
//     match last_word {
//         '=' => println!("EQUAL = null"),
//         '!' => println!("BANG ! null"),
//         '<' => println!("LESS < null"),
//         '>' => println!("GREATER > null"),
//         '/' => println!("SLASH / null"),
//         _ => {}
//     }

//     if ongoing_string {
//         eprintln!("[line {}] Error: Unterminated string.", curr_line);
//         exit_code = 65;
//     }

//     if ongoing_number {
//         let number_literal = &file_contents[literal_start..];
//         let mut formatted_literal = if number_literal.contains('.') {
//             number_literal.to_string()
//         } else {
//             format!("{}.0", number_literal)
//         };
//         loop {
//             let last_char = formatted_literal.pop().unwrap();
//             if last_char != '0' {
//                 formatted_literal.push(last_char);
//                 if last_char == '.' {
//                     formatted_literal.push('0');
//                 }
//                 break;
//             }
//         }
//         println!("NUMBER {} {}", number_literal, formatted_literal);
//     }

//     if ongoing_identifier {
//         let identifier_type = keywords
//             .get(&file_contents[literal_start..])
//             .unwrap_or(&"IDENTIFIER");
//         println!(
//             "{} {} null",
//             identifier_type,
//             &file_contents[literal_start..]
//         );
//     }

//     println!("EOF  null");
//     exit(exit_code);
// }
