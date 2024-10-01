use dice_stats_lang::token;

fn main() {
    let input = "( ) { } + - * / . ! -1";
    let mut input_chars = input.chars().peekable();

    let mut current_line = 1;
    let mut current_column = 1;

    while input_chars.peek().is_some() {
        let token = token::read_token(&mut input_chars, &mut current_line, &mut current_column);
        match token {
            Ok(token) => {
                dbg!(token);
            }
            Err(token) => {
                dbg!(token);
            }
        }
    }
}
