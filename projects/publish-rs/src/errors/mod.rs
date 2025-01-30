use crate::bindings::GithubError;

impl From<std::io::Error> for GithubError {
    fn from(error: std::io::Error) -> Self {
        GithubError::Custom(error.to_string())
    }
}

impl From<anyhow::Error> for GithubError {
    fn from(error: anyhow::Error) -> Self {
        GithubError::Custom(error.to_string())
    }
}

impl From<wat::Error> for GithubError {
    fn from(error: wat::Error) -> Self {
        GithubError::Custom(error.to_string())
    }
}

impl From<dialoguer::Error> for GithubError {
    fn from(error: dialoguer::Error) -> Self {
        GithubError::Custom(error.to_string())
    }
}
