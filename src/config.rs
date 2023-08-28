#[derive(Debug, Default, PartialEq)]
pub struct Config {
    pub repository: RepoConfig
}

#[derive(Debug, PartialEq)]
pub struct RepoConfig {
    pub enable_branch: bool,
    pub enable_commit: bool,
    pub enable_tag: bool,
    pub enable_push: bool,
    pub tag_pattern: String
}

impl Default for RepoConfig {
    fn default() -> Self {
        RepoConfig {
            enable_branch: true,
            enable_commit: true,
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
                enable_branch: true,
                enable_commit: true,
                enable_tag: true,
                enable_push: true,
                tag_pattern: String::from("{version}")
            }
        };
        assert_eq!(expected, Config::default())
    }
}