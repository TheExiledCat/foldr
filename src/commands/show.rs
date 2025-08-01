use std::path::PathBuf;

use clap::Args;

use crate::{config::Config, templates::Template};

use super::command::{RunCommand, error};

#[derive(Args, Debug)]
pub struct ShowCommand {
    pub name: String,
    #[arg(short, long)]
    pub iteration: Option<u64>,
}

impl RunCommand for ShowCommand {
    fn run(&self, config: Config) -> Result<(), super::command::CommandError> {
        let template = if let Some(iteration) = self.iteration {
            Template::get_existing_by_name_and_iteration(&config, &self.name, iteration)
        } else {
            Template::get_existing_by_name(&config, &self.name)
        };
        if let Some(template) = template {
            let root = template.get_content_hierarchy();
            println!("{}", root);
        } else {
            return Err(error("Template or template version not found"));
        }

        return Ok(());
    }
}
