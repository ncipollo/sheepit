use crate::SheepError;
use git2::{Cred, Error};
use std::path::{Path, PathBuf};
use std::{env, fs};

/// Returns the standard path to the user's ssh key.
fn ssh_key_path() -> String {
    path_from_env().unwrap_or_else(|| find_best_ssh_key().expect("failed to find ssh key"))
}

fn path_from_env() -> Option<String> {
    let path = env::var("SHEEPIT_SSH_KEY_PATH");
    path.ok().map(|p| shellexpand::tilde(&p).to_string())
}

fn find_best_ssh_key() -> Result<String, SheepError> {
    let ssh_dir = shellexpand::tilde("~/.ssh");
    let ssh_dir_path = PathBuf::from(ssh_dir.as_ref());
    let file_names = ssh_file_names(ssh_dir_path)?;

    let best_file_name = find_best_key_name(file_names)?;
    let mut base_path = PathBuf::from(ssh_dir.as_ref());
    base_path.push(best_file_name);
    Ok(base_path.to_string_lossy().to_string())
}

fn ssh_file_names(ssh_dir_path: PathBuf) -> Result<Vec<String>, SheepError> {
    let names = fs::read_dir(ssh_dir_path)?
        .map(|res| res.map(|e| e.file_name()))
        .map({ |res| res.map_or("".to_string(), |name| name.to_string_lossy().to_string()) })
        .collect::<Vec<_>>();
    Ok(names)
}

fn find_best_key_name(file_names: Vec<String>) -> Result<String, SheepError> {
    file_names
        .iter()
        .find(|name| !name.ends_with(".pub") && name.starts_with("id_"))
        .map(|name| name.to_string())
        .ok_or(SheepError::new("failed to find ssh key"))
}

pub fn add_credentials_to_callbacks(remote_callbacks: &mut git2::RemoteCallbacks) {
    remote_callbacks
        .credentials(|_, username_from_url, _| create_ssh_key(username_from_url.unwrap()));
}

pub fn create_ssh_key(username_from_url: &str) -> Result<Cred, Error> {
    let ssh_key_path = ssh_key_path();
    Cred::ssh_key(username_from_url, None, Path::new(&ssh_key_path), None)
}

#[cfg(test)]
mod test {
    use crate::repo::ssh::find_best_key_name;

    #[test]
    fn find_best_key_name_empty_names() {
        let result = find_best_key_name(vec![]);
        result.expect_err("should be an error");
    }

    #[test]
    fn find_best_key_name_typical_ssh() {
        let names = vec!["config", "id_rsa", "id_rsa.pub", "known_hosts"];
        let result = find_best_key_name(names.iter().map(|n| n.to_string()).collect());
        let name = result.expect("failed to find file name");
        assert_eq!("id_rsa", name)
    }
}
