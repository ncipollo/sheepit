use crate::config::Config;
use crate::token;
use crate::version::update::VersionUpdate;

#[derive(Debug, PartialEq)]
pub struct ProjectStrings {
    pub branch_name: String,
    pub commit_message: String,
    pub remote_name: String,
    pub tag_name: String,
}

impl ProjectStrings {
    pub fn new(config: &Config, version_update: &VersionUpdate) -> ProjectStrings {
        let repo_config = &config.repository;
        let next_version = &version_update.next_version.to_string();
        let token = token::VERSION;
        ProjectStrings {
            branch_name: repo_config.branch_pattern.replace(token, next_version),
            commit_message: repo_config.commit_message.replace(token, next_version),
            remote_name: "origin".to_string(),
            tag_name: repo_config.tag_pattern.replace(token, next_version),
        }
    }
}

#[cfg(test)]
mod test {
    use semver::Version;
    use crate::config::Config;
    use crate::project::strings::ProjectStrings;
    use crate::version::update::VersionUpdate;

    #[test]
    fn new() {
        let version_update = VersionUpdate {
            current_version: Version::new(0, 0, 1),
            next_version: Version::new(1, 2, 3),
        };
        let config = Config::default();
        let expected = ProjectStrings {
            branch_name: "release/1.2.3".to_string(),
            commit_message: "preparing release 1.2.3".to_string(),
            remote_name: "origin".to_string(),
            tag_name: "1.2.3".to_string(),
        };
        assert_eq!(expected, ProjectStrings::new(&config, &version_update))
    }
}