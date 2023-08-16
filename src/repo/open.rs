use std::path::Path;
use git2::{Error, Repository};

#[cfg(test)]
use mockall::{automock, concretize};

#[cfg_attr(test, automock)]
pub trait RepoOpener {
    /// Opens the git repository found at the provided path.
    ///
    /// Note: The static lifetime for P is currently needed because of automock
    /// See https://github.com/asomers/mockall/issues/217
    /// I think we can remove once concretize is widely available

    #[cfg_attr(test, concretize)]
    fn open<P: AsRef<Path>>(&self, path: P) -> Result<Repository, Error>;
}

pub struct GitOpener;

impl RepoOpener for GitOpener {
    fn open<P: AsRef<Path>>(&self, path: P) -> Result<Repository, Error> {
        Repository::open(path)
    }
}

impl GitOpener {
    pub fn new() -> GitOpener {
        GitOpener {}
    }
}