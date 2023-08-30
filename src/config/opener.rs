use std::fs;
use std::path::Path;
use mockall_double::double;
use crate::config::Config;
use crate::config::finder::find_config;
#[double]
use crate::file::FileChecker;

pub fn open_config() -> Option<Config> {
    Some(Config::default())
}

fn read_config<P: AsRef<Path>>(repo_path: P) -> Option<String> {
    let file_checker = FileChecker::new();
    let config_path = find_config(&file_checker, repo_path);
    config_path.and_then(|path| fs::read_to_string(path).ok())
}