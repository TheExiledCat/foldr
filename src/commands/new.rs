use std::path::PathBuf;

use clap::Args;

use crate::{config::Config, templates::Template};

use super::command::{Iteration, RunCommand, error};

#[derive(Args, Debug)]
pub struct NewCommand {
    pub name: String,
    #[arg(
        short,
        long,
        help = "A number specifying the version of the template to generate. defaults to the most recent iteration"
    )]
    pub iteration: Option<Iteration>,
    pub path: Option<PathBuf>,
}

impl RunCommand for NewCommand {
    fn run(&self, config: Config) -> Result<(), super::command::CommandError> {
        let existing = if let Some(iteration) = self.iteration {
            Template::get_existing_by_name_and_iteration(&config, &self.name, iteration)?
        } else {
            Template::get_existing_by_name(&config, &self.name)?
        };

        if let None = existing {
            return Err(error("Template or template version not found"));
        }
        let existing = existing.unwrap();
        let spawn_path = self.path.clone().unwrap_or("./".into());

        existing.spawn(&spawn_path);
        println!(
            "Template {} created at {}",
            &self.name,
            spawn_path.to_string_lossy()
        );
        return Ok(());
    }
}
