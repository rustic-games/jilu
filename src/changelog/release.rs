use crate::changelog::ChangeSet;
use crate::git::Tag;
use crate::Error;
use chrono::{offset::Utc, DateTime};
use semver::Version;
use serde::ser::{SerializeStruct, Serializer};
use serde::Serialize;

#[derive(Debug)]
pub struct Release<'a> {
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
        if let Some(subject) = self.subject() {
            state.serialize_field("subject", &subject)?;
        }
        if let Some(notes) = self.notes() {
            state.serialize_field("notes", &notes)?;
        }
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
    pub fn version(&self) -> &Version {
        &self.version
    }

    /// The subject of the release.
    ///
    /// This is similar to the _first line_ of the Git tag annotated message.
    ///
    /// Note that a tag with multiple lines *MUST* have an empty line between
    /// the subject and the body, otherwise the tag is considered to only have a
    /// body, but not a subject.
    ///
    /// If a lightweight tag was used to tag the release, it will have no
    /// subject.
    pub(crate) fn subject(&self) -> Option<&str> {
        self.tag
            .message
            .as_ref()
            .filter(|m| !m.trim().is_empty())
            .and_then(|m| {
                let mut lines = m.lines();
                let first = lines.next()?;
                if lines
                    .next()
                    // If there is no second line, or the second line is empty
                    // and there is a third line which is not empty, then the
                    // first line is the subject.
                    .is_none_or(|v| v.is_empty() && lines.next().is_some_and(|v| !v.is_empty()))
                {
                    return Some(first);
                }

                None
            })
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
            let begin = self.subject().and_then(|_| msg.find('\n')).unwrap_or(0);
            let end = msg
                .find("-----BEGIN")
                .unwrap_or(msg.len())
                .saturating_sub(1);

            msg.get(begin..end).map(str::trim)
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
    pub fn tag(&self) -> &Tag {
        &self.tag
    }

    /// The change set belonging to the release.
    pub(crate) fn changeset(&self) -> &ChangeSet {
        &self.changeset
    }
}
