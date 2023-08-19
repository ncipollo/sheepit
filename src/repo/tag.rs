use git2::{Error, Oid, Repository};
use git2::string_array::StringArray;
#[cfg(test)]
use mockall::{automock};
use crate::repo::commit;

#[cfg_attr(test, automock)]
pub trait RepoTags {
    fn create_tag(&self,
                  repository: &Repository,
                  tag_name: &str,
                  message: &str) -> Result<Oid, Error>;
    fn get_tags(&self, repository: &Repository) -> Result<Vec<String>, Error>;
}

pub struct GitTags;

impl GitTags {
    pub fn new() -> GitTags {
        GitTags {}
    }

    fn map_tag_names_to_vec(&self, tag_names: &StringArray) -> Vec<String> {
        tag_names.iter().filter_map(|name| { name })
            .map(|name| String::from(name))
            .collect()
    }
}


impl RepoTags for GitTags {
    fn create_tag(&self,
                  repository: &Repository,
                  tag_name: &str,
                  message: &str) -> Result<Oid, Error> {
        let signature = repository.signature()?;
        let head_commit = commit::find_last_commit(&repository)?;
        repository.tag(tag_name,
                       &head_commit.into_object(),
                       &signature,
                       message,
                       false)
    }

    fn get_tags(&self, repository: &Repository) -> Result<Vec<String>, Error> {
        repository
            .tag_names(None)
            .map(|tag_names| {
                self.map_tag_names_to_vec(&tag_names)
            })
    }
}