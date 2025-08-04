use std::path::PathBuf;

use clap::Args;

use crate::{
    config::{Config, ExpandablePathBuf},
    templates::Template,
};

use super::command::{RunCommand, error};

#[derive(Args, Debug)]
pub struct SaveCommand {
    #[arg(help = "The directory to save as a template")]
    pub directory: PathBuf,
    #[arg(help = "The name for the template. Must be unique")]
    pub name: String,
}

impl RunCommand for SaveCommand {
    fn run(&self, config: Config) -> Result<(), super::command::CommandError> {
        let existing = Template::get_existing_by_name(&config, &self.name)?;
        if let Some(_template) = existing {
            return Err(error(&format!(
                "Template with the same name already exists: {}\nUse foldr update to overwrite",
                &self.name
            )));
        }
        let result = Template::save(&config, &self.directory.expand(), &self.name, 1)?;
        println!(
            "Created template: {}\nSize: {}",
            result.filename.to_string_lossy(),
            result.filesize
        );
        return Ok(());
    }
}
