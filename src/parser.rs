use std::{iter::Peekable, str::Chars};

use crate::token::next_token;

pub fn parse(file_contents: &str) {
    // let mut exit_code = 0;
    let mut characters = file_contents.chars().peekable();
    while characters.peek().is_some() {
        let value = handle_parsing(&mut characters);
        match value {
            Ok(val) => println!("{}", val),
            Err(some_err) => match &some_err {
                _ => {}
            },
        }
    }
    // exit(exit_code);
}

fn handle_parsing(characters: &mut Peekable<Chars>) -> Result<String, String> {
    let token = next_token(characters);
    if token.as_ref().is_err() && token.as_ref().err().unwrap() == "EOF" {
        return Err(String::from("END"));
    }
    let value = parsed_value(token);
    match value {
        Ok(val) => return Ok(val),
        Err(some_err) => match &some_err[..] {
            "LEFT_PAREN" => {
                let value = handle_parenthesis(characters);
                return value;
            }
            "RIGHT_PAREN" => return Err(String::from("PAREN_END")),
            "BANG" => return handle_unary(characters, '!'),
            "MINUS" => return handle_unary(characters, '-'),
            "SEMICOLON" => return Err(some_err),
            u => {
                if is_string_number(u) {
                    return Ok(handle_arithemetics(characters, u));
                }
                return Err(String::from("END"));
            }
        },
    }
}

fn handle_unary(characters: &mut Peekable<Chars>, operation: char) -> Result<String, String> {
    let mut make_string = String::from("(");
    make_string.push(operation);
    match handle_parsing(characters) {
        Ok(val) => make_string.push_str(&format!(" {})", val)),
        Err(val) => match &val[..] {
            _ => return Err("SOME_ERROR_OCCURED".to_string()),
        },
    }
    return Ok(make_string);
}

fn handle_arithemetics(characters: &mut Peekable<Chars>, number: &str) -> String {
    let mut make_string = String::from(number);
    while characters.peek().is_some() {
        let value = handle_parsing(characters);
        match value {
            Ok(val) => {
                if is_string_number(&val) {
                    make_string.push_str(&([" ", &val, ")"].join("")));
                } else {
                    make_string = String::from(["(", &val, " ", &make_string].join(""));
                }
            }
            Err(_val) => {
                break;
            }
        }
    }
    return make_string.to_string();
}

fn handle_parenthesis(characters: &mut Peekable<Chars>) -> Result<String, String> {
    let mut make_string = String::from("(group");

    match handle_grouping(characters) {
        Ok(val) => make_string.push_str(&format!("{}", val)),
        Err(val) => match &val[..] {
            "NON_TERMINATED" => {
                return Err("NOT_POSSIBLE".to_owned());
            }
            _ => return Err("SOME_ERROR_OCCURED".to_string()),
        },
    }
    return Ok(make_string);
}

fn handle_grouping(characters: &mut Peekable<Chars>) -> Result<String, String> {
    let mut make_string = String::new();

    while characters.peek().is_some() {
        let value = handle_parsing(characters);
        match value {
            Ok(val) => make_string.push_str(&format!(" {}", val.to_owned())),
            Err(some_err) => match &some_err[..] {
                "PAREN_END" => {
                    make_string.push(')');
                    return Ok(make_string.to_owned());
                }
                _ => {}
            },
        }
    }
    return Err("NON_TERMINATED".to_string());
}

fn parsed_value(token: Result<(String, String, String), String>) -> Result<String, String> {
    match token {
        Ok((token_type, representation, value)) => match &token_type[..] {
            "LEFT_PAREN" => Err(token_type),
            "RIGHT_PAREN" => Err(token_type),
            "MINUS" => Err(token_type),
            "BANG" => Err(token_type),
            "IDENTIFIER" => Ok(representation),
            "NUMBER" => Err(value),
            "STRING" => Ok(value),
            _ => Ok(representation),
        },
        Err(some_err) => match &some_err[..] {
            _ => return Err(some_err),
        },
    }
}

fn is_string_number(s: &str) -> bool {
    s.parse::<f64>().is_ok()
}
