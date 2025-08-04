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
            template_dir: PathBuf::from(
                dirs::home_dir()
                    .expect("Home env variable must be set")
                    .join(".foldr/templates"),
            ),
            use_cache: true,
            require_https: false,
        }
    }
}
