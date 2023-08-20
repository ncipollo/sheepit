use clap::Args;

#[derive(Args, Debug)]
#[command(about = "Performs a major version bump")]
pub struct MajorBumpArgs {
    #[arg(short='d', long)]
    dry_run: bool,
    #[arg(value_name = "DIR", value_hint = clap::ValueHint::DirPath)]
    working_dir: Option<std::path::PathBuf>,
}

#[derive(Args, Debug)]
#[command(about = "Performs a minor version bump")]
pub struct MinorBumpArgs {
    #[arg(short='d', long)]
    dry_run: bool,
    #[arg(value_name = "DIR", value_hint = clap::ValueHint::DirPath)]
    working_dir: Option<std::path::PathBuf>,
}

#[derive(Args, Debug)]
#[command(about = "Performs a patch version bump")]
pub struct PatchBumpArgs {
    #[arg(short='d', long)]
    dry_run: bool,
    #[arg(value_name = "DIR", value_hint = clap::ValueHint::DirPath)]
    working_dir: Option<std::path::PathBuf>,
}