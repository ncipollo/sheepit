use std::path::Path;
#[cfg(test)]
use mockall::automock;
#[cfg(test)]
use mockall::concretize;

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

#[cfg(test)]
mod test {
    use crate::file::FileChecker;

    #[test]
    fn file_exists_existing_file() {
        let checker = FileChecker::new();
        let path = file!();
        assert!(checker.file_exists(path))
    }

    #[test]
    fn file_exists_invalid_file() {
        let checker = FileChecker::new();
        let path = format!("{}__invalid", file!());
        assert!(!checker.file_exists(path))
    }
}


