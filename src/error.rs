use std::{error, fmt};

/// All possible library errors.
#[derive(Debug)]
pub enum Error {
    /// Configuration error
    Config(ron::de::Error),

    /// CLI error
    Cli(lexopt::Error),

    /// The commit does not adhere to the conventional spec.
    ConventionalCommit(conventional::Error),

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
    SemVer(semver::Error),

    /// A templating error.
    Template(tera::Error),

    /// A Timestamp related error.
    Timestamp(chrono::format::ParseError),

    /// Any error related to invalid UTF-8 bytes.
    Utf8Error,

    /// A JSON parsing error.
    Json(serde_json::Error),

    /// A JQ program error.
    Jq(String),
}

use jaq_core::load;
use Error::*;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Config(ref err) => write!(f, "Configuration error: {}", err),
            Cli(ref err) => write!(f, "CLI error: {}", err),
            ConventionalCommit(ref err) => write!(f, "Conventional Commit error: {}", err),
            Format(ref err) => write!(f, "Format error: {}", err),
            Generic(ref string) => write!(f, "Unknown error: {}", string),
            InvalidCommitType => f.write_str("Invalid commit type"),
            InvalidTag => f.write_str("Invalid Git tag"),
            IO(ref err) => write!(f, "IO error: {}", err),
            Git(ref err) => write!(f, "Git error: {}", err),
            MissingCommitMessage => f.write_str("Missing commit message"),
            SemVer(ref err) => write!(f, "SemVer error: {}", err),
            Template(ref err) => write!(f, "Template error: {}", {
                use std::error::Error as _;

                let mut msg = err.to_string();
                let mut source = err.source();
                while let Some(err) = source {
                    msg.push_str(&format!(": {}", err));
                    source = err.source();
                }

                msg
            }),
            Timestamp(ref err) => write!(f, "Timestamp error: {}", err),
            Utf8Error => f.write_str("UTF-8 error"),
            Json(ref err) => write!(f, "JSON error: {}", err),
            Jq(ref err) => write!(f, "JQ error: {}", err),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Config(ref err) => Some(err),
            Cli(ref err) => Some(err),
            ConventionalCommit(ref err) => Some(err),
            Format(ref err) => Some(err),
            Git(ref err) => Some(err),
            IO(ref err) => Some(err),
            SemVer(ref err) => Some(err),
            Template(ref err) => Some(err),
            Timestamp(ref err) => Some(err),
            Json(ref err) => Some(err),

            Generic(_) | InvalidCommitType | InvalidTag | MissingCommitMessage | Utf8Error
            | Jq(_) => None,
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

impl From<semver::Error> for Error {
    fn from(err: semver::Error) -> Self {
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

impl From<conventional::Error> for Error {
    fn from(err: conventional::Error) -> Self {
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

impl From<lexopt::Error> for Error {
    fn from(err: lexopt::Error) -> Self {
        Error::Cli(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err)
    }
}

impl From<jaq_core::Error<jaq_json::Val>> for Error {
    fn from(err: jaq_core::Error<jaq_json::Val>) -> Self {
        Error::Jq(err.to_string())
    }
}

impl<P> From<load::Errors<&str, P, Vec<jaq_core::compile::Error<&str>>>> for Error {
    fn from(err: load::Errors<&str, P, Vec<jaq_core::compile::Error<&str>>>) -> Self {
        Error::Jq(
            err.into_iter()
                .next()
                .into_iter()
                .flat_map(|e| {
                    e.1.into_iter()
                        .map(|(a, b)| format!("undefined {}: {a}", b.as_str()))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
                .join(", "),
        )
    }
}

impl<P> From<load::Errors<&str, P, load::Error<&str>>> for Error {
    fn from(err: load::Errors<&str, P, load::Error<&str>>) -> Self {
        Error::Jq(
            err.first()
                .map(|e| match &e.1 {
                    load::Error::Io(items) => items
                        .iter()
                        .map(|(_, e)| e.clone())
                        .collect::<Vec<_>>()
                        .join(", "),
                    load::Error::Lex(items) => items
                        .iter()
                        .map(|(a, b)| format!("Expected {}, got {b}", a.as_str()))
                        .collect::<Vec<_>>()
                        .join(", "),
                    load::Error::Parse(items) => items
                        .iter()
                        .map(|(a, b)| format!("Expected {}, got {b}", a.as_str()))
                        .collect::<Vec<_>>()
                        .join(", "),
                })
                .unwrap_or_default(),
        )
    }
}
