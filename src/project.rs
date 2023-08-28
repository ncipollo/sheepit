mod dryrun;
pub mod operation;
mod project_version;

use std::path::{Path, PathBuf};
use git2::{Repository};
use mockall_double::double;
use crate::config::Config;
use crate::error::SheepError;
use crate::project::operation::Operation;
use crate::repo::clone::GitCloner;
use crate::repo::open::{GitOpener};
use crate::repo::path;
use crate::repo::remote::GitRemotes;

#[double]
use crate::project::project_version::ProjectVersion;

pub struct Project {
    config: Config,
    repo: Repository,
    is_dry_run_project: bool,
}

impl Project {
    pub fn new_local_project<P: AsRef<Path>>(path: P) -> Result<Project, SheepError> {
        let repo = GitOpener::new().open(path)?;
        let config = Config {};
        let project = Project {
            config,
            repo,
            is_dry_run_project: false,
        };
        Ok(project)
    }

    pub fn new_remote_project<P: AsRef<Path>>(url: &str, directory: P) -> Result<Project, SheepError> {
        let repo_path = path::repo_path(url, directory)?;
        let repo = GitCloner::new().clone(url, repo_path)?;
        let config = Config {};
        let project = Project {
            config,
            repo,
            is_dry_run_project: false,
        };
        Ok(project)
    }

    pub fn new_dry_run_project<P: AsRef<Path>>(path: P) -> Result<Project, SheepError> {
        let remotes = GitRemotes::new();
        let local_project = Project::new_local_project(path)?;
        let remote_url = remotes.remote_url(&local_project.repo, "origin")?;
        let directory = dryrun::directory()?;

        let remote_project = Project::new_remote_project(&remote_url, directory)?;
        let dry_run_project = Project {
            config: local_project.config,
            is_dry_run_project: true,
            repo: remote_project.repo,
        };
        Ok(dry_run_project)
    }

    pub fn update(&self, operation: Operation) -> Result<ProjectUpdateInfo, SheepError> {
        let project_version = ProjectVersion::new(&self);
        let version_update = operation.version_update(&project_version);

        // First get the version update information based upon operations type

        // Next create project strings object based upon configuration & version update

        // Create branch if enabled in configuration

        // Create commit if enabled in configuration

        // Create tag if enabled in configuration

        // Push if enabled in configuration

        // Process subprojects if there are any

        // Return project info
        let repo_path = self.repo.path();
        Ok(ProjectUpdateInfo::new(repo_path))
    }
}

pub struct ProjectUpdateInfo {
    pub repo_path: PathBuf,
}

impl ProjectUpdateInfo {
    fn new(repo_path: &Path) -> ProjectUpdateInfo {
        ProjectUpdateInfo {
            repo_path: repo_path.to_path_buf()
        }
    }
}