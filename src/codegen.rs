use std::{collections::VecDeque, path::PathBuf};
use CodeGenLib::{AdressManager, Builder, Function};
use PrintLib::colorize::Colorize;
use crate::token::Token;

pub struct CodeGen {
    builder: Builder,
    tokens: VecDeque<Token>,

    func_scope: bool,
    funct: Function,
}

impl CodeGen {
    pub fn new() -> Self {
        let builder = Builder::new();
        let func = Function::new("null", &mut {let adrmng = AdressManager::new((0, 0)); adrmng});

        Self {
            builder: builder,
            tokens: VecDeque::new(),

            func_scope: false,
            funct: func,
        }
    }

    fn advance(&mut self) -> Token {
        self.tokens.pop_front().unwrap_or(Token::EOF)
    }

    fn peek(&mut self) -> Token {
        let ret = self.tokens.pop_front().unwrap_or(Token::EOF);
        self.tokens.push_front(ret.clone());
        ret
    }

    fn scan(&mut self) {
        match self.advance() {
            Token::RET => {
                if self.func_scope {
                    self.funct.asm_ret();
                } else {
                    println!("{}{} unexpected instruction 'ret'", "error".red().bold(), ":".bold());
                    println!("{} | consider putting it into a function:", "+".green().bold());
                    println!("{} | {}", "+".green().bold(), "function {".gray());
                    println!("{} | {}", "+".green().bold(), "\tret".gray());
                    println!("{} | {}", "+".green().bold(), "}".gray());
                }
            },
            Token::IDENT(x) => {
                if !self.func_scope {
                    let peek = self.peek();
                    println!("self.peek() => {:#?}", peek);
                    if peek == Token::LBracket {
                        self.funct = self.builder.add_function(x.clone().as_str()).to_owned();
                        self.func_scope = true;
                    } else {
                        println!("{}{} unexpected token '{:#?}' after function", "error".red().bold(), ":".bold(), peek);
                        match peek {
                            Token::IDENT(x) => println!("{} | {x}{}", "+".green().bold(), "{".gray()),
                            _ => {}
                        }
                    }
                } else {
                    println!("{}{} unexpected identifer '{}'", "error".red().bold(), ":".bold(), x);
                }
            }

            Token::RBracket => {
                if self.func_scope {
                    self.func_scope = false;
                } else {
                    println!("{}: unexpected '}}'", "error".red().bold());
                }
            }

            _ => {}
        }
    }

    pub fn gen(&mut self, tokens: Vec<Token> ) {
        self.tokens = tokens.into();

        while self.tokens.len() > 0 {
            self.scan()
        }
    }

    pub fn build(&mut self, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        self.builder.build(format!("{}", path.display()).as_str(), CodeGenLib::BinaryFormat::Coff)
    }
}