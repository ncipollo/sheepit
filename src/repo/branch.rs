use git2::{Error, Repository};

pub trait RepoBranches {
    fn checkout_branch(&self, repository: &Repository, branch_name: String) -> Result<(), Error>;
}

struct GithubBranches;

impl RepoBranches for GithubBranches {
    fn checkout_branch(&self, repository: &Repository, branch_name: String) -> Result<(), Error> {
        let ref_name = format!("refs/heads/{branch_name}");
        repository.set_head(&ref_name)?;
        repository.checkout_head(None)
    }
}
