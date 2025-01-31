use crate::helpers::{github_arch::GithubArch, github_os::GithubOs};
use std::{path::PathBuf, sync::LazyLock};

pub mod github_arch;
pub mod github_os;

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

pub static GITHUB_WORKSPACE: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from(get_env("GITHUB_WORKSPACE")));

pub static RUNNER_OS: LazyLock<GithubOs> = LazyLock::new(|| {
    let os = get_env("RUNNER_OS");
    match os.as_str() {
        "Windows" => GithubOs::Windows,
        "Linux" => GithubOs::Linux,
        "macOS" => GithubOs::MacOs,
        _ => panic!("unknown os: {}", os),
    }
});

pub static RUNNER_ARCH: LazyLock<GithubArch> = LazyLock::new(|| {
    let os = get_env("RUNNER_ARCH");
    match os.as_str() {
        "x86_64" => GithubArch::X86_64,
        "x64" => GithubArch::X64,
        "arm64" => GithubArch::Arm64,
        _ => panic!("Unknown RUNNER_ARCH: {}", os),
    }
});
