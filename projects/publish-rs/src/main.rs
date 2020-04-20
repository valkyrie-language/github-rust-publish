use clap::Parser;
use github::{GithubCLI, GithubError};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), GithubError> {
    GithubCLI::parse().run().await
}
