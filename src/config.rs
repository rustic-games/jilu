use crate::Error;
use boolinator::Boolinator;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::io;

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub github: Option<Github>,
    pub accept_types: Option<Vec<String>>,
    pub type_headers: HashMap<String, String>,
    pub scope_headers: HashMap<String, String>,

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

        Self {
            github: None,
            accept_types: None,
            type_headers,
            scope_headers: HashMap::new(),
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
    pub fn from_environment() -> Result<Self, Error> {
        Ok(Self::from_file("CHANGELOG.md")?.unwrap_or_default())
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
        for line in text.lines().rev().skip_while(|l| !l.contains("-->")) {
            metadata.push(line);

            if line.contains("<!--") {
                break;
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

        Ok(if config.contains(&"Config") {
            let mut config: Config = ron::de::from_str(&config)?;
            config.template = (!template.is_empty()).as_some(template.join("\n"));
            config.metadata = (!metadata.is_empty()).as_some(metadata.join("\n"));
            Some(config)
        } else {
            None
        })
    }
}
