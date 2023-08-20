use git2::{Error, Repository};
use crate::repo::commit;
#[cfg(test)]
use mockall::{automock};
use crate::repo::reference::branch_ref_name;

#[cfg_attr(test, automock)]
pub trait RepoBranches {
    fn create_branch(&self,
                     repository: &Repository,
                     branch_name: &str) -> Result<String, Error>;
    fn checkout_branch(&self, repository: &Repository, branch_name: &str) -> Result<(), Error>;
}

pub struct GithubBranches;

impl GithubBranches {
    pub fn new() -> GithubBranches {
        GithubBranches {}
    }
}

impl RepoBranches for GithubBranches {
    fn create_branch(&self,
                     repository: &Repository,
                     branch_name: &str) -> Result<String, Error> {
        let commit = commit::find_last_commit(repository)?;
        let branch = repository.branch(&branch_name, &commit, false)?;
        let created_branch_name = branch.name()?;
        Ok(created_branch_name.unwrap_or_default().to_string())
    }

    fn checkout_branch(&self, repository: &Repository, branch_name: &str) -> Result<(), Error> {
        let ref_name = branch_ref_name(branch_name);
        repository.set_head(&ref_name)?;
        repository.checkout_head(None)
    }
}
