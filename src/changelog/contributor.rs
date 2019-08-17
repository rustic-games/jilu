use serde::Serialize;
use std::cmp::Ordering;

/// The contributor to a change.
#[derive(Debug, Serialize, Eq, PartialEq, PartialOrd)]
pub(crate) struct Contributor {
    pub(crate) name: String,
    pub(crate) email: String,
}

impl Ord for Contributor {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

impl From<(&str, &str)> for Contributor {
    fn from((name, email): (&str, &str)) -> Self {
        Self {
            name: name.to_owned(),
            email: email.to_owned(),
        }
    }
}
