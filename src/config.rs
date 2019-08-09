use git2::Oid;
use std::collections::HashMap;

const TITLE: &str = "Changelog";
const DESCRIPTION: &str = "\
All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning]. The file is auto-generated using [Conventional Commits].

[keep a changelog]: https://keepachangelog.com/en/1.0.0/
[semantic versioning]: https://semver.org/spec/v2.0.0.html
[conventional commits]: https://www.conventionalcommits.org/en/v1.0.0-beta.4/\
";
const THANK_YOU: &str = "\
This release was made possible by the following people (in alphabetical order).
Thank you all for your contributions. Your work – no matter how significant – is
greatly appreciated by the community. 💖\
";

#[derive(Debug)]
pub struct Config<'a> {
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub toc: bool,
    pub root: Option<Oid>,
    pub unreleased: bool,
    pub contributors: Contributors<'a>,
    pub types: HashMap<&'a str, &'a str>,
    pub ignore: Ignore<'a>,
    pub commit: Commit,
    pub github: Github<'a>,
}

impl<'a> Default for Config<'a> {
    fn default() -> Self {
        let mut types = HashMap::new();
        types.insert("fix", "bug fixes");
        types.insert("style", "code styling");
        types.insert("docs", "documentation");
        types.insert("feat", "features");
        types.insert("perf", "performance improvements");
        types.insert("refactor", "refactoring");
        types.insert("test", "tests");

        // FIXME: TESTING DATA
        // let github = Github {
        //     repo: Some("https://github.com/rustic-games/prototype"),
        // };
        let github = Github::default();

        Self {
            title: TITLE,
            description: Some(DESCRIPTION),
            toc: true,
            root: None,
            unreleased: true,
            contributors: Contributors::default(),
            types,
            ignore: Ignore::default(),
            commit: Commit::default(),
            github,
        }
    }
}

#[derive(Debug, Default)]
pub struct Ignore<'a> {
    types: Vec<&'a str>,
    commits: Vec<Oid>,
}

#[derive(Debug)]
pub struct Commit {
    pub body: bool,
}

impl<'a> Default for Commit {
    fn default() -> Self {
        Self { body: true }
    }
}

#[derive(Debug)]
pub struct Contributors<'a> {
    pub show: bool,
    pub thank_you: Option<&'a str>,
    pub ignore: Vec<&'a str>,
}

impl<'a> Default for Contributors<'a> {
    fn default() -> Self {
        Self {
            show: true,
            thank_you: Some(THANK_YOU),
            ignore: vec![],
        }
    }
}

#[derive(Debug, Default)]
pub struct Github<'a> {
    pub repo: Option<&'a str>,
}
