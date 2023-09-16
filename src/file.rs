use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
#[cfg(test)]
use mockall::automock;
#[cfg(test)]
use mockall::concretize;
use crate::SheepError;

pub struct FileChecker;

#[cfg_attr(test, automock)]
impl FileChecker {
    pub fn new() -> Self {
        Self {}
    }

    #[cfg_attr(test, concretize)]
    pub fn file_exists<P: AsRef<Path>>(&self, path: P) -> bool {
        path.as_ref().exists()
    }
}

pub struct FileReader;

#[cfg_attr(test, automock)]
impl FileReader {
    pub fn new() -> Self {
        Self {}
    }

    #[cfg_attr(test, concretize)]
    pub fn read_to_string<P: AsRef<Path>>(&self, path: P) -> Result<String, SheepError> {
        Ok(fs::read_to_string(path)?)
    }
}

pub struct FileWriter;

#[cfg_attr(test, automock)]
impl FileWriter {
    pub fn new() -> Self {
        Self {}
    }

    #[cfg_attr(test, concretize)]
    pub fn write_string_to_file<P: AsRef<Path>>(&self, path: P,
                                                text: &str) -> Result<(), SheepError> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)?;

        file.write_all(text.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use tempfile::{NamedTempFile, TempPath};
    use crate::file::{FileChecker, FileReader, FileWriter};

    #[test]
    fn checker_file_exists_existing_file() {
        let checker = FileChecker::new();
        let path = file!();
        assert!(checker.file_exists(path))
    }

    #[test]
    fn checker_file_exists_invalid_file() {
        let checker = FileChecker::new();
        let path = format!("{}__invalid", file!());
        assert!(!checker.file_exists(path))
    }

    #[test]
    fn reader_read_into_string_valid_file() {
        let reader = FileReader::new();
        let path = file!();
        let text = reader.read_to_string(path).expect("reader failed to read file");
        assert!(text.len() > 0)
    }

    #[test]
    fn reader_read_into_string_invalid_file() {
        let reader = FileReader::new();
        let path = format!("{}__invalid", file!());
        reader.read_to_string(path).expect_err("reader should have produced error");
    }

    #[test]
    fn writer_write_string_to_file_fresh_file() {
        let expected_text = "test!!";
        let reader = FileReader::new();
        let writer = FileWriter::new();
        let temp_path = temp_path();
        let path = temp_path.to_path_buf();

        writer.write_string_to_file(path.clone(), expected_text)
            .expect("failed to write to file");
        let file_contents = reader.read_to_string(path).expect("couldn't read file");
        assert_eq!(expected_text, file_contents)
    }

    #[test]
    fn writer_write_string_to_file_overwrites_contents() {
        let old_text = "old";
        let expected_text = "new";
        let reader = FileReader::new();
        let writer = FileWriter::new();
        let temp_path = temp_path();
        let path = temp_path.to_path_buf();

        writer.write_string_to_file(path.clone(), old_text)
            .expect("failed to write to file");
        writer.write_string_to_file(path.clone(), expected_text)
            .expect("failed to overwrite to file");
        let file_contents = reader.read_to_string(path).expect("couldn't read file");
        assert_eq!(expected_text, file_contents)
    }

    fn temp_path() -> TempPath {
        let file = NamedTempFile::new().expect("unable to create temp file");
        file.into_temp_path()
    }
}


