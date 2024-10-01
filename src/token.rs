use std::{iter::Peekable, str::Chars};

#[derive(Copy, Clone, Debug)]
pub enum TokenType {
    Int(i32),
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Plus,
    Minus,
    Star,
    Slash,
    Dot,
    Eof,
}

#[derive(Copy, Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(token_type: TokenType, line: usize, column: usize) -> Self {
        Self {
            token_type,
            line,
            column,
        }
    }
}

#[derive(Debug)]
pub enum TokenError {
    UnsupportedToken(String),
    InvalidNumberToken(String),
}

pub fn read_token(
    input: &mut Peekable<Chars>,
    current_line: &mut usize,
    current_column: &mut usize,
) -> Result<Token, TokenError> {
    while let Some(next_char) = input.next() {
        let token = match next_char {
            ' ' => None,
            '\n' => {
                *current_line += 1;
                *current_column = 1;
                None
            }
            '(' => Some(Token::new(
                TokenType::LeftParen,
                *current_line,
                *current_column,
            )),
            ')' => Some(Token::new(
                TokenType::RightParen,
                *current_line,
                *current_column,
            )),
            '{' => Some(Token::new(
                TokenType::LeftBrace,
                *current_line,
                *current_column,
            )),
            '}' => Some(Token::new(
                TokenType::RightBrace,
                *current_line,
                *current_column,
            )),
            '+' => Some(Token::new(TokenType::Plus, *current_line, *current_column)),
            '-' => Some(Token::new(TokenType::Minus, *current_line, *current_column)),
            '*' => Some(Token::new(TokenType::Star, *current_line, *current_column)),
            '/' => Some(Token::new(TokenType::Slash, *current_line, *current_column)),
            '.' => Some(Token::new(TokenType::Dot, *current_line, *current_column)),
            first_digit if first_digit.is_ascii_digit() => {
                *current_column += 1;
                let mut n = vec![first_digit];

                while input.peek().is_some() {
                    n.push(input.next().unwrap());
                    *current_column += 1;
                }

                let n = n.into_iter().collect::<String>();

                match n.parse() {
                    Ok(n) => Some(Token::new(TokenType::Int(n), *current_line, *current_line)),
                    Err(_) => return Err(TokenError::InvalidNumberToken(n)),
                }
            }
            c => return Err(TokenError::UnsupportedToken(String::from(c))),
        };

        *current_column += 1;

        if let Some(token) = token {
            return Ok(token);
        }
    }

    Ok(Token::new(TokenType::Eof, *current_line, *current_column))
}
