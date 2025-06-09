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

struct Opts {
    /// The change log file. Defaults to `CHANGELOG.md`.
    file: String,

    /// If set, the change log will be written to the file instead of printed to
    /// `stdout`.
    write: bool,

    /// Version to use for the unreleased changes.
    release: Option<String>,

    /// Optional release notes for the unreleased changes.
    release_notes: Option<String>,

    /// Edit the release notes in `$EDITOR`.
    edit_release_notes: bool,
}

impl Opts {
    fn parse() -> Result<Self, Error> {
        use lexopt::{Arg::*, ValueExt as _};

        let mut write = false;
        let mut file = None;
        let mut release = None;
        let mut release_notes = None;
        let mut edit_release_notes = false;

        let mut parser = lexopt::Parser::from_env();
        while let Some(arg) = parser.next()? {
            match arg {
                Short('w') | Long("write") => {
                    write = true;
                }
                Short('r') | Long("release") => {
                    release = Some(parser.value()?.parse()?);
                }
                Short('n') | Long("notes") => {
                    release_notes = Some(parser.value()?.parse()?);
                }
                Short('e') | Long("edit") => {
                    edit_release_notes = true;
                }
                Short('h') | Long("help") => {
                    println!("Usage: jilu [-r|--release=VERSION] [-n|--notes=RELEASE_NOTES] [-e|--edit] [-w|--write] [CHANGELOG]");
                    std::process::exit(0);
                }
                Value(v) if file.is_none() => {
                    file = Some(v.string()?);
                }
                _ => return Err(arg.unexpected().into()),
            }
        }

        let file = file
            .or_else(|| env::var("CHANGELOG").ok())
            .unwrap_or_else(|| "CHANGELOG.md".to_owned());

        let write = write || env::var("WRITE_CHANGELOG").is_ok();
        let release = release.or_else(|| env::var("RELEASE").ok());
        let release_notes = release_notes.or_else(|| env::var("RELEASE_NOTES").ok());
        let edit_release_notes = edit_release_notes || env::var("RELEASE_EDIT").is_ok();

        Ok(Self {
            file,
            write,
            release,
            release_notes,
            edit_release_notes,
        })
    }
}

fn run() -> Result<String, Error> {
    let opts = Opts::parse()?;

    let repo = git2::Repository::open(".")?;
    let config = Config::from_environment(&repo, &opts.file)?;
    let commits = git::commits(&repo)?;
    let mut tags = git::tags(&repo)?;

    if let Some(version) = opts.release {
        let log = Changelog::new(&config, &commits, tags.clone())?;
        let tag = tag_unreleased(
            &repo,
            version,
            opts.release_notes,
            opts.edit_release_notes,
            log.unreleased().changes(),
        )?;
        tags.push(tag);
        tags.sort_by(|a, b| a.version.cmp(&b.version));
    }

    let out = Changelog::new(&config, &commits, tags)?.render()?;
    Ok(if opts.write {
        std::fs::write(opts.file, out)?;
        String::new()
    } else {
        out
    })
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
    notes: Option<String>,
    edit: bool,
    changes: &[Change],
) -> Result<Tag, Error> {
    let version = Version::parse(version.strip_prefix('v').unwrap_or(&version))?;
    let notes = notes.map(|v| v.replace("\\n", "\n"));
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
