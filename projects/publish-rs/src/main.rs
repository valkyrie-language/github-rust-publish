use github::{GithubError, GithubCLI};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), GithubError> {
    GithubCLI::new()?.run().await
}
