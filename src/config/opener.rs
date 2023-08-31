use std::fs;
use std::path::Path;
use mockall_double::double;
use crate::config::Config;
use crate::config::finder::find_config;
#[double]
use crate::file::FileChecker;
use crate::SheepError;

pub fn open_config<P: AsRef<Path>>(repo_path: P) -> Result<Config, SheepError> {
    let config_text = read_config(repo_path);
    match config_text {
        None => Ok(Config::default()),
        Some(text) => {
            let config: Config = toml::from_str(&text)?;
            Ok(config)
        }
    }
}

fn read_config<P: AsRef<Path>>(repo_path: P) -> Option<String> {
    let file_checker = FileChecker::new();
    let config_path = find_config(&file_checker, repo_path);
    config_path.and_then(|path| fs::read_to_string(path).ok())
}