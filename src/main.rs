use clap::{Args, Parser};
use tempfile::{Builder, TempDir};
use crate::cli::{MajorBumpArgs, MinorBumpArgs, PatchBumpArgs};

mod cli;

#[derive(Parser, Debug)] // requires `derive` feature
#[command(name = "sheepit")]
#[command(bin_name = "sheepit")]
pub enum SheepitCLI {
    Major(MajorBumpArgs),
    Minor(MinorBumpArgs),
    Patch(PatchBumpArgs),
}

fn main() {
    let temp = Builder::new().prefix("sheepit").tempdir()
        .expect("couldn't make temp");
    let temp_path = temp.into_path();
    println!("Temp path: {:?}", temp_path.to_str().unwrap());
    let command = SheepitCLI::parse();
    println!("command: {:?}", command);
}
