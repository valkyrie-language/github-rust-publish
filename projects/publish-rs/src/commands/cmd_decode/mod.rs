use crate::{GithubError, commands::LegionArguments};
use clap::Parser;
use std::path::{Path, PathBuf};
use wasmprinter::PrintFmtWrite;

#[derive(Debug, Parser)]
pub struct CommandDecode {
    /// The input `*.wasm` file path
    input: String,
    /// The output `*.wat` file path
    ///
    /// If not specified, the output file will be named `<input>.wat`.
    output: Option<String>,
    /// Skip output if file already exists
    #[arg(short = 'p', long, visible_alias = "protect")]
    output_protect: bool,
    /// Generate a skeleton only file
    #[arg(short = 's', long)]
    skeleton: bool,
    #[arg(long, default_value = "4", value_name = "INDENT_LEVEL")]
    indent: usize,
    #[arg(long)]
    indent_text: Option<String>,
    #[arg(short = 'f', long, visible_alias = "fold")]
    fold_instructions: bool,
    #[arg(long)]
    print: bool,
}

impl CommandDecode {
    pub async fn run(&self, _: &LegionArguments) -> Result<(), GithubError> {
        let bytes = std::fs::read(&self.input)?;
        let mut parser = wasmprinter::Config::new();
        parser.name_unnamed(true);
        parser.print_offsets(false);
        parser.print_skeleton(self.skeleton);
        parser.indent_text(self.get_indent_text());
        parser.fold_instructions(self.fold_instructions);
        let mut dst = String::new();
        parser.print(&bytes, &mut PrintFmtWrite(&mut dst))?;
        if self.print {
            println!("{}", dst);
        }
        let output = self.guess_output();
        if output.exists() {
            if self.output_protect {
                println!("Output file already exists, skipping.");
                return Ok(());
            }
        }
        Ok(std::fs::write(&output, dst.as_bytes())?)
    }
    pub fn get_indent_text(&self) -> String {
        match self.indent_text.as_ref() {
            Some(s) => s.to_string(),
            None => " ".repeat(self.indent),
        }
    }
    pub fn guess_output(&self) -> PathBuf {
        let input = Path::new(&self.input);
        match self.output.as_ref() {
            Some(s) => PathBuf::from(s),
            None => input.with_extension("wat"),
        }
    }
}
