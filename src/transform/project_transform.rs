use crate::config::TransformConfig;
#[double]
use crate::file::{FileReader, FileWriter};
use crate::transform::file_transform::FileTransformer;
use crate::version::update::VersionUpdate;
use crate::SheepError;
use mockall_double::double;
use std::path::{Path, PathBuf};

pub struct ProjectTransformer {
    file_reader: FileReader,
    file_writer: FileWriter,
    project_path: PathBuf,
}
impl ProjectTransformer {
    #[cfg(test)]
    pub fn for_tests(
        file_reader: FileReader,
        file_writer: FileWriter,
        project_path: PathBuf,
    ) -> Self {
        ProjectTransformer {
            file_reader,
            file_writer,
            project_path,
        }
    }

    pub fn new<P: AsRef<Path>>(project_path: P) -> Self {
        ProjectTransformer {
            file_reader: FileReader::new(),
            file_writer: FileWriter::new(),
            project_path: project_path.as_ref().to_path_buf(),
        }
    }

    pub fn transform(
        &self,
        configs: &Vec<TransformConfig>,
        version_update: &VersionUpdate,
    ) -> Result<(), SheepError> {
        configs
            .iter()
            .map(|c| {
                FileTransformer::new(c, &self.file_reader, &self.file_writer, &self.project_path)
            })
            .try_for_each(|t| t.transform(version_update))
    }
}

#[cfg(test)]
mod test {
    use crate::config::TransformConfig;
    use crate::file::{MockFileReader, MockFileWriter};
    use crate::transform::project_transform::ProjectTransformer;
    use crate::version::update::VersionUpdate;
    use crate::SheepError;
    use semver::Version;
    use std::path::PathBuf;

    const PATH_1: &str = "path_1";
    const PATH_2: &str = "path_2";
    const PROJECT_PATH: &str = "project";
    const FULL_PATH_1: &str = "project/path_1";
    const FULL_PATH_2: &str = "project/path_2";

    #[test]
    fn transform_file_transform_error() {
        let project_transformer = ProjectTransformer::for_tests(
            failed_reader(),
            MockFileWriter::default(),
            PathBuf::from(PROJECT_PATH),
        );
        project_transformer
            .transform(&configs(), &version_update())
            .expect_err("should have failed");
    }

    #[test]
    fn transform_applies_all_transforms() {
        let reader = mock_reader("first_1.0.0".to_string(), "second_1.0.0".to_string());
        let writer = mock_writer("first_2.0.0".to_string(), "second_2.0.0".to_string());
        let project_transformer =
            ProjectTransformer::for_tests(reader, writer, PathBuf::from(PROJECT_PATH));
        project_transformer
            .transform(&configs(), &version_update())
            .expect("transform fails")
    }

    fn failed_reader() -> MockFileReader {
        let mut mock = MockFileReader::default();
        mock.expect_read_to_string()
            .return_once(|_| Err(SheepError::new("transform fail")));
        mock
    }

    fn mock_reader(first_text_read: String, second_text_read: String) -> MockFileReader {
        let mut mock = MockFileReader::default();
        mock.expect_read_to_string()
            .withf_st(|p| p.as_ref().to_str().unwrap() == FULL_PATH_1)
            .return_once(|_| Ok(first_text_read));
        mock.expect_read_to_string()
            .withf_st(|p| p.as_ref().to_str().unwrap() == FULL_PATH_2)
            .return_once(|_| Ok(second_text_read));
        mock
    }

    fn mock_writer(first_expected: String, second_expected: String) -> MockFileWriter {
        let mut mock = MockFileWriter::default();
        mock.expect_write_string_to_file()
            .withf_st(move |p, t| p.as_ref().to_str().unwrap() == FULL_PATH_1 && t == first_expected)
            .return_once(|_, _| Ok(()));
        mock.expect_write_string_to_file()
            .withf_st(move |p, t| p.as_ref().to_str().unwrap() == FULL_PATH_2 && t == second_expected)
            .return_once(|_, _| Ok(()));
        mock
    }

    fn configs() -> Vec<TransformConfig> {
        vec![
            TransformConfig {
                path: PATH_1.to_string(),
                find: None,
                replace: "first_{version}".to_string(),
            },
            TransformConfig {
                path: PATH_2.to_string(),
                find: None,
                replace: "second_{version}".to_string(),
            },
        ]
    }

    fn version_update() -> VersionUpdate {
        VersionUpdate {
            current_version: Version::parse("1.0.0").expect("failed to parse version"),
            next_version: Version::parse("2.0.0").expect("failed to parse version"),
        }
    }
}
