use crate::error::SheepError;
use std::path::PathBuf;
use tempfile::Builder;

pub fn directory() -> Result<PathBuf, SheepError> {
    let temp = Builder::new().prefix("sheepit").tempdir()?;
    Ok(temp.keep())
}
