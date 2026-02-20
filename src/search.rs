use std::{
    env,
    fs::read_to_string,
    path::{Path, PathBuf},
};

pub struct Searcher {
    search_paths: Vec<PathBuf>,
}

impl Searcher {
    pub fn new(prepend: Vec<PathBuf>, append: Vec<PathBuf>) -> Option<Searcher> {
        let mut search_paths: Vec<PathBuf> = prepend.to_owned();
        match env::var("TEMPLATO_SEARCH_PATHS") {
            Ok(var) => {
                for i in env::split_paths(&var) {
                    search_paths.push(i);
                }
            }
            Err(_) => {
                search_paths.push(std::env::home_dir()?.join("Templates"));
            }
        }
        search_paths.extend(append);

        Some(Searcher { search_paths })
    }

    pub fn find(&self, path: impl AsRef<Path>) -> Option<String> {
        for i in &self.search_paths {
            let path: PathBuf = i.join(path.as_ref());
            let Ok(data) = read_to_string(path) else {
                continue;
            };
            return Some(data);
        }
        None
    }
}
