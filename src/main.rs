use std::{convert::TryInto as _, env, io::Write as _, process::Command};

use jilu::{
    changelog::Change,
    git::{self, Tag},
    Changelog, Config, Error,
};
use semver::Version;

fn main() {
    match run() {
        Ok(log) => print!("{}", log),
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    }
}

fn run() -> Result<String, Error> {
    let repo = git2::Repository::open(".")?;
    let config = Config::from_environment(&repo)?;
    let commits = git::commits(&repo)?;
    let mut tags = git::tags(&repo)?;

    if let Ok(version) = env::var("RELEASE") {
        let log = Changelog::new(&config, &commits, tags.clone())?;
        let tag = tag_unreleased(&repo, version, log.unreleased().changes())?;
        tags.push(tag);
        tags.sort_by(|a, b| a.version.cmp(&b.version));
    }

    Changelog::new(&config, &commits, tags)?.render()
}

/// Group all unreleased commits into a new release.
///
/// This does **NOT** create a tag, but instead returns a "fake" `Tag`, which
/// allows the change log template to render the unreleased commits as part of
/// that tag.
///
/// Once the change log is rendered, the user can create a commit with the
/// updated change log, and then create a tag with the same version and release
/// notes as the one used for the unreleased commits.
fn tag_unreleased(
    repo: &git2::Repository,
    version: String,
    changes: &[Change],
) -> Result<Tag, Error> {
    let version = Version::parse(version.strip_prefix('v').unwrap_or(&version))?;
    let notes = env::var("RELEASE_NOTES")
        .map(|v| v.replace("\\n", "\n"))
        .ok();
    let edit = env::var("RELEASE_EDIT").is_ok_and(|v| !v.is_empty());
    let mut instructions = format!(
        r#"
#
# Write a message for release:
#   v{}
#
# - The first line is the release title.
# - Subsequent lines are the release notes.
# - This comment will be stripped from the release notes."#,
        version
    );

    if !changes.is_empty() {
        instructions.push_str("\n#\n# CHANGES:\n#");
        for change in changes {
            instructions.push_str(&format!("\n# {change:#}"));
        }
    }

    let mut message = notes.unwrap_or_default();
    if edit {
        message.push_str("\n\n");
        message.push_str(&instructions);

        let mut file = tempfile::NamedTempFile::new()?;
        file.write_all(message.as_bytes())?;
        Command::new(git::editor(repo)).arg(file.path()).status()?;

        let buf = std::fs::read_to_string(file.path())?;
        message = buf
            .trim()
            .strip_suffix(&instructions)
            .unwrap_or(&buf)
            .to_owned();
    }

    Ok(Tag {
        name: format!("v{}", version),
        message: Some(message),
        version,
        tagger: repo.signature()?.try_into().ok(),
        commit: repo.head()?.peel_to_commit()?.try_into()?,
    })
}
