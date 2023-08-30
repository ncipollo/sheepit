use std::ffi::OsString;
use std::path::{Path, PathBuf};
use mockall_double::double;
#[double]
use crate::file::FileChecker;

pub fn find_config<P: AsRef<Path>>(file_checker: &FileChecker, repo_path: P) -> Option<PathBuf> {
    let paths = config_paths(repo_path);
    paths.iter().find(|path| file_checker.file_exists(path))
        .cloned()
}

fn config_paths<P: AsRef<Path>>(repo_path: P) -> Vec<PathBuf> {
    let base_path_ref = repo_path.as_ref();
    let base_path = base_path_ref.as_os_str().to_os_string();

    config_names().iter()
        .map(|name| [&base_path, name].iter().collect())
        .collect()
}

fn config_names() -> Vec<OsString> {
    vec![OsString::from("sheepit.toml"),
         OsString::from(".sheepit.toml")]
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;
    use mockall_double::double;
    use crate::config::finder::find_config;
    #[double]
    use crate::file::{FileChecker};

    const PATH: &str = "/path/";

    #[test]
    fn find_config_all_configs() {
        let mut file_checker = FileChecker::default();
        file_checker.expect_file_exists().return_const(true);

        let config_path = find_config(&file_checker, PATH);
        let expected = PathBuf::from("/path/sheepit.toml");
        assert_eq!(Some(expected), config_path);
    }

    #[test]
    fn find_config_no_configs() {
        let mut file_checker = FileChecker::default();
        file_checker.expect_file_exists().return_const(false);

        let config_path = find_config(&file_checker, PATH);
        assert_eq!(None, config_path);
    }

    #[test]
    fn find_config_on_config_not_hidden() {
        let mut file_checker = FileChecker::default();
        file_checker.expect_file_exists()
            .withf(|path| path.as_ref() == PathBuf::from("/path/sheepit.toml"))
            .return_const(true);
        file_checker.expect_file_exists()
            .withf(|path| path.as_ref() == PathBuf::from("/path/.sheepit.toml"))
            .return_const(false);

        let config_path = find_config(&file_checker, PATH);
        let expected = PathBuf::from("/path/sheepit.toml");
        assert_eq!(Some(expected), config_path);
    }

    #[test]
    fn find_config_on_config_hidden() {
        let mut file_checker = FileChecker::default();
        file_checker.expect_file_exists()
            .withf(|path| path.as_ref() == PathBuf::from("/path/sheepit.toml"))
            .return_const(false);
        file_checker.expect_file_exists()
            .withf(|path| path.as_ref() == PathBuf::from("/path/.sheepit.toml"))
            .return_const(true);

        let config_path = find_config(&file_checker, PATH);
        let expected = PathBuf::from("/path/.sheepit.toml");
        assert_eq!(Some(expected), config_path);
    }
}