use std::path::Path;
use git2::{Error, Repository};
pub struct GitOpener;

impl GitOpener {
    pub fn new() -> Self {
        GitOpener {}
    }

    pub fn open<P: AsRef<Path>>(&self, path: P) -> Result<Repository, Error> {
        Repository::open(path)
    }
}