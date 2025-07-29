use clap::Args;

use crate::templates::Template;

use super::command::RunCommand;

#[derive(Args, Debug)]
pub struct ListCommand {}

impl RunCommand for ListCommand {
    fn run(&self, config: crate::config::Config) -> Result<(), super::command::CommandError> {
        let infos = Template::get_existing(&config);
        for info in infos {
            println!("{:>20}{:>3}", info.name, info.iteration);
        }

        return Ok(());
    }
}
