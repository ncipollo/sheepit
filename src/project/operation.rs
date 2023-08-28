use mockall_double::double;
use semver::Version;
use crate::version::update::VersionUpdate;
#[double]
use crate::project::project_version::ProjectVersion;
use crate::version::bump;

pub enum Operation {
    BumpVersion(BumpMode),
    SetVersion { current_version: Option<Version>, next_version: Version },
}

pub enum BumpMode {
    Major,
    Minor,
    Patch,
}

impl Operation {
    pub fn version_update(&self, project_version: &ProjectVersion) -> VersionUpdate {
        match self {
            Operation::BumpVersion(bump_mode) => {
                Self::bump_version(project_version, bump_mode)
            }
            Operation::SetVersion { current_version, next_version } => {
                Self::set_version(project_version, current_version, next_version)
            }
        }
    }

    fn bump_version(project_version: &ProjectVersion, bump_mode: &BumpMode) -> VersionUpdate {
        let current_version = project_version.current_version();
        let next_version = match bump_mode {
            BumpMode::Major => bump::major_version(&current_version),
            BumpMode::Minor => bump::minor_version(&current_version),
            BumpMode::Patch => bump::patch_version(&current_version),
        };
        VersionUpdate { current_version, next_version }
    }

    fn set_version(project_version: &ProjectVersion,
                   current_version: &Option<Version>,
                   next_version: &Version) -> VersionUpdate {
        let project_current_version = current_version.as_ref()
            .map(|version_ref| version_ref.clone())
            .unwrap_or_else(|| project_version.current_version());
        VersionUpdate {
            current_version: project_current_version,
            next_version: next_version.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use semver::Version;
    use crate::project::operation::{BumpMode, Operation};
    use crate::project::project_version::MockProjectVersion;
    use crate::version::update::VersionUpdate;

    #[test]
    fn version_update_bump_version_major() {
        let project_version = MockProjectVersion::mock();
        let operation = Operation::BumpVersion(BumpMode::Major);

        let version_update = operation.version_update(&project_version);
        let expected = VersionUpdate {
            current_version: Version::new(1, 0, 0),
            next_version: Version::new(2, 0, 0),
        };
        assert_eq!(expected, version_update)
    }

    #[test]
    fn version_update_bump_version_minor() {
        let project_version = MockProjectVersion::mock();
        let operation = Operation::BumpVersion(BumpMode::Minor);

        let version_update = operation.version_update(&project_version);
        let expected = VersionUpdate {
            current_version: Version::new(1, 0, 0),
            next_version: Version::new(1, 1, 0),
        };
        assert_eq!(expected, version_update)
    }

    #[test]
    fn version_update_bump_version_patch() {
        let project_version = MockProjectVersion::mock();
        let operation = Operation::BumpVersion(BumpMode::Patch);

        let version_update = operation.version_update(&project_version);
        let expected = VersionUpdate {
            current_version: Version::new(1, 0, 0),
            next_version: Version::new(1, 0, 1),
        };
        assert_eq!(expected, version_update)
    }

    #[test]
    fn version_update_set_version_no_current_version() {
        let project_version = MockProjectVersion::mock();
        let next_version = Version::new(2, 0, 0);
        let operation = Operation::SetVersion { current_version: None, next_version };

        let version_update = operation.version_update(&project_version);
        let expected = VersionUpdate {
            current_version: Version::new(1, 0, 0),
            next_version: Version::new(2, 0, 0),
        };
        assert_eq!(expected, version_update)
    }

    #[test]
    fn version_update_set_version_with_current_version() {
        let project_version = MockProjectVersion::mock();
        let current_version = Some(Version::new(0, 1, 0));
        let next_version = Version::new(2, 0, 0);
        let operation = Operation::SetVersion { current_version, next_version };

        let version_update = operation.version_update(&project_version);
        let expected = VersionUpdate {
            current_version: Version::new(0, 1, 0),
            next_version: Version::new(2, 0, 0),
        };
        assert_eq!(expected, version_update)
    }
}