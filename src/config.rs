use crate::{git, Error};
use git2::Repository;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::io;
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub github: Option<Github>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub type_headers: HashMap<String, String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub scope_headers: HashMap<String, String>,

    /// The root commit to start the change log from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_commit: Option<String>,

    /// A list of commits to ignore.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ignore_commits: Vec<String>,

    /// A list of footer tokens (e.g. `Co-authored-by`) to use to find contributors.
    pub contributor_footers: Vec<String>,

    #[serde(skip)]
    pub template: Option<String>,

    #[serde(skip)]
    pub metadata: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        let mut types = HashMap::new();
        types.insert("fix", "Bug Fixes");
        types.insert("style", "Code Styling");
        types.insert("docs", "Documentation");
        types.insert("feat", "Features");
        types.insert("perf", "Performance Improvements");
        types.insert("refactor", "Refactoring");
        types.insert("test", "Tests");
        types.insert("chore", "Miscellaneous Tasks");
        let type_headers = types
            .into_iter()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect();

        let contributor_footers = vec![
            "co-authored-by",
            "signed-off-by",
            "reported-by",
            "tested-by",
            "reviewed-by",
            "suggested-by",
            "acked-by",
        ]
        .into_iter()
        .map(Into::into)
        .collect();

        Self {
            github: None,
            accept_types: None,
            type_headers,
            scope_headers: HashMap::new(),
            root_commit: None,
            ignore_commits: Vec::new(),
            contributor_footers,
            template: None,
            metadata: None,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Github {
    pub repo: String,
}

impl Config {
    pub fn from_environment(repo: &Repository, file: &str) -> Result<Self, Error> {
        Ok(Self::from_file(file)?.unwrap_or_else(|| Self {
            github: git::origin_url(repo)
                .ok()
                .and_then(|url| {
                    Url::parse(&url).ok().map(|u| {
                        u.path()
                            .strip_suffix(".git")
                            .unwrap_or(u.path())
                            .strip_prefix("/")
                            .unwrap_or(u.path())
                            .to_owned()
                    })
                })
                .map(|repo| Github { repo }),
            ..Default::default()
        }))
    }

    fn from_file(name: &str) -> Result<Option<Self>, Error> {
        let text = match read_to_string(name) {
            Ok(file) => file,
            Err(err) => match err.kind() {
                io::ErrorKind::NotFound => return Ok(None),
                _ => return Err(err.into()),
            },
        };

        let mut metadata = vec![];
        let mut level = 0;
        for line in text.lines().rev().skip_while(|l| !l.contains("-->")) {
            if line.contains("-->") {
                level += 1;
            }
            metadata.push(line);

            if line.contains("<!--") {
                if level == 1 {
                    break;
                }
                level -= 1;
            }
        }

        metadata.reverse();

        let mut config: Vec<&str> = vec![];
        let mut template: Vec<&str> = vec![];
        let mut bit = 0;
        for line in &metadata {
            // Start configuration fetching.
            if line.trim_end() == "Config(" {
                bit = 1;
                config.push("#![enable(implicit_some)]");
                config.push(line);
                continue;
            }

            // End configuration fetching.
            if bit == 1 && line.trim_end() == ")" {
                bit = 0;
                config.push(line);
                continue;
            }

            // Continue configuration fetching.
            if bit == 1 {
                config.push(line);
                continue;
            }

            // Start template fetching.
            if line.trim_end() == "Template(" {
                bit = 2;
                continue;
            }

            // End template fetching.
            if bit == 2 && line.trim_end() == ")" {
                bit = 0;
                continue;
            }

            // Continue template fetching.
            if bit == 2 {
                template.push(line);
            }
        }

        let config = config.join("\n");

        Ok(if config.contains("Config") {
            let mut config: Config = ron::de::from_str(&config).map_err(ron::Error::from)?;
            config.template = (!template.is_empty()).then_some(template.join("\n"));
            config.metadata = (!metadata.is_empty()).then_some(metadata.join("\n"));
            Some(config)
        } else {
            None
        })
    }
}
