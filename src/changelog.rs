mod change;
mod changeset;
mod contributor;
mod release;

pub use self::change::Change;
pub(crate) use self::changeset::ChangeSet;
pub(crate) use self::contributor::Contributor;
pub(crate) use self::release::Release;
use crate::git::{Commit, Tag};
use crate::render;
use crate::{Config, Error};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Changelog<'a> {
    config: &'a Config,
    unreleased: ChangeSet<'a>,
    releases: Vec<Release<'a>>,
}

impl<'a> Changelog<'a> {
    pub fn new(config: &'a Config, commits: &'a [Commit], tags: Vec<Tag>) -> Result<Self, Error> {
        let mut offset = 0;

        let mut releases = tags
            .into_iter()
            .map(Release::new)
            .collect::<Result<Vec<_>, _>>()?;

        for release in &mut releases {
            let mut changeset = ChangeSet::new(&config.contributor_footers);
            offset = changeset.take_commits(
                offset,
                commits,
                &config.accept_types,
                Some(release.tag()),
            )?;
            release.with_changeset(changeset);
        }

        releases.reverse();

        let mut unreleased = ChangeSet::new(&config.contributor_footers);
        unreleased.take_commits(offset, commits, &config.accept_types, None)?;

        Ok(Self {
            config,
            releases,
            unreleased,
        })
    }

    /// Get the releases.
    pub fn releases(&self) -> impl Iterator<Item = &Release<'a>> {
        self.releases.iter()
    }

    /// Get the unreleased changes.
    pub fn unreleased(&self) -> &ChangeSet<'a> {
        &self.unreleased
    }

    pub fn render(&self, include_metadata: bool) -> Result<String, Error> {
        let context = tera::Context::from_serialize(self)?;
        let mut tera = tera::Tera::default();
        let template = self
            .config
            .template
            .as_deref()
            .unwrap_or(include_str!("../template.md"));

        let type_header = render::TypeHeader(self.config.type_headers.clone());
        let scope_header = render::ScopeHeader(self.config.scope_headers.clone());

        tera.add_raw_template("template", template)?;
        tera.register_filter("indent", render::indent);
        tera.register_filter("typeheader", type_header);
        tera.register_filter("scopeheader", scope_header);

        let mut log = tera.render("template", &context)?;
        if include_metadata {
            if let Some(metadata) = &self.config.metadata {
                log.push_str(&format!("\n\n{}\n", metadata));
            }
        }

        Ok(log)
    }
}
