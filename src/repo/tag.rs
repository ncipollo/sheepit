use git2::{Error, Repository};
use git2::string_array::StringArray;

pub trait RepoTagger {
    fn get_tags(&self, repository: &Repository) -> Result<Vec<String>, Error>;
}

pub struct GitTagger;

impl RepoTagger for GitTagger {
    fn get_tags(&self, repository: &Repository) -> Result<Vec<String>, Error> {
        repository
            .tag_names(None)
            .map(|tag_names| {
                self.map_tag_names_to_vec(&tag_names)
            })
    }
}

impl GitTagger {
    pub fn new() -> GitTagger {
        GitTagger {}
    }

    fn map_tag_names_to_vec(&self, tag_names: &StringArray) -> Vec<String> {
        tag_names.iter().filter_map(|name| { name })
            .map(|name| String::from(name))
            .collect()
    }
}
