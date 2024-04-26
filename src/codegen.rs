use crate::{token::Token, util::{is_reg, to_reg}};
use std::{
    collections::{HashMap, VecDeque},
    path::PathBuf,
};
use CodeGenLib::{asm::AsmInstructionEnum, Builder, IR::*};
use PrintLib::colorize::Colorize;

pub struct CodeGen {
    obj: Builder,
    tokens: VecDeque<Token>,
    funct: HashMap<String, Vec<AsmInstructionEnum>>,

    labels: usize,

    error: bool,
}

impl CodeGen {
    pub fn new() -> Self {
        Self {
            obj: Builder::new(),
            tokens: VecDeque::new(),

            funct: HashMap::new(),

            error: false,

            labels: 0,
        }
    }

    fn request_label_name(&self) -> String {
        self.labels.to_string()
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

                            self.error = true;
                        }
                    } else {
                        if x == "ret" {
                            asm.push (Ret);
                        } else if x == "nop" {
                            asm.push (Nop);
                        } else if x == "call" {
                            let peek = self.peek();
                            match peek {
                                Token::IDENT(x) => { 
                                    asm.push(Call(x)); 
                                    self.advance(); // so index gets updated
                                },
                                _ => { println!("needs to be identifier"); }
                            }
                        } else if is_reg(&x) {
                            let reg = to_reg(&x);

                            let peek = self.peek();

                            if peek == Token::EQUAL {
                                self.advance();
                                let op = {
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
                                
                                if is_reg(&op) {
                                    asm.push(MovReg(reg, to_reg(&op)));
                                } else if op == "ptr" {
                                    let data = match self.advance() {
                                        Token::STR(string) => string,
                                        token => {
                                            println!("name needs to be str. found {:?}", token);
                                            break;
                                        }
                                    };

                                    let data = data.as_bytes();
                                    let data = data.to_vec();

                                    let name = self.request_label_name();

                                    self.obj.define_label(
                                        &name, 
                                        false,
                                        data
                                    );

                                    asm.push( MovPtr( to_reg(&x), name ) );
                                } else {
                                    asm.push( MovVal(reg, op.parse::<i64>().unwrap()));
                                }
                            } else if peek == Token::ADD {
                                self.advance();
                                let op = {
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

                                if is_reg(&op) {
                                    asm.push(AddReg(reg, to_reg(&op)));
                                } else {
                                    asm.push( AddVal(reg, op.parse::<i64>().unwrap()));
                                }
                            } else if peek == Token::SUB {
                                self.advance();
                                let op = {
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

                                if is_reg(&op) {
                                    asm.push( SubReg(reg, to_reg(&op)));
                                } else {
                                    asm.push( SubVal(reg, op.parse::<i64>().unwrap()));
                                }
                            } else if peek == Token::MUL {
                                self.advance();
                                let op = {
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

                                if is_reg(&op) {
                                    asm.push(MulReg(reg, to_reg(&op)));
                                } else {
                                    asm.push( MulVal(reg, op.parse::<i64>().unwrap()));
                                }
                            } else {
                                println!("Unexpected register {:?}", reg);
                            }
                        } else if x == "push" {
                            let op = {
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

                            if is_reg(&op) {
                                asm.push(Push(to_reg(&op)));
                            } else {
                                asm.push( PushVal(op.parse::<i64>().unwrap()));
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
            println!("{:?}", func);

            self.obj.define(func.0, true, func.1.to_owned())?;
        }

        self.obj
            .write(format!("{}", path.display()).as_str()
        )?;

        Ok(())
    }
}
