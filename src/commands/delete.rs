use clap::Args;
use inquire::Confirm;

use crate::templates::Template;

use super::command::{Iteration, RunCommand, error};

#[derive(Args, Debug)]
pub struct DeleteCommand {
    #[arg(help = "Template to delete")]
    template_name: String,
    #[arg(
        short,
        long,
        help = "When given, only delete the specified version of the template"
    )]
    iteration: Option<Iteration>,
}

impl RunCommand for DeleteCommand {
    fn run(&self, config: crate::config::Config) -> Result<(), super::command::CommandError> {
        let confirmed = Confirm::new(&format!(
            "Are you sure you want to  delete template {}?",
            self.template_name
        ))
        .with_default(false)
        .prompt()
        .map_err(|_| error("Prompt error"))?;

        if !confirmed {
            return Err(error("Cancelled deletion"));
        }
        let success = if let Some(iteration) = self.iteration {
            Template::delete_by_name_and_iteration(&config, &self.template_name, iteration)?
        } else {
            Template::delete_by_name(&config, &self.template_name)?
        };
        if !success {
            return Err(error(&format!(
                "Unable to find template: {}",
                &self.template_name
            )));
        }
        return Ok(());
    }
}
