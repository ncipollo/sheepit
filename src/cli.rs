use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
use clap::Args;

#[derive(Args, Debug)]
#[command(about = "Performs a major version bump")]
pub struct MajorBumpArgs {
    #[arg(short = 'd', long)]
    pub dry_run: bool,
    #[
    arg(value_name = "repo",
    default_value=".",
    value_hint = clap::ValueHint::DirPath)
    ]
    pub repo_path: PathBuf,
}

#[derive(Args, Debug)]
#[command(about = "Performs a minor version bump")]
pub struct MinorBumpArgs {
    #[arg(short = 'd', long)]
    pub dry_run: bool,
    #[
    arg(value_name = "repo",
    default_value=get_default_path(),
    value_hint = clap::ValueHint::DirPath)
    ]
    pub repo_path: PathBuf,
}

#[derive(Args, Debug)]
#[command(about = "Performs a patch version bump")]
pub struct PatchBumpArgs {
    #[arg(short = 'd', long)]
    pub dry_run: bool,
    #[
    arg(value_name = "repo",
    default_value=get_default_path(),
    value_hint = clap::ValueHint::DirPath)
    ]
    pub repo_path: PathBuf,
}

fn get_default_path() -> OsString {
    env::current_dir()
        .expect("unable to get working directory")
        .into_os_string()
}