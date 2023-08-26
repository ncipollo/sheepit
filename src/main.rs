use clap::{Args, Parser};
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
    let command = SheepitCLI::parse();
    println!("command: {:?}", command);
}
