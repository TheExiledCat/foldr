use clap::Args;

use crate::{config::Config, templates::Template};

use super::command::{Iteration, RunCommand, error};

#[derive(Args, Debug)]
pub struct ShowCommand {
    #[arg(help = "The template to show the contents for")]
    pub template_name: String,
    #[arg(
        short,
        long,
        help = "The version of the template to show. Defaults to the most recent version"
    )]
    pub iteration: Option<Iteration>,
}

impl RunCommand for ShowCommand {
    fn run(&self, config: Config) -> Result<(), super::command::CommandError> {
        let template = if let Some(iteration) = self.iteration {
            Template::get_existing_by_name_and_iteration(&config, &self.template_name, iteration)?
        } else {
            Template::get_existing_by_name(&config, &self.template_name)?
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
