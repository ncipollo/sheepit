use std::path::PathBuf;
use tempfile::Builder;
use crate::error::SheepError;

pub fn directory() -> Result<PathBuf, SheepError> {
    let temp = Builder::new().prefix("sheepit").tempdir()?;
    Ok(temp.into_path())
}