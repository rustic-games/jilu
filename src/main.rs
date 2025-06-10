use std::{convert::TryInto as _, env, io::Write as _, process::Command};

use jaq_core::load;
use jilu::{
    changelog::Change,
    git::{self, Tag},
    Changelog, Config, Error,
};
use semver::Version;
use serde_json::Value;

fn main() {
    let opts = match Opts::parse() {
        Ok(opts) => opts,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    let file = match opts.output_file.as_deref() {
        None => None,
        Some(file) => match std::fs::OpenOptions::new().write(true).open(file) {
            Ok(file) => Some(file),
            Err(err) => {
                eprintln!("Cannot open output file {}: {}", file, err);
                std::process::exit(1);
            }
        },
    };

    match (run(opts), file) {
        (Ok(log), None) => print!("{}", log),
        (Ok(log), Some(mut file)) => {
            if let Err(err) = file.write_all(log.as_bytes()) {
                eprintln!("Cannot write to output file: {}", err);
                std::process::exit(1);
            }
        }
        (Err(err), _) => {
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

    /// If set, the change log will be rendered without inline configuration.
    strip_config: bool,

    /// Output the release notes either in `text` or `json` format. Defaults to
    /// `text`, unless `write` is set, in which case it defaults to `none`.
    output: Option<String>,

    /// If set, the output will be written to the file instead of printed to
    /// `stdout`.
    output_file: Option<String>,

    /// Version to use for the unreleased changes.
    release: Option<String>,

    /// Optional release notes for the unreleased changes.
    release_notes: Option<String>,

    /// Edit the release notes in `$EDITOR`.
    edit_release_notes: bool,

    /// Optional `jq` query filter to apply to the JSON output.
    jq: Option<String>,
}

impl Opts {
    fn parse() -> Result<Self, Error> {
        use lexopt::{Arg::*, ValueExt as _};

        let mut write = false;
        let mut strip_config = false;
        let mut output = None;
        let mut output_file = None;
        let mut file = None;
        let mut jq = None;
        let mut release = None;
        let mut release_notes = None;
        let mut edit_release_notes = false;

        let mut parser = lexopt::Parser::from_env();
        while let Some(arg) = parser.next()? {
            match arg {
                Short('w') | Long("write") => {
                    write = true;
                }
                Short('o') | Long("output") => {
                    output = match parser.value()?.parse()? {
                        v if v == "text" => Some(v),
                        v if v == "json" => Some(v),
                        v if v == "none" => Some(v),
                        _ => None,
                    };
                }
                Short('f') | Long("output-file") => {
                    output_file = Some(parser.value()?.string()?);
                }
                Short('r') | Long("release") => {
                    release = Some(parser.value()?.parse()?);
                }
                Short('n') | Long("notes") => {
                    release_notes = Some(parser.value()?.parse()?);
                }
                Short('q') | Long("jq") => {
                    jq = Some(parser.value()?.parse()?);
                }
                Short('e') | Long("edit") => {
                    edit_release_notes = true;
                }
                Long("strip-config") => {
                    strip_config = true;
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
        if !write && output.is_none() {
            output = Some("text".to_owned());
        } else if write && output.is_some() && output_file.is_none() {
            Err(lexopt::Error::from(
                "Using --write and --output together requires --output-file.",
            ))?;
        }

        Ok(Self {
            file,
            write,
            strip_config,
            output,
            output_file,
            release,
            release_notes,
            edit_release_notes,
            jq,
        })
    }
}

fn run(opts: Opts) -> Result<String, Error> {
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

    let log = Changelog::new(&config, &commits, tags)?;

    if opts.write {
        std::fs::write(&opts.file, log.render(!opts.strip_config)?)?;
    }

    match (opts.output.as_deref(), opts.jq.as_deref()) {
        (Some("text"), _) => Ok(log.render(!opts.strip_config)?),
        (Some("json"), None) => Ok(serde_json::to_string(&log)?),
        (Some("json"), Some(code)) => {
            let json = serde_json::to_value(&log)?;
            let program = load::File { code, path: () };
            let loader = load::Loader::new(jaq_std::defs().chain(jaq_json::defs()));
            let arena = load::Arena::default();
            let modules = loader.load(&arena, program)?;
            let filter = jaq_core::Compiler::default()
                .with_funs(
                    jaq_std::funs()
                        .chain(jaq_json::funs())
                        .chain(jq_raw().into_vec().into_iter().map(jaq_std::run)),
                )
                .compile(modules)?;
            let inputs = jaq_core::RcIter::new(core::iter::empty());
            let out = filter.run((jaq_core::Ctx::new([], &inputs), jaq_json::Val::from(json)));

            let value = out
                .map(|v| v.map(Value::from))
                .collect::<Result<Vec<_>, jaq_core::Error<jaq_json::Val>>>()?;

            let value = if value.len() <= 1 {
                value.into_iter().next().unwrap_or(Value::Null)
            } else {
                value.into()
            };

            match value {
                Value::String(s) if s.starts_with("$$special::raw$$") => Ok(s
                    .strip_prefix("$$special::raw$$")
                    .unwrap_or_default()
                    .to_string()),
                _ => Ok(serde_json::to_string(&value)?),
            }
        }
        _ => Ok(String::new()),
    }
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

/// Mark a JSON string as "raw".
///
/// This is used when printing JSON values to stdout, to allow printing JSON
/// strings without surrounding quotes.
///
/// This is similar to Jq's `-r` or `--raw-output` flag:
///
/// ```sh
/// jilu --output json --jq '.config.github.repo'       # => "rust-lang/jilu"
/// jilu --output json --jq '.config.github.repo | raw' # =>  rust-lang/jilu
/// ```
///
/// Note that this uses an internal marker to signal that the string should be
/// printed without quotes. This means using this function anywhere other than
/// as the last filter in a chain will result in unexpected output:
///
/// ```sh
/// jilu --output json --jq '.config.github | map(raw)' # => ["$$special::raw$$rustic-games/jilu"]
/// ```
fn jq_raw() -> Box<[jaq_std::Filter<jaq_core::RunPtr<jaq_json::Val>>]> {
    use jaq_core::box_iter::box_once;
    use jaq_std::v;

    Box::new([("raw", v(0), |_, cv| {
        box_once(Ok(match cv.1 {
            jaq_json::Val::Str(v) => format!("$$special::raw$${v}").into(),
            _ => cv.1.to_string().into(),
        }))
    })])
}
