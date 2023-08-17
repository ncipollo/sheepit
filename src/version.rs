use semver::{BuildMetadata, Prerelease, Version};

pub fn bump_major_version(version: Version) -> Version {
    Version {
        major: version.major + 1,
        minor: 0,
        patch: 0,
        pre: Prerelease::EMPTY,
        build: BuildMetadata::EMPTY,
    }
}

pub fn bump_minor_version(version: Version) -> Version {
    Version {
        major: version.major,
        minor: version.minor + 1,
        patch: 0,
        pre: Prerelease::EMPTY,
        build: BuildMetadata::EMPTY,
    }
}

pub fn bump_patch_version(version: Version) -> Version {
    Version {
        major: version.major,
        minor: version.minor,
        patch: version.patch + 1,
        pre: Prerelease::EMPTY,
        build: BuildMetadata::EMPTY,
    }
}

#[cfg(test)]
mod test {
    use semver::Version;
    use crate::version::{bump_major_version, bump_minor_version, bump_patch_version};

    fn test_version() -> Version {
        Version {
            major: 1,
            minor: 2,
            patch: 3,
            pre: Default::default(),
            build: Default::default(),
        }
    }

    #[test]
    fn test_bump_major_version() {
        let version = bump_major_version(test_version());
        let expected = Version {
            major: 2,
            minor: 0,
            patch: 0,
            pre: Default::default(),
            build: Default::default(),
        };
        assert_eq!(expected, version)
    }

    #[test]
    fn test_bump_minor_version() {
        let version = bump_minor_version(test_version());
        let expected = Version {
            major: 1,
            minor: 3,
            patch: 0,
            pre: Default::default(),
            build: Default::default(),
        };
        assert_eq!(expected, version)
    }

    #[test]
    fn test_bump_patch_version() {
        let version = bump_patch_version(test_version());
        let expected = Version {
            major: 1,
            minor: 2,
            patch: 4,
            pre: Default::default(),
            build: Default::default(),
        };
        assert_eq!(expected, version)
    }
}