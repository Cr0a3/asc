use std::path::PathBuf;

use CodeGenLib::{Builder, ArtifactError};

pub struct CodeGen<'a> {
    builder: Builder<'a>,
}

impl<'a> CodeGen<'a> {
    pub fn new() -> Self {
        let builder = Builder::new();
        Self {
            builder: builder,
        }
    }

    pub fn gen(&mut self, file: PathBuf) -> Result<(), ArtifactError>{
        self.builder.build(&format!("{}", file.display()))?;

        Ok(())
    }
}