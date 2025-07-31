use std::{path::PathBuf, sync::Mutex};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub template_dir: PathBuf,
    pub use_cache: bool,
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
        }
    }
}
