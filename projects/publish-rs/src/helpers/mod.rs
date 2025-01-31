use std::path::PathBuf;

pub fn get_input(input: &str) -> String {
    let key = format!("INPUT_{}", input.to_ascii_uppercase());
    match std::env::var(key) {
        Ok(o) => o,
        Err(_) => panic!("missing `{}`", input),
    }
}
pub fn get_env(key: &str) -> String {
    match std::env::var(key) {
        Ok(o) => o,
        Err(e) => panic!("missing `{}`", key),
    }
}
pub fn github_workspace() -> PathBuf {
    PathBuf::from(get_env("GITHUB_WORKSPACE"))
}

pub enum GithubOs {
    Windows,
    Linux,
    MacOs,
}
pub fn image_os() -> GithubOs {
    let os = get_env("ImageOS");
    if os.starts_with("ubuntu") {
        return GithubOs::Linux;
    }
    if os.starts_with("macos") {
        return GithubOs::MacOs;
    }
    if os.starts_with("win") {
        return GithubOs::Windows;
    }
    panic!("unknown os: {}", os);
}

impl GithubOs {
    pub fn cargo_bin(&self) -> &'static str {
        match self {
            GithubOs::Linux => "/home/runner/.cargo/bin",
            GithubOs::Windows => "C:\\Users\\runneradmin\\.cargo\\bin",
            GithubOs::MacOs => "/Users/runner/.cargo/bin",
        }
    }
}
