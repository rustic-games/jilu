use crate::changelog::{Change, Contributor};
use crate::git::{Commit, Tag};
use crate::Error;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

/// A set of changes belonging together.
#[derive(Debug, Default)]
pub(crate) struct ChangeSet {
    /// Internal reference to the changes in this change set.
    changes: Vec<Change>,
}

impl ChangeSet {
    /// Given a set of Git commits, take all commits belonging to this change
    /// set.
    ///
    /// There are two ways in which this method determines if a commit belongs
    /// to the change set.
    ///
    /// 1. If no tag is provided as the second argument, all provided commits
    ///    are considered to be part of the change set.
    ///
    /// 2. If a tag is provided, only commits belonging to that tag are added.
    ///
    /// This method mutates the provided slice of commits. After it is done, the
    /// slice will contain only those commits not part of this change set.
    ///
    /// If any type filters are provided, any commit that would be part of the
    /// change set is removed from the commit list, but not added to the change
    /// set.
    ///
    /// # Important
    ///
    /// Because of the design of Git, if a tag is provided, a commit is
    /// considered to be part of a change set if it is in the tree of parents of
    /// the commit the tag points to.
    ///
    /// This means that if you have a set of commits that you want to distribute
    /// across multiple change sets, you should make sure to call this method on
    /// a change set with the earliest tag first, and then move your way up to
    /// the latest tag. This ensures that each change set can only take commits
    /// belonging to itself.
    ///
    /// # Errors
    ///
    /// If any of the Git operations fail, an error is returned.
    pub(crate) fn take_commits(
        &mut self,
        commits: &mut Vec<Commit>,
        accept_types: &Option<Vec<String>>,
        tag: Option<&Tag>,
    ) -> Result<(), Error> {
        if commits.is_empty() {
            return Ok(());
        }

        let idx = match tag {
            None => Some(commits.len().checked_sub(1).unwrap_or_default()),
            Some(tag) => commits
                .iter()
                .enumerate()
                .skip_while(|(_, c)| c.id != tag.commit.id)
                .map(|(idx, _)| idx)
                .next(),
        };

        let changes = match idx {
            None => return Ok(()),
            Some(idx) => commits
                .drain(0..=idx)
                .filter_map(|commit| match Change::new(commit) {
                    Err(Error::InvalidCommitType) => None,
                    Err(err) => Some(Err(err)),
                    Ok(change) => Some(Ok(change)),
                })
                .collect::<Result<Vec<_>, _>>()?,
        };

        self.changes.append(
            &mut changes
                .into_iter()
                .rev()
                .filter(|c| {
                    if let Some(types) = accept_types {
                        types.iter().any(|f| f == c.type_())
                    } else {
                        true
                    }
                })
                .collect(),
        );

        Ok(())
    }

    /// Return the list of changes in this change set.
    pub(crate) fn changes(&self) -> &[Change] {
        &self.changes
    }

    /// A list of people who contributed to this change set.
    ///
    /// You can pass in a list of optional contributor emails to ignore.
    pub(crate) fn contributors(&self, ignore: Option<&[String]>) -> Vec<Contributor> {
        let mut contributors: Vec<_> = self
            .changes
            .iter()
            .map(Change::contributor)
            .filter(|c| !ignore.unwrap_or(&[]).iter().any(|email| email == &c.email))
            .collect();

        contributors.sort_unstable_by(|a, b| a.name.cmp(&b.name));
        contributors
    }
}

impl Serialize for ChangeSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ChangeSet", 2)?;
        state.serialize_field("changes", &self.changes())?;
        state.serialize_field("contributors", &self.contributors(None))?;
        state.end()
    }
}
