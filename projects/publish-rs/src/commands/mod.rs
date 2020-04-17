pub use self::{cmd_build::CommandBuild, cmd_encode::CommandEncode, cmd_run::CommandRun};
use crate::{
    GithubError,
    commands::{cmd_decode::CommandDecode, cmd_polyfill::CommandPolyfill},
};
use clap::{Args, Subcommand};
use std::path::PathBuf;

mod cmd_build;
mod cmd_decode;
mod cmd_encode;
mod cmd_polyfill;
mod cmd_run;

#[derive(Debug, Subcommand)]
pub enum LegionCommands {
    /// Build the legion project
    Run(CommandRun),
    /// Build the legion project
    Build(CommandBuild),
    /// encode `wat`, `wast` to wasm
    Encode(CommandEncode),
    /// decode `wasm` to `wat`
    Decode(CommandDecode),
    /// decode `wasm` to `js`
    #[command(visible_alias = "shim")]
    Polyfill(CommandPolyfill),
}

#[derive(Debug, Args)]
pub struct LegionArguments {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

impl LegionCommands {
    pub async fn run(self, arguments: &LegionArguments) -> Result<(), GithubError> {
        match self {
            Self::Run(cmd) => cmd.run(arguments).await,
            Self::Build(cmd) => cmd.run(arguments).await,
            Self::Encode(cmd) => cmd.run(arguments).await,
            Self::Decode(cmd) => cmd.run(arguments).await,
            Self::Polyfill(cmd) => cmd.run(arguments).await,
        }
    }
}
