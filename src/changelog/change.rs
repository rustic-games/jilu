use crate::changelog::Contributor;
use crate::git::Commit;
use crate::Error;
use conventional::{Commit as CCommit, Simple as _};
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::collections::HashMap;
use std::fmt;
use tera::Value;

/// A single change in the change log.
#[derive(Debug)]
pub struct Change<'a> {
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

    /// The author details of this change.
    pub(crate) fn author(&self) -> Contributor {
        (
            self.commit.author.name.as_str(),
            self.commit.author.email.as_str(),
        )
            .into()
    }

    /// The committer details of this change.
    pub(crate) fn committer(&self) -> Contributor {
        (
            self.commit.committer.name.as_str(),
            self.commit.committer.email.as_str(),
        )
            .into()
    }

    /// The list of contributors for this change.
    ///
    /// This includes the author and committer of the change, as well as any
    /// contributors listed in the footers of the commit.
    pub(crate) fn contributors(&self, contributor_footers: &[String]) -> Vec<Contributor> {
        let mut contributors: Vec<_> = self
            .conventional
            .footers()
            .iter()
            .filter(|f| contributor_footers.contains(&f.token().to_ascii_lowercase()))
            .filter_map(|f| parse_contributor_footer(f.value()))
            .chain([self.author(), self.committer()])
            .collect();

        contributors.sort_unstable();
        contributors.dedup();
        contributors
    }
}

/// Best-effort parsing of a contributor from a commit footer.
fn parse_contributor_footer(value: &str) -> Option<Contributor> {
    let (name, email) = value.rsplit_once('<').unwrap_or((value, ""));
    let email = email.rsplit_once('>').unwrap_or((email, "")).0;

    Some((name, email).into())
}

impl fmt::Display for Change<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            self.conventional
                .to_string()
                .lines()
                .next()
                .unwrap_or_default()
                .fmt(f)
        } else {
            write!(f, "{}", self.conventional)
        }
    }
}

impl Serialize for Change<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut count = 5;

        let scope = self.scope().inspect(|_| count += 1);
        let body = self.body().inspect(|_| count += 1);
        let commit = HashMap::from([("id", self.id()), ("short_id", self.short_id())]);
        let merge_commit = self.merge_commit_description().map(|c| {
            count += 1;
            HashMap::<_, Value>::from([
                ("description", c.description().into()),
                ("pr_number", c.pr_number().into()),
            ])
        });

        let mut state = serializer.serialize_struct("Change", count)?;
        state.serialize_field("type", self.type_())?;
        state.serialize_field("description", self.description())?;
        state.serialize_field("commit", &commit)?;
        state.serialize_field("author", &self.author())?;
        state.serialize_field("committer", &self.committer())?;

        if let Some(scope) = scope {
            state.serialize_field("scope", &scope)?;
        }
        if let Some(merge_commit) = merge_commit {
            state.serialize_field("merge_commit_description", &merge_commit)?;
        }
        if let Some(body) = body {
            state.serialize_field("body", &body)?;
        }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_contributor_footer() {
        #[rustfmt::skip]
        let cases = [
            ("John", Some(("John", ""))),
            ("John Doe", Some(("John Doe", ""))),
            ("john@doe.com", Some(("john@doe.com", ""))),
            ("John Doe <john@doe.com>", Some(("John Doe", "john@doe.com"))),
        ];

        for (value, expected) in cases {
            assert_eq!(parse_contributor_footer(value), expected.map(Into::into));
        }
    }
}
