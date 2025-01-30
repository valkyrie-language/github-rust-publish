use crate::{
    GithubError, bindings,
    bindings::{Guest, export},
};
use std::{
    path::{Path, PathBuf},
};
use crate::bindings::GithubTarget;

pub struct RunningContext {}

impl Guest for RunningContext {
    fn run_with_config(config: String, target: GithubTarget) -> Result<(), GithubError> {
        tokio::runtime::Builder::new_current_thread().enable_all().build()?.block_on(async {
            let ctx = RunningContext {};
            ctx.run(config).await
        })
    }
}

impl RunningContext {
    async fn run(&self, config: String) -> Result<(), GithubError> {
        match std::env::var("GITHUB_WORKSPACE") {
            Ok(s) => {
                let config = PathBuf::from(s).join(&config);
                let txt = std::fs::read_to_string(config);
                println!("Config: {:?}", txt)
            }
            Err(e) => {
                println!("{}", e)
            }
        }
        println!("Env: {:#?}", std::env::vars());
        Ok(())
    }
}

export!(RunningContext with_types_in bindings);
