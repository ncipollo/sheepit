/// Returns the standard path to the user's ssh key.
fn default_ssh_key_path() -> String {
    shellexpand::tilde("~/.ssh/id_rsa").to_string()
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