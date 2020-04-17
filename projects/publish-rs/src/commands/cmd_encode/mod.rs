use crate::{GithubError, commands::LegionArguments};
use clap::Parser;
use std::path::{Path, PathBuf};
use wat::GenerateDwarf;

/// encode `wat`, `wast` to wasm
#[derive(Debug, Parser)]
pub struct CommandEncode {
    /// Input `wat` file path
    input: String,
    /// Output `wasm` file path
    output: Option<String>,
    /// Skip output if file already exists
    #[arg(short = 'p', long, visible_alias = "protect")]
    output_protect: bool,
    /// Generate DWARF debugging information
    #[arg(short = 'd', long)]
    generate_dwarf: bool,
}

impl CommandEncode {
    pub async fn run(&self, _: &LegionArguments) -> Result<(), GithubError> {
        let input = std::fs::read_to_string(&self.input)?;
        let mut parser = wat::Parser::new();
        if self.generate_dwarf {
            parser.generate_dwarf(GenerateDwarf::Full);
        }
        let bytes = parser.parse_str(None, input)?;
        let output = self.guess_output();
        if self.output_protect {
            if output.exists() {
                println!("{}", "Skipping override");
                return Ok(());
            }
        }
        Ok(std::fs::write(&output, bytes)?)
    }
    pub fn guess_output(&self) -> PathBuf {
        let input = Path::new(&self.input);
        match self.output.as_ref() {
            Some(s) => PathBuf::from(s),
            None => input.with_extension("wasm"),
        }
    }
}
