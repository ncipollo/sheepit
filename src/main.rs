use clap::{Args, Parser};
use sheepit::{BumpMode, Operation, project_update, SheepError};
use crate::cli::{MajorBumpArgs, MinorBumpArgs, PatchBumpArgs};

mod cli;

#[derive(Parser, Debug)]
#[command(name = "sheepit", version)]
pub enum SheepitCLI {
    Major(MajorBumpArgs),
    Minor(MinorBumpArgs),
    Patch(PatchBumpArgs),
}

fn main() -> Result<(), SheepError> {
    let command = SheepitCLI::parse();
    match command {
        SheepitCLI::Major(args) => {
            let operation = Operation::BumpVersion(BumpMode::Major);
            project_update(operation, args.repo_path, args.dry_run)?
        }
        SheepitCLI::Minor(args) => {
            let operation = Operation::BumpVersion(BumpMode::Minor);
            project_update(operation, args.repo_path, args.dry_run)?
        }
        SheepitCLI::Patch(args) => {
            let operation = Operation::BumpVersion(BumpMode::Patch);
            project_update(operation, args.repo_path, args.dry_run)?
        }
    };
    Ok(())
}
