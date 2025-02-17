use semver::Version;

#[derive(Clone, Debug, PartialEq)]
pub struct VersionUpdate {
    pub current_version: Version,
    pub next_version: Version,
}

impl VersionUpdate {
    pub fn new(current_version: &str, next_version: &str) -> Self {
        Self {
            current_version: Version::parse(current_version).unwrap(),
            next_version: Version::parse(next_version).unwrap(),
        }
    }
}
