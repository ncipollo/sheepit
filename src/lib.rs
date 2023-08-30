use std::path::{Path, PathBuf};
use crate::project::{Project};

pub use crate::project::operation::{BumpMode, Operation};
pub use crate::error::SheepError;

mod repo;
mod token;
mod version;
mod project;
mod config;
mod error;

pub fn project_update<P: AsRef<Path>>(operation: Operation,
                                      path: P,
                                      dry_run: bool) -> Result<(), SheepError> {
    let expanded_path = expand_path(path);
    let project = if dry_run {
        Project::new_dry_run_project(&expanded_path)?
    } else {
        Project::new_local_project(&expanded_path)?
    };

    project.update(operation)
}

fn expand_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let lossy_path = path.as_ref().to_string_lossy();
    let path_string = lossy_path.as_ref();
    let expanded = shellexpand::tilde(path_string);
    PathBuf::from(expanded.as_ref())
}

#[cfg(test)]
mod test {
    use std::env;
    use std::path::Path;
    use crate::expand_path;

    #[test]
    fn expanded_path() {
        let home = env::var("HOME").unwrap();
        let expected = format!("{home}/foo");

        let path = Path::new("~/foo");
        let expanded_path = expand_path(path);

        assert_eq!(expected, expanded_path.to_string_lossy().to_string())
    }
}