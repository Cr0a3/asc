use std::path::PathBuf;
use print::print_tokens;
use PrintLib::colorize::Colorize;
use clap::Parser;

pub mod token;
pub mod lexer;
pub mod codegen;
pub mod print;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// File to compile
    filename: String,

    /// Sets the output file
    #[arg(short, long, value_name = "FILE")]
    out: Option<PathBuf>,

}

struct Cmd {
    pub file: PathBuf,
    pub outfile: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let to_comp = cli.filename;
    let to_comp_stem = PathBuf::from(to_comp.clone()).file_stem().unwrap().to_str().unwrap().to_string();

    let cmd = Cmd {
        file: PathBuf::from(&to_comp),
        outfile: cli.out.unwrap_or(
            PathBuf::from(format!("{}.o", to_comp_stem))
        ),
    };

    let mut lexer = lexer::Lexer::new();
    match lexer.scan(cmd.file) {
        Ok(_) => {},
        Err(e) => {
            println!("{} {}", "error:".red().bold(), e);
        },
    };

    print_tokens(&lexer.tokens);
    println!("tokens: {}", lexer.tokens.len());

    let mut code_gen = codegen::CodeGen::new();
    match code_gen.gen(cmd.outfile) {
        Ok(_) => {},
        Err(e) => {
            println!("{} {}", "error:".red().bold(), e);
        },
    };
}