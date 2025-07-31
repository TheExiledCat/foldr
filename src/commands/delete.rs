use clap::Args;

use crate::templates::Template;

use super::command::RunCommand;

#[derive(Args, Debug)]
pub struct DeleteCommand {
    name: String,
}

impl RunCommand for DeleteCommand {
    fn run(&self, config: crate::config::Config) -> Result<(), super::command::CommandError> {
        let success = Template::delete_by_name(&config, &self.name);
        return Ok(());
    }
}
