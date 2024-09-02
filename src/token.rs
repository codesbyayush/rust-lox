use std::collections::HashMap;
use std::{iter::Peekable, str::Chars};

pub fn next_token(token_vec: &mut Peekable<Chars>) -> Result<(String, String, String), String> {
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

pub fn handle_string(token_vec: &mut Peekable<Chars>) -> Result<(String, String, String), String> {
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

pub fn handle_numeral(token_vec: &mut Peekable<Chars>) -> Result<(String, String, String), String> {
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

pub fn handle_identifier(
    token_vec: &mut Peekable<Chars>,
) -> Result<(String, String, String), String> {
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
