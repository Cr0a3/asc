use std::{collections::HashMap, env::current_exe, path::PathBuf};
use CodeGenLib::{AdressManager, ArtifactError, Builder, Function};
use crate::token::Token;

pub struct CodeGen {
    builder: Builder,
}

impl CodeGen {
    pub fn new() -> Self {
        let builder = Builder::new();
        Self {
            builder: builder,
        }
    }

    pub fn gen(&mut self, tokens: Vec<Token> ) {

        let mut func_scope = false;

        let mut funct: &mut Function = &mut Function::new("null", &mut {let adrmng = AdressManager::new((0, 0)); adrmng});

        for token in tokens {
            match token {
                Token::RET => {
                    if func_scope {
                        funct.asm_ret();
                    }
                },
                Token::IDENT(x) => {
                    if !func_scope {
                        funct = self.builder.add_function(x.clone().as_str());
                        func_scope = true;
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