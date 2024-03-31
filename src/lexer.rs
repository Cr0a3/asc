use std::{fs::File, io::{self, Read}, path::PathBuf};
use PrintLib::colorize::Colorize;

use crate::token::Token;

pub struct Lexer {
    tokens: Vec<Token>
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            tokens: vec![],
        }
    }

    pub fn scan(&mut self, file: PathBuf) -> io::Result<()>{
        let mut file = File::open(file)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let chars = content.chars();

        for c in chars {
            match c {
                '\n' => {},
                '\r' => {},
                '\t' => {},
                '+' =>  { self.tokens.push( Token::ADD );   },
                '-' =>  { self.tokens.push( Token::ADD );   },
                '*' =>  { self.tokens.push( Token::ADD );   },
                '/' =>  { self.tokens.push( Token::DIV );   },
                '=' =>  { self.tokens.push( Token::EQUAL ); },
                _ => {
                    if  c >= '0' && c <= '9'{
                        self.num();
                    }
                    else if  (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z') || c == '_' || c == '-' {
                        self.identifer();
                    } else {
                        println!("{} unexpected character: '{c}'", "error:".red().bold());
                    }
                }
            }
        }

        Ok(())
    }

    pub fn num(&mut self) {

    }

    pub fn identifer(&mut self) {
        
    }
}