pub mod operation;
mod project_version;
mod strings;
mod temp;

use crate::config::{Config, RepoConfig};
use crate::error::SheepError;
use crate::project::operation::Operation;
use crate::repo::clone::GitCloner;
use crate::repo::open::GitOpener;
use crate::repo::path;
use crate::repo::remote::GitRemotes;
use git2::Repository;
use mockall_double::double;
use std::path::Path;

#[double]
use crate::project::project_version::ProjectVersion;
use crate::project::strings::ProjectStrings;
use crate::repo::branch::GitBranches;
use crate::repo::commit::GitCommits;
use crate::repo::tag::GitTags;
use crate::script::ScriptRunner;
use crate::transform::project_transform::ProjectTransformer;
use crate::version::update::VersionUpdate;

pub struct Project {
    config: Config,
    repo: Repository,
    transformer: ProjectTransformer,
    is_dry_run_project: bool,
}

impl Project {
    pub fn new_local_project<P: AsRef<Path>>(path: P) -> Result<Project, SheepError> {
        let repo = GitOpener::new().open(&path)?;
        let config = Config::open(&path)?;
        let transformer = ProjectTransformer::new(path);
        let project = Project {
            config,
            repo,
            transformer,
            is_dry_run_project: false,
        };
        Ok(project)
    }

    pub fn new_remote_project<P: AsRef<Path>>(
        url: &str,
        directory: P,
        is_dry_run_project: bool,
    ) -> Result<Project, SheepError> {
        let repo_path = path::repo_path(url, directory)?;
        let repo = GitCloner::new().clone(url, &repo_path)?;
        let config = Config::open(&repo_path)?;
        let transformer = ProjectTransformer::new(&repo_path);
        let project = Project {
            config,
            repo,
            transformer,
            is_dry_run_project,
        };
        Ok(project)
    }

    pub fn new_dry_run_project<P: AsRef<Path>>(path: P) -> Result<Project, SheepError> {
        let remotes = GitRemotes::new();
        let local_project = Project::new_local_project(path)?;
        let remote_url = remotes.remote_url(&local_project.repo, "origin")?;
        let directory = temp::directory()?;

        let remote_project = Project::new_remote_project(&remote_url, directory, false)?;
        let dry_run_project = Project {
            config: local_project.config,
            is_dry_run_project: true,
            repo: remote_project.repo,
            transformer: remote_project.transformer,
        };
        Ok(dry_run_project)
    }

    pub fn update(&self, operation: Operation) -> Result<(), SheepError> {
        let repo_config = &self.config.repository;
        let project_version = ProjectVersion::new(&self);
        let version_update = operation.version_update(&project_version);
        let project_strings = ProjectStrings::new(&self.config, &version_update);

        self.update_repo(repo_config, &project_strings, &version_update)?;

        // Print out completion message, including dry run path if needed
        if self.is_dry_run_project {
            let mut repo_path_buf = self.repo.path().to_path_buf();
            repo_path_buf.pop(); // remove the .git path component
            let repo_path = repo_path_buf.to_string_lossy().to_string();
            println!("🐑 dry run results may be found here: {repo_path}");
        } else {
            println!("🐑 project has been sheep'd");
        }

        // Process subprojects if there are any
        self.update_subprojects(&version_update)?;

        Ok(())
    }

    fn update_repo(
        &self,
        repo_config: &RepoConfig,
        project_strings: &ProjectStrings,
        version_update: &VersionUpdate,
    ) -> Result<(), SheepError> {
        let repo = &self.repo;
        let mut working_dir = self.repo.path().to_path_buf();
        working_dir.pop(); // remove the .git path component

        let scripts = &self.config.scripts;
        let script_runner = ScriptRunner::new(working_dir, version_update.clone());

        // Create branch if enabled in configuration
        if repo_config.enable_branch {
            println!("🌲 creating branch {}", &project_strings.branch_name);
            let branches = GitBranches::new();
            branches.create_branch(repo, &project_strings.branch_name)?;
            branches.checkout_branch(repo, &project_strings.branch_name)?;
        }
        // Create commit if enabled in configuration and we have transforms
        let transforms = &self.config.transforms;
        if repo_config.enable_commit && (!transforms.is_empty() || scripts.before_commit.is_some())
        {
            println!("🤖 applying transforms");
            self.transformer.transform(transforms, version_update)?;

            script_runner.run(scripts.before_commit.clone(), "before_commit")?;

            println!("✍️  committing changes");
            let commits = GitCommits::with_default_branch(&repo_config.default_branch);
            commits.commit(repo, &project_strings.commit_message)?;
        }
        // Create tag if enabled in configuration
        if repo_config.enable_tag {
            println!("️🏷  creating tag {}", &project_strings.tag_name);
            let tags = GitTags::new();
            tags.create_tag(repo, &project_strings.tag_name, None)?;
        }
        // Push if enabled in configuration
        if repo_config.enable_push && !self.is_dry_run_project {
            println!("🚀 pushing to remote {}", &project_strings.remote_name);

            let remotes = GitRemotes::new();
            if repo_config.enable_branch {
                remotes.push_branch(
                    repo,
                    &project_strings.branch_name,
                    &project_strings.remote_name,
                )?;
            } else if repo_config.enable_commit {
                remotes.push_branch(
                    repo,
                    &repo_config.default_branch,
                    &project_strings.remote_name,
                )?;
            }
            if repo_config.enable_tag {
                remotes.push_tag(
                    repo,
                    &project_strings.tag_name,
                    &project_strings.remote_name,
                )?;
            }
        }
        Ok(())
    }

    fn update_subprojects(&self, version_update: &VersionUpdate) -> Result<(), SheepError> {
        let configs = &self.config.subprojects;
        let is_dry_run = self.is_dry_run_project;
        if configs.is_empty() {
            return Ok(());
        }

        let directory = temp::directory()?;

        for config in configs {
            let operation = Operation::SetVersion {
                current_version: Some(version_update.current_version.clone()),
                next_version: version_update.next_version.clone(),
            };
            let url = &config.repo_url;
            let project = Self::new_remote_project(url, &directory, is_dry_run)?;
            println!("------------");
            println!("🚢 sheep'n subproject {}", url);
            project.update(operation)?;
        }

        Ok(())
    }
}
