pub mod config;
pub mod error;

pub use config::Config;
pub use error::Error;

use chrono::{offset::FixedOffset, DateTime};
use conventional_commit::ConventionalCommit;
use git2::{Commit, ObjectType, Signature, Tag};
use semver::Version;
use std::collections::HashMap;
use std::fmt::Write;
use std::str::FromStr;

#[derive(Debug)]
pub struct Changelog<'a> {
    config: Config<'a>,
    unreleased: ChangeSet<'a>,
    releases: Vec<Release<'a>>,
}

impl<'a> Changelog<'a> {
    pub fn new(
        config: Config<'a>,
        mut commits: Vec<Commit<'a>>,
        tags: Vec<Tag<'a>>,
    ) -> Result<Self, Error> {
        let mut releases = tags
            .into_iter()
            .filter(|tag| tag.name().unwrap_or("").starts_with('v'))
            .filter_map(|tag| match Release::new(tag) {
                Ok(release) => release,
                Err(err) => {
                    eprintln!("error creating release: {}", err);
                    None
                }
            })
            .collect::<Vec<_>>();

        for release in &mut releases {
            let mut changeset = ChangeSet::default();
            changeset.take_commits(&mut commits, Some(release.tag()), &config.types)?;
            release.with_changeset(changeset);
        }

        releases.reverse();

        let mut unreleased = ChangeSet::default();
        unreleased.take_commits(&mut commits, None, &config.types)?;

        Ok(Self {
            config,
            releases,
            unreleased,
        })
    }

    pub fn format<W: Write>(&self, f: &mut W) -> Result<(), Error> {
        writeln!(f, "# {}\n", self.config.title)?;

        if let Some(description) = self.config.description {
            writeln!(f, "{}\n", description)?;
        }

        if self.config.toc {
            self.format_toc(f)?;
        }

        if self.config.unreleased {
            self.format_unreleased(f)?;
        }

        for r in &self.releases {
            self.format_release(f, r)?;
        }

        self.format_references(f)
    }

    fn format_toc<W: Write>(&self, f: &mut W) -> Result<(), Error> {
        f.write_str("## Overview\n\n")?;
        f.write_str("- [_unreleased_](#unreleased)\n")?;

        for r in &self.releases {
            let date = r.date()?.format("%Y.%m.%d");
            let mut tag = r.version().to_string();
            tag.retain(|c| !".".contains(c));

            writeln!(f, "- [**`{}`**](#{}) – _{}_", r.version(), tag, date)?;
        }

        f.write_str("\n").map_err(Into::into)
    }

    fn format_unreleased<W: Write>(&self, f: &mut W) -> Result<(), Error> {
        f.write_str("## _[Unreleased]_\n\n")?;

        if self.unreleased.changes().is_empty() {
            f.write_str("_nothing new to show for… yet!_\n\n")?;
        } else {
            for (_ty, changes) in self.unreleased.changes() {
                for change in &changes {
                    writeln!(f, "- **{}** ([`{}`])", change.title(), change.short_id()?,)?;
                }
            }
        }

        f.write_str("\n").map_err(Into::into)
    }

    fn format_release<W: Write>(&self, f: &mut W, r: &Release<'a>) -> Result<(), Error> {
        use inflector::cases::titlecase::to_title_case;

        // details
        write!(f, "## [{}]", r.version())?;
        if let Some(title) = r.title() {
            write!(f, " – _{}_", title)?;
        }
        f.write_str("\n\n")?;

        writeln!(f, "_{}_\n", r.date()?.format("%Y.%m.%d"))?;

        // description
        if let Some(notes) = r.notes() {
            writeln!(f, "{}\n", notes)?;
        }

        // contributors
        if self.config.contributors.show {
            let ignore = &self.config.contributors.ignore;
            let contributors = r.changeset().contributors(Some(ignore));

            if !contributors.is_empty() {
                f.write_str("### Contributions\n\n")?;

                if let Some(thank_you) = self.config.contributors.thank_you {
                    writeln!(f, "{}\n", thank_you)?;
                }

                for contributor in r.changeset().contributors(None) {
                    write!(f, "- {}", contributor.name())?;
                    if let Some(email) = contributor.email() {
                        write!(f, " \\(<{}>\\)", email)?;
                    }

                    f.write_str("\n")?;
                }

                f.write_str("\n")?;
            }
        }

        // changes
        f.write_str("### Changes\n\n")?;

        let mut types = vec![];
        for (ty, changes) in r.changeset().changes() {
            types.push(ty);

            writeln!(f, "#### {}\n", to_title_case(&ty.to_string()))?;

            for change in &changes {
                writeln!(f, "- **{}** ([`{}`])", change.title(), change.short_id()?,)?;

                if self.config.commit.body {
                    if let Some(body) = change.body() {
                        f.write_str("\n")?;

                        let mut result = String::new();
                        for line in body.lines() {
                            if line.chars().any(|c| !c.is_whitespace()) {
                                result.push_str("  ");
                                result.push_str(line);
                            }
                            result.push('\n');
                        }

                        write!(f, "{}", result)?;
                    }
                }

                f.write_str("\n")?;
            }
        }

        // missing
        let missing = self
            .config
            .types
            .values()
            .filter(|v| !types.contains(v))
            .copied()
            .collect::<Vec<_>>()
            .join(", ");

        if !missing.is_empty() {
            f.write_str("#### _Unchanged_\n\n")?;

            writeln!(
                f,
                "_The following categories contain no changes in this release:\n{}_.\n",
                missing
            )?;
        };

        Ok(())
    }

