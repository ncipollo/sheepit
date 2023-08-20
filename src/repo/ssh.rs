use std::path::Path;
use git2::{Cred, Error};

/// Returns the standard path to the user's ssh key.
pub fn default_ssh_key_path() -> String {
    shellexpand::tilde("~/.ssh/id_rsa").to_string()
}

pub fn add_credentials_to_callbacks(remote_callbacks: &mut git2::RemoteCallbacks) {
    remote_callbacks.credentials(|_, username_from_url, _| {
        create_ssh_key(username_from_url.unwrap())
    });
}

pub fn create_ssh_key(username_from_url: &str) -> Result<Cred, Error> {
    Cred::ssh_key(
        username_from_url,
        None,
        Path::new(&default_ssh_key_path()),
        None,
    )
}

#[cfg(test)]
mod test {
    use std::env;
    use crate::repo::ssh::default_ssh_key_path;

    #[test]
    fn default_path_expands_tilde() {
        let home = env::var("HOME").unwrap();
        let expected = format!("{home}/.ssh/id_rsa");
        assert_eq!(expected, default_ssh_key_path())
    }
}