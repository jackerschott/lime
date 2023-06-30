use std::env;
use std::fs;

//token enum that represents the different types of tokens
#[derive(Debug)]
enum Token {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
}

// basic lexer that takes a string and returns a vector of tokens
fn lexer(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut iter = input.chars().peekable();

    while let Some(&c) = iter.peek() {
        match c {
            '0'..='9' => {
                let number = parse_number(&mut iter)?;
                tokens.push(Token::Number(number));
            }
            '+' => {
                tokens.push(Token::Plus);
                iter.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                iter.next();
            }
            '*' => {
                tokens.push(Token::Multiply);
                iter.next();
            }
            '/' => {
                tokens.push(Token::Divide);
                iter.next();
            }
            ' ' => {
                iter.next();
            }
            _ => {
                return Err(format!("Invalid character: {}", c));
            }
        }
    }

    Ok(tokens)

}

// basic parser that takes an iterator of chars and returns a number or an error
fn parse_number(iter: &mut std::iter::Peekable<std::str::Chars>) -> Result<i32, String> {
    let mut number = String::new();

    while let Some(&c) = iter.peek() {
        if c.is_digit(10) {
            number.push(c);
            iter.next();
        } else {
            break;
        }
    }

    number.parse::<i32>().map_err(|e| format!("Failed to parse number: {}", e))
}

// let's use a pest for parsing: http://pest.rs/
fn test_lexer() {
    let input: &str = "2 + 3 * 4 - 5 / 2";
    let tokens: Vec<Token> = lexer(input).unwrap();
    println!("{:?}", tokens);
}

struct Options {
    script_path : String
}

fn parse_args(args: std::env::Args) -> Options {
    let args: Vec<String> = args.collect();
    assert!(args.len() == 2);

    return Options {
        script_path: args[1].clone(),
    };
}

fn main() {
    // 1. call program as `lime <script>`
    let args = std::env::args();
    let options = parse_args(args);

    // 2. load script as lines

    // 3. parse whole script into some appropriate format (do this first completely,
    //      because its cheap; processing an image is expensive)

    // 4. execute parsing result
    // 4.1 when a function is called, extract function name, arguments
    //      and apply appropriate rust routine
    // 4.2 when a variable is assigned, evaluate right hand side expression and
    //      store result in some lookup table (definitely needed for layers as in
    //      e.g. Photoshop)
    // 4.3 ignore interactive calls for now
    // 4.4 opening and closing images are simply handled as functions
}
