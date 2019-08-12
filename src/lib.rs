pub mod config;
pub mod error;
pub mod git;
mod render;

pub use config::Config;
pub use error::Error;

use chrono::{offset::Utc, DateTime};
use conventional_commit::ConventionalCommit;
use git::{Commit, Tag};
use semver::Version;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Serialize)]
pub struct Changelog {
    config: Config,
    unreleased: ChangeSet,
    releases: Vec<Release>,
}

impl Changelog {
    pub fn new(config: Config, mut commits: Vec<Commit>, tags: Vec<Tag>) -> Result<Self, Error> {
        let mut releases = tags
            .into_iter()
            .map(Release::new)
            .collect::<Result<Vec<_>, _>>()?;

        for release in &mut releases {
            let mut changeset = ChangeSet::default();
            changeset.take_commits(&mut commits, &config.accept_types, Some(release.tag()))?;
            release.with_changeset(changeset);
        }

        releases.reverse();

        let mut unreleased = ChangeSet::default();
        unreleased.take_commits(&mut commits, &config.accept_types, None)?;

        Ok(Self {
            config,
            releases,
            unreleased,
        })
    }

    pub fn render(&self) -> Result<String, Error> {
        let context = tera::Context::from_serialize(self)?;
        let mut tera = tera::Tera::default();
        let template = self
            .config
            .template
            .as_ref()
            .map(String::as_str)
            .unwrap_or(include_str!("../template.md"));

        let type_header = render::TypeHeader(self.config.type_headers.clone());

        tera.add_raw_template("template", template)?;
        tera.register_filter("indent", render::indent);
        tera.register_filter("typeheader", type_header);

        let mut log = tera.render("template", context)?;
        if let Some(metadata) = &self.config.metadata {
            log.push_str(&format!("\n{}\n", metadata));
        }

        Ok(log)
    }
}

/// A set of changes belonging together.
#[derive(Debug, Default)]
struct ChangeSet {
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
    pub fn take_commits(
        &mut self,
        commits: &mut Vec<Commit>,
        accept_types: &Option<Vec<String>>,
        tag: Option<&Tag>,
    ) -> Result<(), Error> {
        if commits.is_empty() {
            return Ok(());
        }

        let idx = match tag {
            None => Some(commits.len() - 1),
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
    pub fn changes(&self) -> &[Change] {
        &self.changes
    }

    /// A list of people who contributed to this change set.
    ///
    /// You can pass in a list of optional contributor names to ignore.
    pub fn contributors(&self, ignore: Option<&[String]>) -> Vec<Contributor> {
        let ignore = ignore.unwrap_or(&[]);
        let mut authors = HashMap::new();

        for change in &self.changes {
            let author = &change.commit.author;
            let _ = authors.insert(author.name.as_str(), author.email.as_str());
        }

        let mut contributors = authors
            .into_iter()
            .map(Into::into)
            .filter(|c: &Contributor| !ignore.iter().any(|name| name == &c.name))
            .collect::<Vec<_>>();

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

#[derive(Debug)]
struct Release {
    /// The version of the release.
    version: Version,

    /// Internal reference to the Git tag of this release.
    tag: Tag,

    /// Internal reference to the change set of this release.
    changeset: ChangeSet,
}

impl Serialize for Release {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Release", 5)?;
        state.serialize_field("version", &self.version())?;
        state.serialize_field("title", &self.title())?;
        state.serialize_field("notes", &self.notes())?;
        state.serialize_field("date", &self.date())?;
        state.serialize_field("changeset", &self.changeset())?;
        state.end()
    }
}

impl Release {
    fn new(tag: Tag) -> Result<Self, Error> {
        let version = Version::parse(&tag.name[1..])?;

        Ok(Self {
            version,
            tag,
            changeset: ChangeSet::default(),
        })
    }

    /// Add a set of changes to the release.
    pub fn with_changeset(&mut self, changeset: ChangeSet) {
        self.changeset = changeset;
    }

    /// The SemVer version of the release.
    pub fn version(&self) -> &Version {
        &self.version
    }

    /// The title of the release.
    ///
    /// This is similar to the _first line_ of the Git tag annotated message.
    ///
    /// If a lightweight tag was used to tag the release, it will have no
    /// title.
    pub fn title(&self) -> Option<&str> {
        self.tag.message.lines().next()
    }

    /// The release notes.
    ///
    /// This is similar to all lines _after_ the first line, and _before_ the
    /// PGP signature of the Git tag annotated message.
    ///
    /// If a lightweight tag was used to tag the release, it will have no
    /// notes.
    pub fn notes(&self) -> Option<&str> {
        let msg = &self.tag.message;
        let begin = msg.find('\n').unwrap_or(0);
        let end = msg.find("-----BEGIN").unwrap_or_else(|| msg.len()) - 1;

        msg.get(begin..=end).map(str::trim)
    }

    /// The release date.
    ///
    /// If an annotated tag was used to tag the release, this will be the
    /// timestamp attached to the tag. If a lightweight tag was used, this will
    /// be the timestamp of the commit to which the tag points.
    ///
    /// # Errors
    ///
    /// If an error occurs during Git operations, this method will return an
    /// error.
    ///
    /// If the time returned by Git is not a valid UNIX timestamp, an error is
    /// returned, but this is highly unlikely.
    pub fn date(&self) -> DateTime<Utc> {
        self.tag
            .tagger
            .as_ref()
            .map(|t| t.time)
            .unwrap_or_else(|| self.tag.commit.time)
    }

    /// The Git tag belonging to the release.
    pub fn tag(&self) -> &Tag {
        &self.tag
    }

    /// The change set belonging to the release.
    pub fn changeset(&self) -> &ChangeSet {
        &self.changeset
    }
}

/// The contributor to a change.
#[derive(Debug, Serialize)]
struct Contributor {
    name: String,
    email: String,
}

impl From<(&str, &str)> for Contributor {
    fn from((name, email): (&str, &str)) -> Self {
        Self {
            name: name.to_owned(),
            email: email.to_owned(),
        }
    }
}

/// A single change in the change log.
#[derive(Debug)]
struct Change {
    commit: Commit,
    conventional: ConventionalCommit,
}

impl Change {
    pub fn new(commit: Commit) -> Result<Self, Error> {
        let conventional = ConventionalCommit::from_str(&commit.message)?;

        Ok(Self {
            commit,
            conventional,
        })
    }

    /// The type of the change.
    pub fn type_(&self) -> &str {
        &self.conventional.type_()
    }

    /// The short description of the change.
    pub fn description(&self) -> &str {
        self.conventional.description()
    }

    /// The body of the change.
    pub fn body(&self) -> Option<&str> {
        self.conventional.body()
    }

    /// Get the "short Git ID"
    ///
    /// This default to 7 characters of the object ID, but is extended to avoid
    /// ambiguity.
    pub fn short_id(&self) -> &str {
        &self.commit.short_id
    }

    /// The regular Git reference.
    pub fn id(&self) -> &str {
        &self.commit.id
    }
}

impl Serialize for Change {
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
