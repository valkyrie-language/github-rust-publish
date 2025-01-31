mod commands;
mod errors;
mod helpers;

pub use crate::errors::GithubError;
use std::{path::PathBuf, str::FromStr};
use crate::helpers::{get_input, github_workspace};

#[derive(Debug)]
pub struct GithubCLI {
    root: PathBuf,
    config: PathBuf,
    mode: GithubTarget,
}

impl GithubCLI {
    pub fn new() -> Result<Self, GithubError> {
        let root = github_workspace();
        let config = PathBuf::from(get_input("config"));
        let mode = GithubTarget::from_str(&get_input("mode"))?;
        Ok(GithubCLI { root, config, mode })
    }
}



#[repr(u8)]
#[derive(Clone, Copy, Eq, Debug, Ord, PartialEq, PartialOrd)]
pub enum GithubTarget {
    All,
    Github,
    Npm,
    Cargo,
}

impl FromStr for GithubTarget {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "all" => Ok(GithubTarget::All),
            "github" => Ok(GithubTarget::Github),
            "npm" => Ok(GithubTarget::Npm),
            "cargo" => Ok(GithubTarget::Cargo),
            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid target")),
        }
    }
}
