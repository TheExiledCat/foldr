use std::path::PathBuf;

use clap::Args;

use crate::{config::Config, templates::Template};

use super::command::{RunCommand, error};

#[derive(Args, Debug)]
pub struct UpdateCommand {
    pub name: String,
    pub directory: PathBuf,
}

impl RunCommand for UpdateCommand {
    fn run(&self, config: Config) -> Result<(), super::command::CommandError> {
        let existing = Template::get_existing_by_name(&config, &self.name);
        if let Some(template) = existing {
            let result = Template::save(
                &config,
                &self.directory,
                &self.name,
                template.info.iteration + 1,
            )?;

            println!(
                "Updated template {} to version {}",
                template.info.name, template.info.iteration
            )
        } else {
            return Err(error(&format!(
                "Template to update not found: {}",
                self.name
            )));
        }
        return Ok(());
    }
}
