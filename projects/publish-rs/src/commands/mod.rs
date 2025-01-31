use crate::{
    GithubError, bindings,
    bindings::{GithubTarget, Guest, export},
};
use std::{collections::HashMap, path::Path};
use std::collections::BTreeMap;

pub struct RunningContext {}

impl Guest for RunningContext {
    fn run_with_config(config: String, target: GithubTarget) -> Result<(), GithubError> {
        match std::env::var("INPUT_CONFIG") {
            Ok(o) => {
                println!("Config Path: {}", o);
            }
            Err(e) => {
                println!("MissingConfig {}", e);
            }
        }
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
                println!("GITHUB_WORKSPACE");
                read_dir(&s)
            }
            Err(e) => {
                println!("    {}", e)
            }
        }
        let mut envs = BTreeMap::new();
        for (key, value) in std::env::vars() {
            envs.insert(key, value);
        }
        println!("Env: {:#?}", envs);
        Ok(())
    }
}
fn read_dir(dir_path: &str) {
    let path = Path::new(dir_path);
    if let Ok(dir_entries) = std::fs::read_dir(path) {
        for entry in dir_entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                println!("{}", file_name.to_string_lossy());
            }
        }
    }
    else {
        println!("Error reading directory: {}", dir_path);
    }
}
export!(RunningContext with_types_in bindings);
