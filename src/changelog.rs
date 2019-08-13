mod change;
mod changeset;
mod contributor;
mod release;

pub(crate) use self::change::Change;
pub(crate) use self::changeset::ChangeSet;
pub(crate) use self::contributor::Contributor;
pub(crate) use self::release::Release;
use crate::git::{Commit, Tag};
use crate::render;
use crate::{Config, Error};
use serde::Serialize;

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
