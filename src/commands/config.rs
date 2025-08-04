use std::{
    fs::{self, File},
    io::Write,
    path::PathBuf,
    time::Duration,
};

use clap::Args;
use indicatif::ProgressBar;
use inquire::{self, Text};

use crate::{
    commands::command::error,
    config::{Config, ExpandablePathBuf},
    globals,
};

use super::command::Result;
#[derive(Args, Debug, Clone)]
pub struct ConfigCommand {
    #[arg(short, long)]
    output: Option<PathBuf>,
}

impl ConfigCommand {
    pub fn generate_config(&self) -> Result<()> {
        println!("Starting config generation process");
        let mut config_location;

        if let Some(path) = self.output.clone() {
            config_location = path;
        } else {
            config_location = PathBuf::from(
                Text::new("Please fill in which directory to generate the configuration file: ")
                    .with_default(globals::FOLDR_CONFIG_DIR)
                    .prompt()
                    .map_err(|_| error("Invalid directory"))?,
            );
        }
        config_location = config_location.expand();

        if !config_location.exists() {
            fs::create_dir_all(&config_location)
                .map_err(|_| error("Failure to create config directory"))?;
        }

        println!("Generating config.json");
        let bar = ProgressBar::new_spinner();
        bar.enable_steady_tick(Duration::from_millis(100));

        let default_config = Config::default();
        let output_file_content =
            serde_json::to_string_pretty(&default_config).map_err(|_| error("Serializer error"))?;
        let mut output_file = File::create(config_location.join("config.json"))
            .map_err(|_| error("IO Error generating config file"))?;
        output_file
            .write_all(output_file_content.as_bytes())
            .map_err(|_| error("Failed to write config file"))?;
        println!(
            "Config.json generated at {}",
            config_location.join("config.json").to_string_lossy()
        );
        return Ok(());
    }
}
