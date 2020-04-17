use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
};

mod convert;

pub struct GithubError {
    kind: Box<LegionErrorKind>,
}

#[derive(Debug)]
pub enum LegionErrorKind {
    Custom { message: String },
}

impl Error for GithubError {}

impl Debug for GithubError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.kind, f)
    }
}
impl Display for GithubError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}
impl Display for LegionErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LegionErrorKind::Custom { message } => {
                write!(f, "{}", message)
            }
        }
    }
}

impl AsRef<LegionErrorKind> for GithubError {
    fn as_ref(&self) -> &LegionErrorKind {
        &self.kind
    }
}

impl AsMut<LegionErrorKind> for GithubError {
    fn as_mut(&mut self) -> &mut LegionErrorKind {
        &mut self.kind
    }
}
