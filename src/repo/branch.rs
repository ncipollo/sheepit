use git2::{Error, Repository};
use crate::repo::commit;
use crate::repo::reference::branch_ref_name;

pub struct GitBranches;

impl GitBranches {
    pub fn new() -> Self {
        GitBranches {}
    }

    pub fn create_branch(&self,
                     repository: &Repository,
                     branch_name: &str) -> Result<String, Error> {
        let commit = commit::find_last_commit(repository)?;
        let branch = repository.branch(&branch_name, &commit, false)?;
        let created_branch_name = branch.name()?;
        Ok(created_branch_name.unwrap_or_default().to_string())
    }

    pub fn checkout_branch(&self, repository: &Repository, branch_name: &str) -> Result<(), Error> {
        let ref_name = branch_ref_name(branch_name);
        repository.set_head(&ref_name)?;
        repository.checkout_head(None)
    }
}