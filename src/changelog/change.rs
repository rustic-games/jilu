use crate::changelog::Contributor;
use crate::git::Commit;
use crate::Error;
use conventional::{Commit as CCommit, Simple as _};
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::collections::HashMap;

/// A single change in the change log.
#[derive(Debug)]
pub(crate) struct Change<'a> {
    commit: &'a Commit,
    conventional: CCommit<'a>,
}

impl<'a> Change<'a> {
    pub(crate) fn new(commit: &'a Commit) -> Result<Self, Error> {
        let conventional = CCommit::new(&commit.message)?;

        Ok(Self {
            commit,
            conventional,
        })
    }

    /// The type of the change.
    pub(crate) fn type_(&self) -> &str {
        &self.conventional.type_()
    }

    /// The short description of the change.
    pub(crate) fn description(&self) -> &str {
        self.conventional.description()
    }

    /// The body of the change.
    pub(crate) fn body(&self) -> Option<&str> {
        self.conventional.body()
    }

    /// Get the "short Git ID"
    ///
    /// This default to 7 characters of the object ID, but is extended to avoid
    /// ambiguity.
    pub(crate) fn short_id(&self) -> &str {
        &self.commit.short_id
    }

    /// The regular Git reference.
    pub(crate) fn id(&self) -> &str {
        &self.commit.id
    }

    /// The contributor details of this change.
    pub(crate) fn contributor(&self) -> Contributor {
        (
            self.commit.author.name.as_str(),
            self.commit.author.email.as_str(),
        )
            .into()
    }
}

impl Serialize for Change<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut commit = HashMap::new();
        commit.insert("id", self.id());
        commit.insert("short_id", self.short_id());

        let mut state = serializer.serialize_struct("Change", 4)?;
        state.serialize_field("type", &self.type_())?;
        state.serialize_field("description", &self.description())?;
        state.serialize_field("body", &self.body())?;
        state.serialize_field("commit", &commit)?;
        state.end()
    }
}
