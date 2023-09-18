use crate::config::TransformConfig;
#[double]
use crate::file::{FileReader, FileWriter};
use crate::version::update::VersionUpdate;
use crate::{token, SheepError};
use mockall_double::double;
use std::path::{Path, PathBuf};

pub struct FileTransformer<'a> {
    config: &'a TransformConfig,
    file_reader: &'a FileReader,
    file_writer: &'a FileWriter,
    project_path: &'a Path,
}

impl<'a> FileTransformer<'a> {
    pub fn new(
        config: &'a TransformConfig,
        file_reader: &'a FileReader,
        file_writer: &'a FileWriter,
        project_path: &'a Path,
    ) -> Self {
        Self {
            config,
            file_reader,
            file_writer,
            project_path,
        }
    }

    pub fn transform(&self, version_update: &VersionUpdate) -> Result<(), SheepError> {
        let relative_path = &self.config.path;
        let path = self.full_path(relative_path);
        let file_text = self.file_reader.read_to_string(&path)?;
        let transformed = file_text.replacen(
            &self.find_string(version_update),
            &self.replace_string(version_update),
            1,
        );
        self.file_writer.write_string_to_file(&path, &transformed)?;
        Ok(())
    }

    fn full_path(&self, relative_path: &str) -> PathBuf {
        [self.project_path, &Path::new(relative_path)].iter().collect()
    }

    fn find_string(&self, version_update: &VersionUpdate) -> String {
        let find = self
            .config
            .find
            .clone()
            .unwrap_or(self.config.replace.clone());
        find.replacen(
            token::VERSION,
            &version_update.current_version.to_string(),
            1,
        )
    }

    fn replace_string(&self, version_update: &VersionUpdate) -> String {
        let replace = self.config.replace.clone();
        replace.replacen(token::VERSION, &version_update.next_version.to_string(), 1)
    }
}

#[cfg(test)]
mod test {
    use crate::config::TransformConfig;
    use crate::file::{MockFileReader, MockFileWriter};
    use crate::transform::file_transform::FileTransformer;
    use crate::version::update::VersionUpdate;
    use semver::Version;
    use std::path::{PathBuf};

    const PATH: &str = "path";
    const PROJECT_PATH: &str = "project";
    const FULL_PATH: &str = "project/path";

    #[test]
    fn transform_find_and_replace_no_version() {
        let reader = mock_reader("find");
        let writer = mock_writer("replace");
        let config = TransformConfig {
            path: PATH.to_string(),
            find: Some("find".to_string()),
            replace: "replace".to_string(),
        };
        let project_path = project_path();
        let file_transformer = FileTransformer::new(&config, &reader, &writer, &project_path);
        file_transformer
            .transform(&version_update())
            .expect("transform failed");
    }

    #[test]
    fn transform_find_and_replace_find_version() {
        let reader = mock_reader("find_1.0.0");
        let writer = mock_writer("replace");
        let config = TransformConfig {
            path: PATH.to_string(),
            find: Some("find_{version}".to_string()),
            replace: "replace".to_string(),
        };
        let project_path = project_path();
        let file_transformer = FileTransformer::new(&config, &reader, &writer, &project_path);
        file_transformer
            .transform(&version_update())
            .expect("transform failed");
    }

    #[test]
    fn transform_find_and_replace_both_version() {
        let reader = mock_reader("find_1.0.0");
        let writer = mock_writer("replace__2.0.0");
        let config = TransformConfig {
            path: PATH.to_string(),
            find: Some("find_{version}".to_string()),
            replace: "replace__{version}".to_string(),
        };
        let project_path = project_path();
        let file_transformer = FileTransformer::new(&config, &reader, &writer, &project_path);
        file_transformer
            .transform(&version_update())
            .expect("transform failed");
    }

    #[test]
    fn transform_only_replace() {
        let reader = mock_reader("version_1.0.0");
        let writer = mock_writer("version_2.0.0");
        let config = TransformConfig {
            path: PATH.to_string(),
            find: None,
            replace: "version_{version}".to_string(),
        };
        let project_path = project_path();
        let file_transformer = FileTransformer::new(&config, &reader, &writer, &project_path);
        file_transformer
            .transform(&version_update())
            .expect("transform failed");
    }

    fn project_path() -> PathBuf {
        PathBuf::from(PROJECT_PATH)
    }

    fn version_update() -> VersionUpdate {
        VersionUpdate {
            current_version: Version::parse("1.0.0").expect("failed to parse version"),
            next_version: Version::parse("2.0.0").expect("failed to parse version"),
        }
    }

    fn mock_reader(text: &str) -> MockFileReader {
        let text_copy = text.to_string();
        let mut mock = MockFileReader::default();
        mock.expect_read_to_string()
            .withf_st(|p| p.as_ref().to_str().unwrap() == FULL_PATH)
            .return_once(|_| Ok(text_copy));
        mock
    }

    fn mock_writer(expected: &str) -> MockFileWriter {
        let expected_copy = expected.to_string();
        let mut mock = MockFileWriter::default();
        mock.expect_write_string_to_file()
            .withf_st(move |p, t| {
                p.as_ref().to_str().unwrap() == FULL_PATH && t == expected_copy.clone()
            })
            .return_once(|_, _| Ok(()));
        mock
    }
}
