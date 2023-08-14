use std::path::Path;
use git2::{Commit, Config, Error, ObjectType, Oid, Repository};
use git2_ext::ops::{Sign, UserSign};

pub trait RepoCommitter {
    fn commit<P: AsRef<Path>>(&self, repo: &Repository, paths: Vec<P>, message: &str) -> Result<Oid, Error>;
}

pub struct GitCommitter;

impl RepoCommitter for GitCommitter {
    fn commit<P: AsRef<Path>>(&self, repo: &Repository, paths: Vec<P>, message: &str) -> Result<Oid, Error> {
        let git_config = Config::open_default()?;
        let binding = UserSign::from_config(repo, &git_config).ok();
        let signing = binding.as_ref().map(|sign| sign as &dyn Sign);
        let signature = repo.signature()?;

        let mut index = repo.index()?;
        paths.iter().for_each(|path| {
            index.add_path(path.as_ref()).expect("failed to add path")
        });
        let oid = index.write_tree()?;
        let tree = repo.find_tree(oid)?;

        let parent_commit = find_last_commit(&repo)?;

        let commit_result = git2_ext::ops::commit(
            repo,
            &signature,
            &signature,
            message,
            &tree,
            &[&parent_commit],
            signing,
        );

        if let Ok(commit) = commit_result {
            let head = repo.head()?;
            let branch = head.shorthand().unwrap_or("master");
            repo.reference(
                &format!("refs/heads/{}", branch),
                commit,
                true,
                message
            )?;
        }

        commit_result
    }
}

impl GitCommitter {
    pub fn new() -> GitCommitter { GitCommitter {} }
}

fn find_last_commit(repo: &Repository) -> Result<Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit().map_err(|_| Error::from_str("Couldn't find commit"))
}