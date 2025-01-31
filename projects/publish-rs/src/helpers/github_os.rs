use std::path::Path;

pub enum GithubOs {
    Windows,
    Linux,
    MacOs,
}

impl GithubOs {
    pub fn cargo_bin(&self) -> &'static Path {
        let path = match self {
            GithubOs::Linux => "/home/runner/.cargo/bin",
            GithubOs::Windows => "C:\\Users\\runneradmin\\.cargo\\bin",
            GithubOs::MacOs => "/Users/runner/.cargo/bin",
        };
        Path::new(path)
    }
}
