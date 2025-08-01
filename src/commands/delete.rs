use clap::Args;

use crate::{cli::CliUtils, templates::Template};

use super::command::{RunCommand, error};

#[derive(Args, Debug)]
pub struct DeleteCommand {
    name: String,
    #[arg(
        short,
        long,
        help = "When given, only delete the specified version of the template"
    )]
    iteration: Option<u64>,
}

impl RunCommand for DeleteCommand {
    fn run(&self, config: crate::config::Config) -> Result<(), super::command::CommandError> {
        let confirmed = CliUtils::input(&format!(
            "Are you sure you want to  delete template {}? (y/n)",
            self.name
        ));

        if confirmed != "y" && confirmed != "Y" {
            return Err(error("Cancelled deletion"));
        }
        let success = if let Some(iteration) = self.iteration {
            Template::delete_by_name_and_iteration(&config, &self.name, iteration)
        } else {
            Template::delete_by_name(&config, &self.name)
        };
        if !success {
            return Err(error(&format!("Unable to find template: {}", &self.name)));
        }
        return Ok(());
    }
}
