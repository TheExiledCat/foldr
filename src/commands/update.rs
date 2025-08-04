use std::path::PathBuf;

use clap::Args;

use crate::{config::Config, templates::Template};

use super::command::{RunCommand, error};

#[derive(Args, Debug)]
pub struct UpdateCommand {
    #[arg(help = "Template to update")]
    pub template_name: String,
    #[arg(help = "Directory to update the template with")]
    pub directory: PathBuf,
}

impl RunCommand for UpdateCommand {
    fn run(&self, config: Config) -> Result<(), super::command::CommandError> {
        let existing = Template::get_existing_by_name(&config, &self.template_name)?;
        if let Some(template) = existing {
            let _result = Template::save(
                &config,
                &self.directory,
                &self.template_name,
                template.info.iteration + 1,
            )?;

            println!(
                "Updated template {} to version {}",
                template.info.name, template.info.iteration
            )
        } else {
            return Err(error(&format!(
                "Template to update not found: {}",
                self.template_name
            )));
        }
        return Ok(());
    }
}