    fn format_references<W: Write>(&self, f: &mut W) -> Result<(), Error> {
        f.write_str("<!-- [releases] -->\n\n")?;

        if self.config.unreleased {
            if let Some(repo) = self.config.github.repo {
                if let Some(tag) = self.releases.first().map(|r| r.tag()) {
                    let tag = tag.name().unwrap_or("");

                    writeln!(f, "[unreleased]: {}/compare/{}...HEAD", repo, tag)?;
                } else {
                    writeln!(f, "[unreleased]: {}/commits", repo)?;
                }
            } else {
                f.write_str("[unreleased]: #\n")?;
            }
        }

        for r in &self.releases {
            if let Some(repo) = self.config.github.repo {
                let tag = r.tag().name().unwrap_or("");

                writeln!(f, "[{}]: {}/releases/tag/{}", r.version(), repo, tag)?;
            } else {
                f.write_str("[unreleased]: #\n")?;
            }
        }

        f.write_str("\n<!-- [commits] -->\n\n")?;

        for (_ty, changes) in self.unreleased.changes() {
            for change in changes {
                let id = change.short_id()?;
                if let Some(repo) = self.config.github.repo {
                    writeln!(f, "[`{}`]: {}/commit/{}", id, repo, change.id())?;
                } else {
                    writeln!(f, "[`{}`]: #", id)?;
                };
            }
        }

        for r in &self.releases {
            for (_ty, changes) in r.changeset.changes() {
                for change in changes {
                    let id = change.short_id()?;
                    if let Some(repo) = self.config.github.repo {
                        writeln!(f, "[`{}`]: {}/commit/{}", id, repo, change.id())?;
                    } else {
                        writeln!(f, "[`{}`]: #", id)?;
                    };
                }
            }
        }

        Ok(())
    }
}

/// A set of changes belonging together.
#[derive(Debug, Default)]
struct ChangeSet<'repo> {
    /// Internal reference to the changes in this change set.
    changes: Vec<Change<'repo>>,
}

impl<'repo> ChangeSet<'repo> {
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
        commits: &mut Vec<Commit<'repo>>,
        tag: Option<&Tag>,
        types: &HashMap<&str, &str>,
    ) -> Result<(), Error> {
        match tag {
            None => {
                let mut changes = commits
                    .drain(0..)
                    .filter_map(|commit| match Change::new(commit, &types) {
                        Err(Error::InvalidCommitType) => None,
                        Err(err) => Some(Err(err)),
                        Ok(change) => Some(Ok(change)),
                    })
                    .collect::<Result<Vec<_>, _>>()?;

                self.changes.append(&mut changes)
            }
            Some(tag) => {
                let mut target = Some(
                    tag.target()?
                        .into_commit()
                        .expect("invariant: tag always points to commit"),
                );

                while let Some(commit) = target {
                    if let Some(idx) = commits.iter().position(|c| c.id() == commit.id()) {
                        let commit = commits.remove(idx);

                        match Change::new(commit, &types) {
                            Err(Error::InvalidCommitType) => {}
                            Err(err) => return Err(err),
                            Ok(change) => self.changes.push(change),
                        };
                    };

                    // FIXME: we only use the first parent, which won't always work.
                    target = commit.parents().next();
                }
            }
        }

        Ok(())
    }

    /// Return a list of changes, grouped by the change type.
    ///
    /// Any commit not adhering to the conventional commit standard is ignored
    /// in this list.
    ///
    /// Similarly, any conventional commit with a type that is not configured is
    /// ignored.
    pub fn changes(&self) -> HashMap<&str, Vec<&Change>> {
        let mut changes = HashMap::new();

        for change in &self.changes {
            let entry = changes
                .entry(change.type_header())
                .or_insert_with(|| vec![]);
            entry.push(change);
        }

        changes
    }

    /// A list of people who contributed to this change set.
    ///
    /// You can pass in a list of optional contributor names to ignore.
    pub fn contributors(&self, ignore: Option<&[&str]>) -> Vec<Contributor> {
        let ignore = ignore.unwrap_or(&[]);
        let mut authors = HashMap::new();

        for change in &self.changes {
            let author = change.commit.author();

            // Get the author name, if it contains valid UTF-8.
            //
            // TODO: add debug logging.
            let name = match author.name() {
                None => continue,
                Some(name) => name.to_owned(),
            };

            let _ = authors.insert(name, author.email().map(str::to_owned));
        }

        let mut contributors = authors
            .into_iter()
            .map(Into::into)
            .filter(|c: &Contributor| !ignore.contains(&c.name()))
            .collect::<Vec<_>>();

        contributors.sort_unstable_by(|a, b| a.name.cmp(&b.name));
        contributors
    }
}

