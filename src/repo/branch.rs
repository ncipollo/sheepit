use git2::{Branch, Error, Repository};
use crate::repo::commit;

pub trait RepoBranches {
    fn create_branch<'a>(&self,
                         repository: &'a Repository,
                         branch_name: &str) -> Result<Branch<'a>, Error>;
    fn checkout_branch(&self, repository: &Repository, branch_name: &str) -> Result<(), Error>;
}

pub struct GithubBranches;

impl GithubBranches {
    pub fn new() -> GithubBranches {
        GithubBranches {}
    }
}

impl RepoBranches for GithubBranches {
    fn create_branch<'a>(&self,
                         repository: &'a Repository,
                         branch_name: &str) -> Result<Branch<'a>, Error> {
        let commit = commit::find_last_commit(repository)?;
        repository.branch(&branch_name, &commit, false)
    }

    fn checkout_branch(&self, repository: &Repository, branch_name: &str) -> Result<(), Error> {
        let ref_name = format!("refs/heads/{branch_name}");
        repository.set_head(&ref_name)?;
        repository.checkout_head(None)
    }
}
