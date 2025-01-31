use crate::{GithubCLI, GithubError};
use std::{collections::BTreeMap, path::Path};

impl GithubCLI {
    pub async fn run(&self) -> Result<(), GithubError> {
        println!("Config: {:?}", std::fs::read_to_string(&self.config));
        let mut envs = BTreeMap::new();
        for (key, value) in std::env::vars() {
            envs.insert(key, value);
        }
        println!("Env: {:#?}", envs);
        Ok(())
    }
}
