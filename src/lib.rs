use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::string::ToString;
use uuid::Uuid;
use crate::repo::clone::{GitCloner, RepoCloner};
use crate::repo::commit::{GitCommitter, RepoCommitter};
use crate::repo::options::CloneOptions;

mod repo;

pub fn sheep_test() {
    let cloner = GitCloner::new();
    let options = CloneOptions::new(
        "git@github.com:ncipollo/test-sheep.git",
        "~/Desktop/test-sheep",
    );
    let repo = cloner.clone(options)
        .expect("shouldn't fail!");

    write_test_file();

    let committer = GitCommitter::new();
    committer.commit(&repo,
                     vec![Path::new("test.txt")],
                     "test commit!");
}

fn test_file_path() -> PathBuf {
    return PathBuf::from(shellexpand::tilde("~/Desktop/test-sheep/test.txt").to_string())
}

fn write_test_file() {
    let id = Uuid::new_v4();

    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(test_file_path())
        .expect("couldn't open test file");
    write!(file, "uuid: {}", id.to_string()).expect("failed to write to test file");
    println!("uuid: {}", id.to_string())
}