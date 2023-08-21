use git2::{Direction, Error, PushOptions, RemoteCallbacks, Repository};
use crate::repo::{reference, ssh};

pub struct GitRemotes;

impl GitRemotes {
    pub fn new() -> Self {
        GitRemotes {}
    }

    pub fn push_branch(&self, repository: &Repository,
                   branch_name: &str,
                   remote_name: &str) -> Result<(), Error> {
        let ref_name = reference::branch_ref_name(branch_name);
        self.push_ref(repository, &ref_name, remote_name)
    }

    pub fn push_tag(&self, repository: &Repository,
                tag_name: &str,
                remote_name: &str) -> Result<(), Error> {
        let ref_name = reference::tag_ref_name(tag_name);
        self.push_ref(repository, &ref_name, remote_name)
    }

    fn push_ref(&self, repository: &Repository,
                ref_name: &str,
                remote_name: &str) -> Result<(), Error> {
        // Prepare credentials for remote connection.
        let mut callbacks = RemoteCallbacks::new();
        ssh::add_credentials_to_callbacks(&mut callbacks);

        // Connect to remote with authentication
        let mut remote = repository.find_remote(remote_name)?;
        remote.connect_auth(Direction::Push, Some(callbacks), None)?;

        // Prepare push options with authentication.
        let mut push_callbacks = RemoteCallbacks::new();
        ssh::add_credentials_to_callbacks(&mut push_callbacks);
        let mut push_options = PushOptions::new();
        push_options.remote_callbacks(push_callbacks);

        // Push to remote
        let ref_spec = format!("{ref_name}:{ref_name}");
        remote.push(&[ref_spec], Some(&mut push_options))
    }
}