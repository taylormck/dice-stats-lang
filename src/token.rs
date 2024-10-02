use std::{iter::Peekable, str::Chars};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    Operator(OperatorTokenType),
    Literal(LiteralTokenType),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OperatorTokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Plus,
    Minus,
    Star,
    Slash,
    Dot,
    Bang,
    Eof,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LiteralTokenType {
    Int(i32),
    Die(bool),
    Unrecognized(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
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
                *current_column = 0;
                None
            }
            '(' => Some(Token::new(
                TokenType::Operator(OperatorTokenType::LeftParen),
                *current_line,
                *current_column,
            )),
            ')' => Some(Token::new(
                TokenType::Operator(OperatorTokenType::RightParen),
                *current_line,
                *current_column,
            )),
            '{' => Some(Token::new(
                TokenType::Operator(OperatorTokenType::LeftBrace),
                *current_line,
                *current_column,
            )),
            '}' => Some(Token::new(
                TokenType::Operator(OperatorTokenType::RightBrace),
                *current_line,
                *current_column,
            )),
            '+' => Some(Token::new(
                TokenType::Operator(OperatorTokenType::Plus),
                *current_line,
                *current_column,
            )),
            '-' => Some(Token::new(
                TokenType::Operator(OperatorTokenType::Minus),
                *current_line,
                *current_column,
            )),
            '*' => Some(Token::new(
                TokenType::Operator(OperatorTokenType::Star),
                *current_line,
                *current_column,
            )),
            '/' => Some(Token::new(
                TokenType::Operator(OperatorTokenType::Slash),
                *current_line,
                *current_column,
            )),
            '.' => Some(Token::new(
                TokenType::Operator(OperatorTokenType::Dot),
                *current_line,
                *current_column,
            )),
            '!' => Some(Token::new(
                TokenType::Operator(OperatorTokenType::Bang),
                *current_line,
                *current_column,
            )),
            first_digit if first_digit.is_ascii_digit() => {
                let starting_column = *current_column;

                let mut n = vec![first_digit];

                while input.peek().is_some() && input.peek().unwrap().is_ascii_digit() {
                    n.push(input.next().unwrap());
                    *current_column += 1;
                }

                let n = n.into_iter().collect::<String>();

                match n.parse() {
                    Ok(n) => Some(Token::new(
                        TokenType::Literal(LiteralTokenType::Int(n)),
                        *current_line,
                        starting_column,
                    )),
                    Err(_) => return Err(TokenError::InvalidNumberToken(n)),
                }
            }
            first_char if first_char.is_ascii_alphabetic() => {
                let starting_column = *current_column;

                let mut chars = vec![first_char];

                while input.peek().is_some() && is_keyword_character(input.peek().unwrap()) {
                    chars.push(input.next().unwrap());
                    *current_column += 1;
                }

                let literal = chars.into_iter().collect::<String>();

                match literal.as_str() {
                    "d" => Some(Token::new(
                        TokenType::Literal(LiteralTokenType::Die(false)),
                        *current_line,
                        starting_column,
                    )),
                    "die" => Some(Token::new(
                        TokenType::Literal(LiteralTokenType::Die(true)),
                        *current_line,
                        starting_column,
                    )),
                    _ => Some(Token::new(
                        TokenType::Literal(LiteralTokenType::Unrecognized(literal)),
                        *current_line,
                        starting_column,
                    )),
                }
            }
            c => return Err(TokenError::UnsupportedToken(String::from(c))),
        };

        *current_column += 1;

        if let Some(token) = token {
            return Ok(token);
        }
    }

    Ok(Token::new(
        TokenType::Operator(OperatorTokenType::Eof),
        *current_line,
        *current_column,
    ))
}

fn is_keyword_character(c: &char) -> bool {
    c.is_ascii_alphabetic() || *c == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_character_tokens() {
        let input = "( ) { }\n+ - * /\n. ! -420";
        let mut input = input.chars().peekable();

        let expected_tokens: Vec<Token> = vec![
            Token::new(TokenType::Operator(OperatorTokenType::LeftParen), 1, 1),
            Token::new(TokenType::Operator(OperatorTokenType::RightParen), 1, 3),
            Token::new(TokenType::Operator(OperatorTokenType::LeftBrace), 1, 5),
            Token::new(TokenType::Operator(OperatorTokenType::RightBrace), 1, 7),
            Token::new(TokenType::Operator(OperatorTokenType::Plus), 2, 1),
            Token::new(TokenType::Operator(OperatorTokenType::Minus), 2, 3),
            Token::new(TokenType::Operator(OperatorTokenType::Star), 2, 5),
            Token::new(TokenType::Operator(OperatorTokenType::Slash), 2, 7),
            Token::new(TokenType::Operator(OperatorTokenType::Dot), 3, 1),
            Token::new(TokenType::Operator(OperatorTokenType::Bang), 3, 3),
            Token::new(TokenType::Operator(OperatorTokenType::Minus), 3, 5),
            Token::new(TokenType::Literal(LiteralTokenType::Int(420)), 3, 6),
        ];

        let mut actual_tokens: Vec<Token> = vec![];

        let mut current_line = 1;
        let mut current_column = 1;

        while input.peek().is_some() {
            let token = read_token(&mut input, &mut current_line, &mut current_column).unwrap();
            actual_tokens.push(token);
        }

        assert_eq!(expected_tokens, actual_tokens);
    }

    #[test]
    fn test_integers() {
        let input = "1 10 1234 -420";
        let mut input = input.chars().peekable();

        let expected_tokens: Vec<Token> = vec![
            Token::new(TokenType::Literal(LiteralTokenType::Int(1)), 1, 1),
            Token::new(TokenType::Literal(LiteralTokenType::Int(10)), 1, 3),
            Token::new(TokenType::Literal(LiteralTokenType::Int(1234)), 1, 6),
            Token::new(TokenType::Operator(OperatorTokenType::Minus), 1, 11),
            Token::new(TokenType::Literal(LiteralTokenType::Int(420)), 1, 12),
        ];

        let mut actual_tokens: Vec<Token> = vec![];

        let mut current_line = 1;
        let mut current_column = 1;

        while input.peek().is_some() {
            let token = read_token(&mut input, &mut current_line, &mut current_column).unwrap();
            actual_tokens.push(token);
        }

        assert_eq!(expected_tokens, actual_tokens);
    }

    #[test]
    fn test_die() {
        let input = "die d 2d4";
        let mut input = input.chars().peekable();

        let expected_tokens: Vec<Token> = vec![
            Token::new(TokenType::Literal(LiteralTokenType::Die(true)), 1, 1),
            Token::new(TokenType::Literal(LiteralTokenType::Die(false)), 1, 5),
            Token::new(TokenType::Literal(LiteralTokenType::Int(2)), 1, 7),
            Token::new(TokenType::Literal(LiteralTokenType::Die(false)), 1, 8),
            Token::new(TokenType::Literal(LiteralTokenType::Int(4)), 1, 9),
        ];

        let mut actual_tokens: Vec<Token> = vec![];

        let mut current_line = 1;
        let mut current_column = 1;

        while input.peek().is_some() {
            let token = read_token(&mut input, &mut current_line, &mut current_column).unwrap();
            actual_tokens.push(token);
        }

        assert_eq!(expected_tokens, actual_tokens);
    }
}
