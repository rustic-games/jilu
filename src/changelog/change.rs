use crate::changelog::Contributor;
use crate::git::Commit;
use crate::Error;
use conventional::{Commit as CCommit, Simple as _};
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::collections::HashMap;
use tera::Value;

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
        self.conventional.type_()
    }

    /// The scope of the change.
    pub(crate) fn scope(&self) -> Option<&str> {
        self.conventional.scope()
    }

    /// The short description of the change.
    pub(crate) fn description(&self) -> &str {
        self.conventional.description()
    }

    /// The description of a Github merge commit, including the PR number, if
    /// any.
    pub(crate) fn merge_commit_description(&self) -> Option<GithubMergeCommit> {
        GithubMergeCommit::new(self.description())
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

        let merge_commit = self.merge_commit_description().map(|c| {
            let mut map: HashMap<&str, Value> = HashMap::new();
            map.insert("description", c.description().into());
            map.insert("pr_number", c.pr_number().into());
            map
        });

        let mut state = serializer.serialize_struct("Change", 4)?;
        state.serialize_field("type", &self.type_())?;
        state.serialize_field("scope", &self.scope())?;
        state.serialize_field("description", &self.description())?;
        state.serialize_field("merge_commit_description", &merge_commit)?;
        state.serialize_field("body", &self.body())?;
        state.serialize_field("commit", &commit)?;
        state.end()
    }
}

pub(crate) struct GithubMergeCommit<'a> {
    description: &'a str,
    pr_number: usize,
    pr_suffix_start_index: usize,
}

impl<'a> GithubMergeCommit<'a> {
    fn new(description: &'a str) -> Option<Self> {
        /// State machine for the description, walking backwards.
        enum State {
            /// `)`
            Close,
            /// `123`
            Number(usize),
            /// `#`
            Pound,
            /// `(`
            Open,
            /// ` `
            Space,
            // Stop
            Stop,
        }

        let mut index = description.len();
        let mut number: Vec<char> = vec![];
        let mut state = State::Close;
        let mut chars = description.chars().rev();
        let mut c = chars.next()?;
        loop {
            match state {
                State::Close => {
                    if c != ')' {
                        return None;
                    }

                    c = chars.next()?;
                    state = State::Number(0);
                }
                State::Number(count) => {
                    if c.is_numeric() {
                        number.insert(0, c);
                        state = State::Number(count + 1);
                        c = chars.next()?;
                    } else if count == 0 {
                        return None;
                    } else {
                        state = State::Pound;
                    }
                }
                State::Pound => {
                    if c != '#' {
                        return None;
                    }

                    c = chars.next()?;
                    state = State::Open;
                }
                State::Open => {
                    if c != '(' {
                        return None;
                    }

                    c = chars.next()?;
                    state = State::Space;
                }
                State::Space => {
                    if c != ' ' {
                        return None;
                    }

                    state = State::Stop;
                }
                State::Stop => break,
            };

            index -= 1;
        }

        Some(Self {
            description,
            pr_number: number.into_iter().collect::<String>().parse().ok()?,
            pr_suffix_start_index: index,
        })
    }

    pub(crate) fn description(&self) -> &str {
        &self.description[..=self.pr_suffix_start_index]
    }

    pub(crate) fn pr_number(&self) -> usize {
        self.pr_number
    }
}
