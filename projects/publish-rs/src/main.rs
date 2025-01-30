use clap::Parser;
use github::{GithubCLI, GithubError};

fn main() -> Result<(), GithubError> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async { GithubCLI::parse().run().await })
}
