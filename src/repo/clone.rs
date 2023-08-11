use git2::Repository;

pub trait RepoCloner {
    fn clone() -> Result<Repository, git2::Error>;
}

#[derive(Debug)]
pub struct GitCloner {}

impl GitCloner {
    pub fn new() -> GitCloner {
        return GitCloner {};
    }
}

impl RepoCloner for GitCloner {
    fn clone() -> Repository {
        todo!()
    }
}