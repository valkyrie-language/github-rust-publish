mod commands;
mod errors;
mod helpers;

pub use crate::errors::GithubError;
use crate::helpers::{GITHUB_WORKSPACE, get_input};
use std::{path::PathBuf, str::FromStr};

#[derive(Debug)]
pub struct GithubCLI {
    config: PathBuf,
    mode: PublishTarget,
}

impl GithubCLI {
    pub fn new() -> Result<Self, GithubError> {
        let config = GITHUB_WORKSPACE.join(get_input("config"));
        let mode = PublishTarget::from_str(&get_input("mode"))?;
        Ok(GithubCLI { config, mode })
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Eq, Debug, Ord, PartialEq, PartialOrd)]
pub enum PublishTarget {
    All,
    Github,
    Npm,
    Cargo,
}

impl FromStr for PublishTarget {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "all" => Ok(PublishTarget::All),
            "github" => Ok(PublishTarget::Github),
            "npm" => Ok(PublishTarget::Npm),
            "cargo" => Ok(PublishTarget::Cargo),
            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid target")),
        }
    }
}
