use std::env::{current_dir, current_exe};
use std::path::Path;
use crate::{
    GithubError, bindings,
    bindings::{Guest, export},
};

pub struct RunningContext {}

impl Guest for RunningContext {
    fn run_with_config(config: String) -> Result<(), GithubError> {
        tokio::runtime::Builder::new_current_thread().enable_all().build()?.block_on(async {
            let ctx = RunningContext {};
            ctx.run(config).await
        })
    }
}

impl RunningContext {
    async fn run(&self, config: String) -> Result<(), GithubError> {
        let args = std::env::args();
        println!("Args: {:?}", args);
        println!("Dir: {:?}", current_dir());
        println!("Exe: {:?}", current_exe());
        println!("Config: {}", config);
        println!("Config: {:?}", Path::new(&config).canonicalize());
        println!("Env: {:?}", std::env::vars());
        Ok(())
    }
}

export!(RunningContext with_types_in bindings);
