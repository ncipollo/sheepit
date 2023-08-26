use std::fs::{OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::string::ToString;
use chrono::Local;
use git2::{Error, Time};
use uuid::Uuid;
use crate::repo::branch::{GitBranches};
use crate::repo::clone::{GitCloner};
use crate::repo::commit::{GitCommits};
use crate::repo::open::{GitOpener};
use crate::repo::options::CloneOptions;
use crate::repo::remote::{GitRemotes};
use crate::repo::tag::{GitTags};

mod repo;
mod token;
mod version;
mod project;
mod config;
mod error;

pub fn sheep_test() -> Result<(), Error> {
    let opener = GitOpener::new();
    let cloner = GitCloner::new();
    let test_repo_path = shellexpand::tilde("~/Desktop/test-sheep").to_string();
    let options = CloneOptions::new(
        "git@github.com:ncipollo/test-sheep.git",
        &test_repo_path,
    );


    let repo = if Path::new(&test_repo_path).exists() {
        opener.open(&test_repo_path)
            .expect("git repo open failed")
    } else {
        cloner.clone("git@github.com:ncipollo/test-sheep.git", &test_repo_path)
            .expect("clone failed")
    };

    let tagger = GitTags::new();

    let tags = tagger.get_tags(&repo)?;
    for tag in tags {
        println!("Tag: {tag}")
    }

    write_test_file();

    let committer = GitCommits::new();
    let now = Local::now();
    let commit_message = format!("test commit - {now}");
    committer.commit(&repo,
                     vec!["test.txt"],
                     &commit_message)
        .expect("failed to commit");

    tagger.create_tag(&repo, "0.0.42", None)
        .expect("failed to create tag");

    // let branches = GithubBranches::new();
    // let branch = branches.create_branch(&repo, "test_branch")?;
    // let branch_name = branch.name()?.unwrap_or_default();
    // branches.checkout_branch(&repo, branch_name)?;
    // println!("created branch: {branch_name}");

    let remotes = GitRemotes::new();
    remotes.push_branch(&repo, "main", "origin")?;
    remotes.push_tag(&repo, "0.0.42", "origin")?;

    Ok(())
}

fn clone_test() {
    let thing = "string".to_string().clone().split("t");
    let vector: Vec<&str> = thing.collect();
    let joined = vector.join(",");
    println!("{}", joined)
}

fn test_file_path() -> PathBuf {
    return PathBuf::from(shellexpand::tilde("~/Desktop/test-sheep/test.txt").to_string());
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