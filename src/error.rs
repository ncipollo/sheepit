use std::io;

#[derive(Debug, PartialEq)]
pub struct SheepError {
    message: String
}

impl SheepError {
    pub fn new(message: &str) -> Self {
        SheepError {
            message: message.to_string()
        }
    }
}

impl From<git2::Error> for SheepError {
    fn from(value: git2::Error) -> Self {
        Self {
            message: format!("git error: {value}")
        }
    }
}

impl From<io::Error> for SheepError {
    fn from(value: io::Error) -> Self {
        Self {
            message: format!("io error: {value}")
        }
    }
}

impl From<parse_git_url::FromStrError> for SheepError {
    fn from(value: parse_git_url::FromStrError) -> Self {
        Self {
            message: format!("git url parse error: {value}")
        }
    }
}
