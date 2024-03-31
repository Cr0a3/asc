use std::{collections::HashMap, env::current_exe, path::PathBuf};
use CodeGenLib::{ArtifactError, Builder, Function};
use crate::token::Token;

pub struct CodeGen {
    builder: Builder,
    functs: HashMap<String, Function>,
}

impl CodeGen {
    pub fn new() -> Self {
        let builder = Builder::new();
        Self {
            builder: builder,
            functs: HashMap::new(),
        }
    }

    pub fn gen(&mut self, tokens: Vec<Token> ) {

        let mut func_scope = false;

        let mut func_scope_name = String::new();

        for token in tokens {
            match token {
                Token::RET => {
                    if func_scope {
                        print!("ret");
                        let func = self.functs.get_mut(&func_scope_name).unwrap();
                        func.asm_ret();
                    }
                },
                Token::IDENT(x) => {
                    if !func_scope {
                        let mut func = self.builder.add_function(x.clone().as_str()).to_owned();
                        self.functs.insert(x.clone(), func);
                        func_scope = true;
                        func_scope_name = x;
                    } else {

                    }
                }

                _ => {}
            }
        }
    }

    pub fn build(&mut self, path: PathBuf) -> Result<(), ArtifactError> {
        self.builder.build(format!("{}", path.display()).as_str())
    }
}