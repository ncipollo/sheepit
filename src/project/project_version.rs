use semver::Version;
use crate::config::Config;
use crate::project::Project;
use crate::repo::tag::GitTags;
use crate::token::TokenTrimmer;
use crate::version::list::VersionList;

pub struct ProjectVersion<'a> {
    project: &'a Project,
}

#[cfg_attr(test, allow(dead_code))]
impl<'a> ProjectVersion<'a> {
    pub fn new(project: &'a Project) -> Self {
        Self { project }
    }

    pub fn current_version(&self) -> Version {
        let tags = GitTags::new();
        let tag_list = tags.get_tags(&self.project.repo).unwrap_or_default();
        let tag_token_trimmer = Self::tag_token_trimmer(&self.project.config);
        let version_list = VersionList::from_tag_list(&tag_list,
                                                      tag_token_trimmer);
        version_list.latest_version().unwrap_or(Self::default_version())
    }

    fn tag_token_trimmer(config: &Config) -> Option<TokenTrimmer> {
        let tag_pattern = &config.repository.tag_pattern;
        TokenTrimmer::new(tag_pattern, "{version}")
    }

    fn default_version() -> Version {
        Version::new(0, 0, 1)
    }
}

/// Manually created mock for [`ProjectVersion`]. Automock was having trouble with the lifetime
/// generic in ProjectVersion.
#[cfg(test)]
pub struct MockProjectVersion;

#[cfg(test)]
impl MockProjectVersion {
    pub fn mock() -> Self {
        Self {}
    }

    pub fn new(_: &Project) -> Self {
        Self {}
    }

    pub fn current_version(&self) -> Version {
        Version::new(1, 0, 0)
    }
}