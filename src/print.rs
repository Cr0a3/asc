use crate::token::Token;
use PrintLib::colorize::Colorize;

pub fn print_tokens(tokens: &Vec<Token>) {
    for token in tokens {
        let tokenstr = format!("{:?}", token);
        println!("{}  {}", "⬤".gray(), tokenstr.cyan().bold())
    }
}