use std::{
    collections::HashMap,
    fs::File,
    io::{self, Read},
    path::PathBuf,
};
use PrintLib::error::ErrorFactory;

use crate::token::Token;

pub struct Lexer {
    pub tokens: Vec<Token>,

    start: usize,
    pos: usize,

    pos_in_line: usize,
    line: usize,
    linestr: String,
    lines: Vec<String>,

    file: String,
    filename: String,

    keys: HashMap<String, Token>,

    pub error: bool,
}

impl Lexer {
    pub fn new() -> Self {
        let /*mut*/ keywords: HashMap<String, Token> = HashMap::new();

        Self {
            tokens: vec![],

            start: 0,
            pos: 0,
            pos_in_line: 0,

            line: 0,
            linestr: String::new(),
            lines: vec![],

            file: String::new(),
            filename: String::new(),

            keys: keywords,

            error: false,
        }
    }

    fn is_at_end(&self) -> bool {
        self.pos >= (self.file.len() - 1)
    }

    fn advance(&mut self) -> char {
        self.pos += 1;
        let mut peek_res = self.peek();

        if peek_res == '\n' {
            self.pos_in_line = 0;
            self.line += 1;

            if let Some(first) = self.lines.get(self.line - 1).cloned() {
                self.linestr = first;
            } else {
                eprintln!("error, while resolving new line");
            }

            peek_res = self.advance(); //result = new advance
        } else {
            self.pos_in_line += 1;
        }

        peek_res
    }

    fn peek(&self) -> char {
        self.file.chars().nth(self.pos - 1).unwrap_or('\0')
    }

    pub fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '\n' => {}
            '\r' => {}
            '\t' => {}
            ' ' => {}
            '+' => {
                self.tokens.push(Token::ADD);
            }
            '-' => {
                self.tokens.push(Token::SUB);
            }
            '*' => {
                self.tokens.push(Token::MUL);
            }
            '/' => {
                self.tokens.push(Token::DIV);
            }
            '=' => {
                self.tokens.push(Token::EQUAL);
            }
            '{' => {
                self.tokens.push(Token::LBracket);
            }
            '}' => {
                self.tokens.push(Token::RBracket);
            }

            '"' => {
                self.string();
            }

            _ => {
                if c >= '0' && c <= '9' {
                    self.num();
                } else if (c >= 'A' && c <= 'Z') || (c >= 'a' && c <= 'z') || c == '_' || c == '-' {
                    self.identifer();
                } else {
                    let mut e_fab =
                        ErrorFactory::new("".into(), format!("unexpected character '{c}'"));
                    e_fab.add_arrow(self.filename.to_string(), self.line, self.pos);
                    e_fab.add_code_line(self.linestr.clone(), true, self.line, false);
                    e_fab.add_where(self.pos, 1, false, String::new());

                    e_fab.print();
                    self.error = true;
                }
            }
        }
    }

    pub fn scan(&mut self, path: PathBuf) -> io::Result<()> {
        let mut file = File::open(&path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        self.file = content.clone();
        self.lines = content.lines().map(String::from).collect();
        self.filename = format!("{}", path.display());

        while !self.is_at_end() {
            self.start = self.pos.clone();

            self.scan_token();
        }

        Ok(())
    }

    pub fn num(&mut self) {
        let mut str = String::new();

        let mut ad: char = self.peek();

        while (ad >= '0' && ad <= '9') || ad == '.' || ad == '_' || ad == ',' {
            str.push(ad);
            ad = self.advance();
        }

        str = str.replace("_", "");

        self.tokens.push(Token::NUM(str));
    }

    pub fn string(&mut self) {
        let mut str = String::new();

        while self.advance() != '"' && !self.is_at_end() {
            let ad = self.peek();
            str.push(ad);
        }

        if self.is_at_end() {
            
            let mut e_fab =
                ErrorFactory::new("".into(), format!("undetermend string"));
            e_fab.add_arrow(self.filename.to_string(), self.line, self.pos);
            e_fab.add_code_line(self.linestr.clone(), true, self.line, false);
            e_fab.add_where(self.pos, 1, false, String::new());
        }

        self.advance(); // skip the closing "

        self.tokens.push( Token::STR(str) );
    }

    pub fn identifer(&mut self) {
        let mut str = String::new();

        let mut ad: char = self.peek();

        while (ad >= 'A' && ad <= 'Z')
            || (ad >= 'a' && ad <= 'z')
            || ad == '_'
            || (ad >= '0' && ad <= '9')
        {
            str.push(ad);
            ad = self.advance();
        }

        match self.keys.get(&str) {
            Some(&ref keyword) => {
                self.tokens.push(keyword.to_owned());
            }
            _ => {
                // if str is not in the keyword list
                self.tokens.push(Token::IDENT(str));
            }
        }
    }
}
