use crate::changelog::ChangeSet;
use crate::git::Tag;
use crate::Error;
use chrono::{offset::Utc, DateTime};
use semver::Version;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

#[derive(Debug)]
pub(crate) struct Release<'a> {
    /// The version of the release.
    version: Version,

    /// Internal reference to the Git tag of this release.
    tag: Tag,

    /// Internal reference to the change set of this release.
    changeset: ChangeSet<'a>,
}

impl Serialize for Release<'_> {
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

impl<'a> Release<'a> {
    pub(crate) fn new(tag: Tag) -> Result<Self, Error> {
        let version = if tag.name.starts_with('v') {
            &tag.name[1..]
        } else {
            &tag.name
        };

        let version = Version::parse(version)?;

        Ok(Self {
            version,
            tag,
            changeset: ChangeSet::default(),
        })
    }

    /// Add a set of changes to the release.
    pub(crate) fn with_changeset(&mut self, changeset: ChangeSet<'a>) {
        self.changeset = changeset;
    }

    /// The SemVer version of the release.
    pub(crate) fn version(&self) -> &Version {
        &self.version
    }

    /// The title of the release.
    ///
    /// This is similar to the _first line_ of the Git tag annotated message.
    ///
    /// If a lightweight tag was used to tag the release, it will have no
    /// title.
    pub(crate) fn title(&self) -> Option<&str> {
        self.tag.message.as_ref().and_then(|m| m.lines().next())
    }

    /// The release notes.
    ///
    /// This is similar to all lines _after_ the first line, and _before_ the
    /// PGP signature of the Git tag annotated message.
    ///
    /// If a lightweight tag was used to tag the release, it will have no
    /// notes.
    pub(crate) fn notes(&self) -> Option<&str> {
        self.tag.message.as_ref().and_then(|msg| {
            let begin = msg.find('\n').unwrap_or(0);
            let end = msg.find("-----BEGIN").unwrap_or_else(|| msg.len()) - 1;

            msg.get(begin..=end).map(str::trim)
        })
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
    pub(crate) fn date(&self) -> DateTime<Utc> {
        self.tag
            .tagger
            .as_ref()
            .map(|t| t.time)
            .unwrap_or_else(|| self.tag.commit.time)
    }

    /// The Git tag belonging to the release.
    pub(crate) fn tag(&self) -> &Tag {
        &self.tag
    }

    /// The change set belonging to the release.
    pub(crate) fn changeset(&self) -> &ChangeSet {
        &self.changeset
    }
}
