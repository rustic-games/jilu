use semver::SemVerError;
use std::{error, fmt};

/// All possible library errors.
#[derive(Debug)]
pub enum Error {
    /// Configuration error
    Config(ron::de::Error),

    /// The commit does not adhere to the conventional spec.
    ConventionalCommit(conventional_commit::Error),

    /// A formatting error.
    Format(fmt::Error),

    /// A generic error.
    Generic(String),

    /// A Git related error.
    Git(git2::Error),

    /// The commit type is not accepted.
    InvalidCommitType,

    /// The provided Git tag is invalid.
    InvalidTag,

    /// Any IO error.
    IO(std::io::Error),

    /// The commit is missing a message.
    MissingCommitMessage,

    /// A SemVer related error.
    SemVer(SemVerError),

    /// A templating error.
    Template(tera::Error),

    /// A Timestamp related error.
    Timestamp(chrono::format::ParseError),

    /// Any error related to invalid UTF-8 bytes.
    Utf8Error,
}

use Error::*;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Config(ref err) => write!(f, "Configuration error: {}", err),
            ConventionalCommit(ref err) => write!(f, "Conventional Commit error: {}", err),
            Format(ref err) => write!(f, "Format error: {}", err),
            Generic(ref string) => write!(f, "Unknown error: {}", string),
            InvalidCommitType => f.write_str("Invalid commit type"),
            InvalidTag => f.write_str("Invalid Git tag"),
            IO(ref err) => write!(f, "IO error: {}", err),
            Git(ref err) => write!(f, "Git error: {}", err),
            MissingCommitMessage => f.write_str("Missing commit message"),
            SemVer(ref err) => write!(f, "SemVer error: {}", err),
            Template(ref err) => write!(f, "Template error: {}", err),
            Timestamp(ref err) => write!(f, "Timestamp error: {}", err),
            Utf8Error => f.write_str("UTF-8 error"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Config(ref err) => Some(err),
            ConventionalCommit(ref err) => Some(err),
            Format(ref err) => Some(err),
            Git(ref err) => Some(err),
            IO(ref err) => Some(err),
            SemVer(ref err) => Some(err),
            Template(ref err) => Some(err),
            Timestamp(ref err) => Some(err),

            Generic(_) | InvalidCommitType | InvalidTag | MissingCommitMessage | Utf8Error => None,
        }
    }
}

impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::Generic(err.to_owned())
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::Generic(err)
    }
}

impl From<SemVerError> for Error {
    fn from(err: SemVerError) -> Self {
        Error::SemVer(err)
    }
}

impl From<chrono::format::ParseError> for Error {
    fn from(err: chrono::format::ParseError) -> Self {
        Error::Timestamp(err)
    }
}

impl From<git2::Error> for Error {
    fn from(err: git2::Error) -> Self {
        Error::Git(err)
    }
}

impl From<fmt::Error> for Error {
    fn from(err: fmt::Error) -> Self {
        Error::Format(err)
    }
}

impl From<conventional_commit::Error> for Error {
    fn from(err: conventional_commit::Error) -> Self {
        Error::ConventionalCommit(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<ron::de::Error> for Error {
    fn from(err: ron::de::Error) -> Self {
        Error::Config(err)
    }
}

impl From<tera::Error> for Error {
    fn from(err: tera::Error) -> Self {
        Error::Template(err)
    }
}
