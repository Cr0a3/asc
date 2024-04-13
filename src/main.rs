use clap::Parser;
use print::print_tokens;
use std::path::PathBuf;
use PrintLib::colorize::Colorize;

pub mod codegen;
pub mod lexer;
pub mod print;
pub mod token;
pub mod util;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// File to compile
    filename: String,

    /// Sets the output file
    #[arg(short, long, value_name = "FILE")]
    out: Option<PathBuf>,

    /// Should additional information be printed
    #[arg(short, long, action)]
    verbose: bool,
}

struct Cmd {
    pub file: PathBuf,
    pub outfile: PathBuf,
}

fn main() {
    let cli = Cli::parse();

    let to_comp = cli.filename;
    let to_comp_stem = PathBuf::from(to_comp.clone())
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let cmd = Cmd {
        file: PathBuf::from(&to_comp),
        outfile: cli
            .out
            .unwrap_or(PathBuf::from(format!("{}.o", to_comp_stem))),
    };

    let mut lexer = lexer::Lexer::new();
    match lexer.scan(cmd.file) {
        Ok(_) => {}
        Err(e) => {
            println!("{} {}", "error:".red().bold(), e);
        }
    };

    if cli.verbose {
        print_tokens(&lexer.tokens);
    }

    let mut code_gen = codegen::CodeGen::new();
    code_gen.gen(lexer.tokens);
    match code_gen.build(cmd.outfile) {
        Ok(_) => {}
        Err(e) => {
            println!("{} {}", "error:".red().bold(), e);
        }
    };
}
