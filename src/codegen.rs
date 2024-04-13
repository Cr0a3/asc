use crate::{token::Token, util::{is_reg, to_reg}};
use std::{
    collections::{HashMap, VecDeque},
    path::PathBuf,
};
use CodeGenLib::{asm::AsmInstructionEnum, BinFormat, Builder, IR::*};
use PrintLib::colorize::Colorize;

pub struct CodeGen {
    obj: Builder,
    tokens: VecDeque<Token>,
    funct: HashMap<String, Vec<AsmInstructionEnum>>,
}

impl CodeGen {
    pub fn new() -> Self {
        let builder = Builder::new();

        Self {
            obj: builder,
            tokens: VecDeque::new(),

            funct: HashMap::new(),
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

    fn scan_func(&mut self) -> (String, Vec<AsmInstructionEnum>) {
        let mut asm: Vec<AsmInstructionEnum> = vec![];
        let mut name: String = String::from("_asc_error");

        let mut scope = false;

        loop {
            let tok = self.advance();

            match tok {
                Token::IDENT(x) => {
                    if !scope {
                        let peek = self.peek();
                        if peek == Token::LBracket {
                            name = x;
                            scope = true;
                        } else {
                            println!(
                                "{}{} unexpected token '{:#?}' after function",
                                "error".red().bold(),
                                ":".bold(),
                                peek
                            );
                            match peek {
                                Token::IDENT(x) => {
                                    println!("{} | {x}{}", "+".green().bold(), "{".gray())
                                }
                                _ => {}
                            }
                        }
                    } else {
                        if x == "ret" {
                            asm.push (Ret);
                        } else if x == "nop" {
                            asm.push (Nop);
                        } else if is_reg(&x) {
                            let reg = to_reg(&x);

                            let mut op = String::from("rax");

                            if self.peek() == Token::EQUAL {
                                self.advance();
                                op = {
                                    let adv = self.advance();
                                    match adv {
                                        Token::IDENT(x) => x.clone(),
                                        Token::NUM(x) => x,
                                        _ => {
                                            println!("unexpected token");
                                            break;
                                        }
                                    }
                                };
                            } else {
                                println!("Unexpected register {:?}", reg);
                            }

                            if is_reg(&op) {
                                asm.push(MovReg(reg, to_reg(&op)));
                            } else {
                                asm.push( MovVal(reg, op.parse::<u64>().unwrap()));
                            }
                        } else {
                            println!(
                                "{}{} unexpected identifer '{}'",
                                "error".red().bold(),
                                ":".bold(),
                                x
                            );

                        }
                    }
                }

                Token::RBracket => {
                    if scope {
                        break;
                    } else {
                        println!("{}: unexpected '}}'", "error".red().bold());
                    }
                }

                Token::EOF => {
                    break;
                }

                _ => {}
            }
        }

        (name, asm)
    }

    pub fn gen(&mut self, tokens: Vec<Token>) {
        self.tokens = tokens.into();

        while self.tokens.len() > 0 {
            let scan = self.scan_func();
            self.funct.insert(scan.0, scan.1);
        }
    }

    pub fn build(&mut self, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        for func in self.funct.iter() {
            self.obj.define(func.0, true, func.1.to_owned())?;
        }

        self.obj
            .write(format!("{}", path.display()).as_str(), BinFormat::host())?;

        Ok(())
    }
}
