mod commands;
mod errors;

pub use crate::errors::GithubError;
use std::{env::VarError, path::PathBuf, str::FromStr};

#[derive(Debug)]
pub struct GithubCLI {
    root: PathBuf,
    config: PathBuf,
    mode: GithubTarget,
}

impl GithubCLI {
    pub fn new() -> Result<Self, GithubError> {
        let root = match std::env::var("GITHUB_WORKSPACE") {
            Ok(o) => PathBuf::from(o),
            Err(e) => panic!("{e}"),
        };
        let config = match std::env::var("INPUT_CONFIG") {
            Ok(o) => root.join(o),
            Err(_) => panic!("Missing `config`"),
        };
        let mode = match std::env::var("INPUT_MODE") {
            Ok(o) => GithubTarget::from_str(&o)?,
            Err(_) => panic!("Missing `mode`"),
        };
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
