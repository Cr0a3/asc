use crate::token::Token;
use PrintLib::colorize::Colorize;

pub fn print_tokens(tokens: &Vec<Token>) {
    for token in tokens {
        let tokenstr = format!("{:?}", token);
        println!("{}  {}", "⬤".gray(), tokenstr.cyan().bold())
    }
}

pub fn print_unexpect_instruction_error(instr: &str) {
    println!(
        "{}{} unexpected instruction '{instr}'",
        "error".red().bold(),
        ":".bold()
    );
    println!(
        "{} | consider putting it into a function:",
        "+".green().bold()
    );
    println!("{} | {}",     "+".green().bold(), "function {".gray());
    println!("{} | \t{}",   "+".green().bold(), instr.gray());
    println!("{} | {}",     "+".green().bold(), "}".gray());
}