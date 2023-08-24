use git2::{Error, Oid, Repository};
use git2::string_array::StringArray;
use crate::repo::commit;

pub struct GitTags;

impl GitTags {
    pub fn new() -> Self {
        GitTags {}
    }

    pub fn create_tag(&self,
                      repository: &Repository,
                      tag_name: &str,
                      message: Option<&str>) -> Result<Oid, Error> {
        let signature = repository.signature()?;
        let head_commit = commit::find_last_commit(&repository)?;
        match message {
            None => {
                repository.tag_lightweight(tag_name, &head_commit.into_object(), false)
            }
            Some(msg) => {
                repository.tag(tag_name,
                               &head_commit.into_object(),
                               &signature,
                               msg,
                               false)
            }
        }
    }

    pub fn get_tags(&self, repository: &Repository) -> Result<Vec<String>, Error> {
        repository
            .tag_names(None)
            .map(|tag_names| {
                self.map_tag_names_to_vec(&tag_names)
            })
    }

    fn map_tag_names_to_vec(&self, tag_names: &StringArray) -> Vec<String> {
        tag_names.iter().filter_map(|name| { name })
            .map(|name| String::from(name))
            .collect()
    }
}