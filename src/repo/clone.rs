use std::path::Path;
use git2;
use git2::{Error, Repository};
use crate::repo::ssh::default_ssh_key_path;
use super::options::CloneOptions;
#[cfg(test)]
use mockall::automock;
use crate::repo::ssh;

#[cfg_attr(test, automock)]
pub trait RepoCloner {
    fn clone(&self, options: CloneOptions) -> Result<Repository, Error>;
}

pub struct GitCloner {}

impl GitCloner {
    pub fn new() -> GitCloner {
        return GitCloner {};
    }
}

impl RepoCloner for GitCloner {
    fn clone(&self, clone_options: CloneOptions) -> Result<Repository, Error> {
        let mut callbacks = git2::RemoteCallbacks::new();
        ssh::add_credentials_to_callbacks(&mut callbacks);
        // Prepare fetch options.
        let mut fetch_options = git2::FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);
        // Prepare builder.
        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(fetch_options);

        // Clone the project.
        builder.clone(
            &clone_options.repo_url,
            Path::new(&clone_options.path),
        )
    }
}