#[derive(Debug)]
struct Release<'repo> {
    /// The version of the release.
    version: Version,

    /// Internal reference to the Git tag of this release.
    tag: Tag<'repo>,

    /// Internal reference to the change set of this release.
    changeset: ChangeSet<'repo>,
    // Internal reference to the commits of this release.
    // commits: Vec<Commit<'repo>>,
}

impl<'repo> Release<'repo> {
    fn new(tag: Tag<'repo>) -> Result<Option<Self>, Error> {
        // Ignore tags that aren't pointing to a commit object.
        //
        // TODO: add debug logging.
        if tag.target_type() != Some(ObjectType::Commit) {
            return Ok(None);
        }

        // Ignore tags with invalid UTF-8 encoding.
        //
        // TODO: add debug logging.
        let name = match tag.name() {
            Some(name) => name,
            None => return Ok(None),
        };

        // Ignore tags that don't start with `v`.
        //
        // TODO: add debug logging.
        if !name.starts_with('v') {
            return Ok(None);
        }

        let version = Version::parse(&name[1..])?;

        Ok(Some(Self {
            version,
            tag,
            changeset: ChangeSet::default(),
        }))
    }

    /// Add a set of changes to the release.
    pub fn with_changeset(&mut self, changeset: ChangeSet<'repo>) {
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
        self.tag.message().and_then(|m| m.lines().next())
    }

    /// The release notes.
    ///
    /// This is similar to all lines _after_ the first line, and _before_ the
    /// PGP signature of the Git tag annotated message.
    ///
    /// If a lightweight tag was used to tag the release, it will have no
    /// notes.
    pub fn notes(&self) -> Option<&str> {
        match self.tag.message() {
            None => None,
            Some(msg) => {
                let begin = msg.find('\n').unwrap_or(0);
                let end = msg.find("-----BEGIN").unwrap_or_else(|| msg.len()) - 1;

                msg.get(begin..=end).map(str::trim)
            }
        }
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
    pub fn date(&self) -> Result<DateTime<FixedOffset>, Error> {
        let time = match self.tag.tagger().as_ref().map(Signature::when) {
            Some(time) => time,
            None => self
                .tag
                .target()?
                .as_commit()
                .expect("invariant: tag always points to commit")
                .time(),
        };

        let offset = FixedOffset::east(time.offset_minutes() * 60);
        let date = chrono::NaiveDateTime::parse_from_str(&time.seconds().to_string(), "%s")?;

        Ok(chrono::DateTime::from_utc(date, offset))
    }

    /// The Git tag belonging to the release.
    pub fn tag(&self) -> &Tag<'repo> {
        &self.tag
    }

    /// The change set belonging to the release.
    pub fn changeset(&self) -> &ChangeSet<'repo> {
        &self.changeset
    }
}

/// The contributor to a change.
#[derive(Debug)]
struct Contributor {
    name: String,
    email: Option<String>,
}

impl Contributor {
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn email(&self) -> Option<&str> {
        self.email.as_ref().map(String::as_str)
    }
}

impl From<(String, Option<String>)> for Contributor {
    fn from((name, email): (String, Option<String>)) -> Self {
        Self { name, email }
    }
}

/// A single change in the change log.
#[derive(Debug)]
struct Change<'repo> {
    commit: Commit<'repo>,
    conventional: ConventionalCommit,
    ty: String,
}

impl<'repo> Change<'repo> {
    pub fn new(commit: Commit<'repo>, types: &HashMap<&str, &str>) -> Result<Self, Error> {
        let msg = commit.message().ok_or(Error::MissingCommitMessage)?;
        let conventional = ConventionalCommit::from_str(msg)?;

        let ty = types
            .iter()
            .find_map(|(k, v)| {
                if *k == conventional.type_() {
                    Some(v)
                } else {
                    None
                }
            })
            .ok_or(Error::InvalidCommitType)?
            .to_string();

        Ok(Self {
            commit,
            conventional,
            ty,
        })
    }

    /// The type of the change.
    pub fn type_(&self) -> &str {
        &self.conventional.type_()
    }

    /// The long-form header-style name of the type of the change.
    pub fn type_header(&self) -> &str {
        &self.ty
    }

    /// The title of the change.
    pub fn title(&self) -> &str {
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
    pub fn short_id(&self) -> Result<String, Error> {
        self.commit
            .as_object()
            .short_id()
            .map(|id| {
                id.as_str()
                    .map(str::to_owned)
                    .unwrap_or_else(|| "".to_owned())
            })
            .map_err(Into::into)
    }

    /// The regular Git reference.
    pub fn id(&self) -> String {
        self.commit.id().to_string()
    }
}
