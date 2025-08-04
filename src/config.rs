use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    /// Directory where template are stored
    pub template_dir: PathBuf,
    /// Whether to use the sqlite database as a cache to speed up template queries
    pub use_cache: bool,
    /// Requires https when fetching from remote template repositories
    pub require_https: bool,
}

impl Config {
    pub fn default() -> Self {
        Self {
            template_dir: PathBuf::from("~/.foldr/templates"),
            use_cache: true,
            require_https: false,
        }
        .ensure_created()
    }

    fn ensure_created(self) -> Self {
        std::fs::create_dir_all(&self.template_dir.expand()).unwrap();
        return self;
    }
}

pub trait ExpandablePathBuf {
    fn expand(&self) -> Self;
}
impl ExpandablePathBuf for PathBuf {
    fn expand(&self) -> Self {
        return PathBuf::from(shellexpand::tilde(&self.display().to_string()).into_owned());
    }
}
