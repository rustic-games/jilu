use serde::Serialize;

/// The contributor to a change.
#[derive(Debug, Serialize)]
pub(crate) struct Contributor {
    pub(crate) name: String,
    pub(crate) email: String,
}

impl From<(&str, &str)> for Contributor {
    fn from((name, email): (&str, &str)) -> Self {
        Self {
            name: name.to_owned(),
            email: email.to_owned(),
        }
    }
}
