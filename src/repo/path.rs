use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use parse_git_url::GitUrl;
use crate::error::SheepError;

pub fn repo_path(repo_url: &str, directory: &Path) -> Result<PathBuf, SheepError> {
    let repo_name = repo_name(repo_url)?;
    Ok([directory.as_os_str(), OsStr::new(&repo_name)].iter().collect::<PathBuf>())
}

fn repo_name(repo_url: &str) -> Result<String, SheepError> {
    let git_url = GitUrl::parse(repo_url)?;
    if git_url.name.is_empty() {
        Err(SheepError::new("no repo name found in git url"))
    } else {
        Ok(git_url.name)
    }
}


#[cfg(test)]
mod test {
    use std::path::Path;
    use crate::repo::path;

    #[test]
    fn repo_path_invalid_url() {
        path::repo_path("invalid", Path::new("/dir"))
            .expect_err("failed to emit error for invalid URL");
    }

    #[test]
    fn repo_path_https_url() {
        let path = path::repo_path("https://github.com/ncipollo/sheepit.git",
                                   Path::new("/dir"))
            .expect("failed to parse ssh url");
        assert_eq!(Path::new("/dir/sheepit"), path.as_path())
    }

    #[test]
    fn repo_path_ssh_url() {
        let path = path::repo_path("git@github.com:ncipollo/sheepit.git",
                                   Path::new("/dir"))
            .expect("failed to parse ssh url");
        assert_eq!(Path::new("/dir/sheepit"), path.as_path())
    }
}