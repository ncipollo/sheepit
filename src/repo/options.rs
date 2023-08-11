#[derive(Debug, PartialEq)]
pub struct CloneOptions {
    pub repo_url: String,
    pub path: String
}

impl CloneOptions {
    pub fn new(repo_url: &str, path: &str) -> CloneOptions {
        CloneOptions {
            repo_url: repo_url.to_string(),
            path: shellexpand::tilde(&path).to_string()
        }
    }
}

#[cfg(test)]
mod test {
    use std::env;
    use crate::repo::options::CloneOptions;

    #[test]
    fn new_expands_tilde_path() {
        let options = CloneOptions::new("url", "~/repo");

        let home = env::var("HOME").unwrap();
        let expected_path = format!("{home}/repo");
        let expected = CloneOptions{
            repo_url: String::from("url"),
            path: expected_path
        };
        assert_eq!(expected, options)
    }
}