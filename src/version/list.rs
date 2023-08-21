use semver::Version;
use crate::token::TokenTrimmer;

#[derive(Debug, PartialEq)]
pub struct VersionList(Vec<Version>);

impl VersionList {
    pub fn from_tag_list(tag_names: Vec<&str>,
                         token_trimmer: Option<TokenTrimmer>) -> VersionList {
        let trimmed_tag_names = VersionList::trimmed_tag_names(tag_names, token_trimmer);
        let mut versions: Vec<Version> = trimmed_tag_names.iter()
            .filter_map(|tag| lenient_semver::parse(tag).ok())
            .collect();
        versions.sort();
        VersionList(versions)
    }

    fn trimmed_tag_names<'a>(tag_names: Vec<&'a str>,
                             token_trimmer: Option<TokenTrimmer<'a>>) -> Vec<&'a str> {
        match token_trimmer {
            None => tag_names,
            Some(trimmer) => tag_names.iter()
                .map(|name| trimmer.trim_text(name))
                .collect()
        }
    }

    pub fn latest_version(&self) -> Option<Version> {
        self.0.last().map(Version::clone)
    }
}

#[cfg(test)]
mod test {
    use crate::token::TokenTrimmer;
    use crate::version::list::VersionList;

    #[test]
    fn from_tag_list_empty() {
        let version_list = VersionList::from_tag_list(vec![], None);
        let expected = VersionList(vec![]);
        assert_eq!(expected, version_list)
    }

    #[test]
    fn from_tag_list_no_trimmer() {
        let tags = vec![
            "10.0.0",
            "2.0.0",
            "xxxx",
            "0.1.0",
            "0.0.1",
        ];
        let version_list = VersionList::from_tag_list(tags, None);
        let expected_tags = vec![
            "0.0.1",
            "0.1.0",
            "2.0.0",
            "10.0.0",
        ];
        let expected_versions = expected_tags.iter()
            .filter_map(|tag| lenient_semver::parse(tag).ok())
            .collect::<Vec<_>>();
        let expected = VersionList(expected_versions);
        assert_eq!(expected, version_list)
    }

    #[test]
    fn from_tag_list_with_trimmer() {
        let tags = vec![
            "release_10.0.0_xxx",
            "2.0.0",
            "xxxx",
            "release_0.1.0_xxx",
            "release_0.0.1_xxx",
        ];
        let token_trimmer = TokenTrimmer::new("release_$version_xxx", "$version");
        let version_list = VersionList::from_tag_list(tags, token_trimmer);
        let expected_tags = vec![
            "0.0.1",
            "0.1.0",
            "2.0.0",
            "10.0.0",
        ];
        let expected_versions = expected_tags.iter()
            .filter_map(|tag| lenient_semver::parse(tag).ok())
            .collect::<Vec<_>>();
        let expected = VersionList(expected_versions);
        assert_eq!(expected, version_list)
    }

    #[test]
    fn latest_version_empty_version_list() {
        let version_list = VersionList::from_tag_list(vec![], None);
        assert_eq!(None, version_list.latest_version())
    }

    #[test]
    fn latest_version_with_versions() {
        let tags = vec![
            "10.0.0",
            "2.0.0",
            "0.0.1",
        ];
        let version_list = VersionList::from_tag_list(tags, None);
        let expected_version = lenient_semver::parse("10.0.0")
            .expect("expected version should parse");
        assert_eq!(Some(expected_version), version_list.latest_version())
    }
}

