use semver::Version;

#[derive(Clone, Debug, PartialEq)]
pub struct VersionUpdate {
    pub current_version: Version,
    pub next_version: Version
}