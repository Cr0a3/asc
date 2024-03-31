use crate::token::Token;

pub fn print_tokens(tokens: &Vec<Token>) {
    for token in tokens {
        println!("{:?}", token);
    }
}