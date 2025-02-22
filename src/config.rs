use crate::SheepError;
use serde::{Deserialize, Serialize};
use std::path::Path;

mod finder;
mod opener;

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Config {
    #[serde(default)]
    pub repository: RepoConfig,
    #[serde(default)]
    pub scripts: ScriptConfig,
    #[serde(default)]
    pub subprojects: Vec<SubprojectConfig>,
    #[serde(default)]
    pub transforms: Vec<TransformConfig>,
}

impl Config {
    pub fn open<P: AsRef<Path>>(repo_path: P) -> Result<Config, SheepError> {
        opener::open_config(repo_path)
    }
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct RepoConfig {
    #[serde(default = "default_branch_pattern")]
    pub branch_pattern: String,
    #[serde(default = "default_commit_message")]
    pub commit_message: String,
    #[serde(default = "default_default_branch")]
    pub default_branch: String,
    #[serde(default)]
    pub enable_branch: bool,
    #[serde(default)]
    pub enable_commit: bool,
    #[serde(default = "yes")]
    pub enable_push: bool,
    #[serde(default = "yes")]
    pub enable_tag: bool,
    #[serde(default = "default_tag_pattern")]
    pub tag_pattern: String,
}

fn default_branch_pattern() -> String {
    String::from("release/{version}")
}

fn default_commit_message() -> String {
    String::from("preparing release {version}")
}

fn default_default_branch() -> String {
    String::from("main")
}

fn default_tag_pattern() -> String {
    String::from("{version}")
}

fn yes() -> bool {
    true
}

impl Default for RepoConfig {
    fn default() -> Self {
        RepoConfig {
            branch_pattern: default_branch_pattern(),
            commit_message: default_commit_message(),
            default_branch: default_default_branch(),
            enable_branch: false,
            enable_commit: false,
            enable_tag: true,
            enable_push: true,
            tag_pattern: default_tag_pattern(),
        }
    }
}

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct TransformConfig {
    pub path: String,
    #[serde(default)]
    pub find: Option<String>,
    pub replace: String,
}

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct SubprojectConfig {
    pub repo_url: String,
}

#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct ScriptConfig {
    pub before_commit: Option<String>,
}

#[cfg(test)]
mod test {
    use crate::config::{Config, RepoConfig, ScriptConfig, SubprojectConfig, TransformConfig};

    #[test]
    fn default_config() {
        let expected = Config {
            repository: RepoConfig {
                branch_pattern: String::from("release/{version}"),
                commit_message: String::from("preparing release {version}"),
                default_branch: String::from("main"),
                enable_branch: false,
                enable_commit: false,
                enable_tag: true,
                enable_push: true,
                tag_pattern: String::from("{version}"),
            },
            scripts: ScriptConfig::default(),
            subprojects: vec![],
            transforms: vec![],
        };
        assert_eq!(expected, Config::default())
    }

    #[test]
    fn from_toml_completely_empty() {
        let config: Config = toml::from_str("").expect("failed to parse config");
        let expected = Config::default();
        assert_eq!(expected, config)
    }

    #[test]
    fn from_toml_empty_repo_config() {
        let config: Config = toml::from_str(
            r"
        [repository]
        ",
        )
        .expect("failed to parse config");
        let expected = Config::default();
        assert_eq!(expected, config)
    }

    #[test]
    fn from_toml_full_config() {
        let config: Config = toml::from_str(
            r"
        [repository]
        branch_pattern = 'branch'
        commit_message = 'commit'
        default_branch = 'dev'
        enable_branch = true
        enable_commit = true
        enable_tag = false
        enable_push = false
        tag_pattern = 'tag'

        [scripts]
        before_commit = 'echo hello'

        [[subprojects]]
        repo_url = 'https://api.example.com'

        [[transforms]]
        path = 'path_1'
        find = 'find_1'
        replace = 'replace_1'

        [[transforms]]
        path = 'path_2'
        replace = 'replace_2'
        ",
        )
        .expect("failed to parse config");

        let expected = Config {
            repository: RepoConfig {
                branch_pattern: "branch".to_string(),
                commit_message: "commit".to_string(),
                default_branch: "dev".to_string(),
                enable_branch: true,
                enable_commit: true,
                enable_push: false,
                enable_tag: false,
                tag_pattern: "tag".to_string(),
            },
            scripts: ScriptConfig {
                before_commit: "echo hello".to_string().into(),
            },
            subprojects: vec![SubprojectConfig {
                repo_url: "https://api.example.com".to_string(),
            }],
            transforms: vec![
                TransformConfig {
                    path: "path_1".to_string(),
                    find: Some("find_1".to_string()),
                    replace: "replace_1".to_string(),
                },
                TransformConfig {
                    path: "path_2".to_string(),
                    find: None,
                    replace: "replace_2".to_string(),
                },
            ],
        };
        assert_eq!(expected, config)
    }
}
