use git2::{Commit, Config, Error, ObjectType, Oid, Repository, Tree};
use git2_ext::ops::{Sign, UserSign};
use std::path::Path;

pub struct GitCommits {
    default_branch: String,
}

pub fn find_last_commit(repo: &Repository) -> Result<Commit, Error> {
    let obj = repo.head()?.resolve()?.peel(ObjectType::Commit)?;
    obj.into_commit()
        .map_err(|_| Error::from_str("Couldn't find commit"))
}

impl GitCommits {
    pub fn with_default_branch(default_branch: &str) -> GitCommits {
        GitCommits {
            default_branch: String::from(default_branch),
        }
    }

    pub fn commit(&self, repository: &Repository, message: &str) -> Result<Oid, Error> {
        // Get git config and configuration
        let git_config = Config::open_default()?;
        let signature = repository.signature()?;
        // Get signing options
        let user_sign = UserSign::from_config(repository, &git_config).ok();
        let signing = user_sign.as_ref().map(|sign| sign as &dyn Sign);
        // Add paths to index and turn it into a git tree.
        let tree = self.add_paths(repository)?;
        // Get the latest commit
        let parent_commit = find_last_commit(&repository)?;
        // Create the optionally signed commit.
        git2_ext::ops::commit(
            repository,
            &signature,
            &signature,
            message,
            &tree,
            &[&parent_commit],
            signing,
        )
        .and_then(|commit_id| {
            // Update the repository's head so it points to this new commit.
            self.update_head(repository, commit_id, message)
        })
    }

    fn add_paths<'a>(&'a self, repository: &'a Repository) -> Result<Tree, Error> {
        let mut index = repository.index()?;
        index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;
        let oid = index.write_tree()?;
        repository.find_tree(oid)
    }

    fn update_head(
        &self,
        repository: &Repository,
        commit_id: Oid,
        message: &str,
    ) -> Result<Oid, Error> {
        let head = repository.head()?;
        let branch = head.shorthand().unwrap_or(&self.default_branch);
        repository.reference(&format!("refs/heads/{}", branch), commit_id, true, message)?;
        Ok(commit_id)
    }
}

#[cfg(test)]
mod test {
    use crate::repo::commit::GitCommits;

    #[test]
    fn git_committer_with_default_branch() {
        let committer = GitCommits::with_default_branch("develop");
        assert_eq!("develop", committer.default_branch)
    }
}
