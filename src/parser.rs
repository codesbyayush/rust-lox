use std::collections::VecDeque;
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
    let mut make_vec = VecDeque::from([number.to_owned()]);
    while characters.peek().is_some() {
        let token = next_token(characters);
        let value = parsed_value(token);
        match value {
            Ok(val) => {}
            Err(val) => {
                if !is_string_number(&val) {
                    if &val == "*" {
                        make_vec.push_front("(*".to_string());
                    } else if &val == "/" {
                        make_vec.push_front("(/".to_string());
                    } else {
                        break;
                    }
                } else {
                    make_vec.push_back(val.to_owned());
                }
            }
        }
    }
    let mut ans = make_vec.into_iter().collect::<Vec<String>>().join(" ");
    ans.push(')');
    return ans;
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
    if token.as_ref().is_err() && token.as_ref().err().unwrap() == "EOF" {
        return Err(String::from("END"));
    }
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

// Function to determine operator precedence
fn precedence(op: &str) -> i32 {
    match op {
        "+" | "-" => 1,
        "*" | "/" => 2,
        _ => 0,
    }
}

// Function to check if the character is an operator
fn is_operator(c: &str) -> bool {
    matches!(c, "+" | "-" | "*" | "/")
}

// Function to reverse and adjust parentheses in the string
fn reverse_and_adjust_parentheses(expr: &mut Vec<&str>) -> Vec<String> {
    expr.into_iter()
        .rev()
        .map(|c| {
            if c == &"(" {
                ")".to_owned()
            } else if c == &")" {
                "(".to_owned()
            } else {
                c.to_owned()
            }
        })
        .collect()
}

// Function to convert infix to prefix with grouping
fn infix_to_prefix(infix: &str) -> String {
    let mut operators = Vec::new();
    let mut result = VecDeque::new();

    // Reverse the infix expression and adjust parentheses
    let reversed_infix = reverse_and_adjust_parentheses(infix);

    // Iterate through the reversed expression
    for c in reversed_infix.chars() {
        if c.is_alphabetic() || c.is_digit(10) {
            // If it's an operand, add to the result
            result.push_front(c.to_string());
        } else if c == '(' {
            operators.push(c);
        } else if c == ')' {
            while *operators.last().unwrap() != '(' {
                // Pop operator and group operands
                let op = operators.pop().unwrap();
                let operand1 = result.pop_front().unwrap();
                let operand2 = result.pop_front().unwrap();
                let grouped = format!("({} {} {})", op, operand2, operand1);
                result.push_front(grouped);
            }
            operators.pop(); // Pop '('
        } else if is_operator(c) {
            // Pop operators with higher or equal precedence
            while !operators.is_empty() && precedence(*operators.last().unwrap()) > precedence(c) {
                let op = operators.pop().unwrap();
                let operand1 = result.pop_front().unwrap();
                let operand2 = result.pop_front().unwrap();
                let grouped = format!("({} {} {})", op, operand2, operand1);
                result.push_front(grouped);
            }
            operators.push(c);
        }
    }

    // Pop any remaining operators and group them
    while !operators.is_empty() {
        let op = operators.pop().unwrap();
        let operand1 = result.pop_front().unwrap();
        let operand2 = result.pop_front().unwrap();
        let grouped = format!("({} {} {})", op, operand2, operand1);
        result.push_front(grouped);
    }

    result.pop_front().unwrap()
}
