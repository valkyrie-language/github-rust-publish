mod commands;
mod errors;

pub use crate::errors::GithubError;
use std::str::FromStr;

#[derive(Debug)]
pub struct GithubCLI {}

impl GithubCLI {
    pub fn new() -> Result<Self, GithubError> {
        Ok(GithubCLI {})
    }
}

#[repr(u8)]
#[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
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
