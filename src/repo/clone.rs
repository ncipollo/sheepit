use std::path::Path;
use git2;
use crate::repo::ssh::default_ssh_key_path;
use super::options::CloneOptions;

pub trait RepoCloner {
    fn clone(options: CloneOptions) -> Result<git2::Repository, git2::Error>;
}

pub struct GitCloner<'a> {
    remote_callbacks: git2::RemoteCallbacks<'a>,
}

impl<'a> GitCloner<'a> {
    pub fn new() -> GitCloner<'a> {
        let mut remote_callbacks = git2::RemoteCallbacks::new();
        remote_callbacks.credentials(|_, username_from_url, _| {
            GitCloner::create_ssh_key(username_from_url.unwrap())
        });
        return GitCloner { remote_callbacks };
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

impl<'a> RepoCloner for GitCloner<'a> {
    fn clone(options: CloneOptions) -> Result<git2::Repository, git2::Error> {
        todo!()
    }
}