use std::path::Path;
use git2;
use git2::{Error, Repository};
use crate::repo::ssh;

pub struct GitCloner {}

impl GitCloner {
    pub fn new() -> Self {
        return GitCloner {};
    }

    pub fn clone<P: AsRef<Path>>(&self, repo_url: &str, path: P) -> Result<Repository, Error> {
        let mut callbacks = git2::RemoteCallbacks::new();
        ssh::add_credentials_to_callbacks(&mut callbacks);
        // Prepare fetch options.
        let mut fetch_options = git2::FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);
        // Prepare builder.
        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(fetch_options);

        // Clone the project.
        builder.clone(repo_url, path.as_ref())
    }
}