use crate::repo::clone::{GitCloner, RepoCloner};
use crate::repo::options::CloneOptions;

mod repo;

pub fn sheep_test() {
    let cloner = GitCloner::new();
    let options = CloneOptions::new(
        "git@github.com:ncipollo/test-sheep.git",
        "~/Desktop/test-sheep"
    );
    cloner.clone(options).expect("shouldn't fail!");
}