mod finder;
mod opener;

#[derive(Debug, Default, PartialEq)]
pub struct Config {
    pub repository: RepoConfig
}

#[derive(Debug, PartialEq)]
pub struct RepoConfig {
    pub branch_pattern: String,
    pub commit_message: String,
    pub default_branch: String,
    pub enable_branch: bool,
    pub enable_commit: bool,
    pub enable_push: bool,
    pub enable_tag: bool,
    pub tag_pattern: String
}

impl Default for RepoConfig {
    fn default() -> Self {
        RepoConfig {
            branch_pattern: String::from("release/{version}"),
            commit_message: String::from("preparing release {version}"),
            default_branch: String::from("main"),
            enable_branch: false,
            enable_commit: false,
            enable_tag: true,
            enable_push: true,
            tag_pattern: String::from("{version}")
        }
    }
}

#[cfg(test)]
mod test {
    use crate::config::{Config, RepoConfig};

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
                tag_pattern: String::from("{version}")
            }
        };
        assert_eq!(expected, Config::default())
    }
}