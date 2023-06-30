#[derive(Debug)]
enum Token {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
}

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

fn main() {
    let input: &str = "2 + 3 * 4 - 5 / 2";
    let tokens: Vec<Token> = lexer(input).unwrap();
    println!("{:?}", tokens);
}