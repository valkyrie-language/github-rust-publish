pub mod commands;
mod errors;
use crate::commands::{LegionArguments, LegionCommands};
pub use crate::errors::GithubError;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct GithubCLI {
    #[command(subcommand)]
    commands: Option<LegionCommands>,
    #[command(flatten)]
    arguments: LegionArguments,
}

impl GithubCLI {
    pub async fn run(self) -> Result<(), GithubError> {
        let args: Vec<String> = std::env::args().collect();
        println!("Args:\n{:#?}", args);
        let Self { commands, arguments } = self;
        match commands {
            Some(s) => s.run(&arguments).await?,
            None => {}
        }
        Ok(())
    }
}
