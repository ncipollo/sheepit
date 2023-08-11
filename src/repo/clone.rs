use std::path::Path;
use git2;
use crate::repo::ssh::default_ssh_key_path;
use super::options::CloneOptions;

pub trait RepoCloner {
    fn clone(&self, options: CloneOptions) -> Result<git2::Repository, git2::Error>;
}

pub struct GitCloner {}

impl RepoCloner for GitCloner {
    fn clone(&self, clone_options: CloneOptions) -> Result<git2::Repository, git2::Error> {
        let mut callbacks = git2::RemoteCallbacks::new();
        GitCloner::add_credentials_to_callbacks(&mut callbacks);
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

impl GitCloner {
    pub fn new() -> GitCloner {
        return GitCloner {};
    }

    fn add_credentials_to_callbacks(remote_callbacks: &mut git2::RemoteCallbacks) {
        remote_callbacks.credentials(|_, username_from_url, _| {
            GitCloner::create_ssh_key(username_from_url.unwrap())
        });
    }

    fn create_ssh_key(username_from_url: &str) -> Result<git2::Cred, git2::Error> {
        git2::Cred::ssh_key(
            username_from_url,
            None,
            Path::new(&default_ssh_key_path()),
            None,
        )
    }
}