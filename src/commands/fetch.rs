use std::path::PathBuf;

use clap::Args;

use crate::{config::Config, templates::Template};

use super::command::{Iteration, RunCommand, error};

#[derive(Args, Debug)]
pub struct FetchCommand {
    pub endpoint: PathBuf,
    pub name: String,
    pub iteration: Option<Iteration>,
    #[arg(short, long)]
    pub force: bool,
}

impl RunCommand for FetchCommand {
    fn run(&self, config: Config) -> Result<(), super::command::CommandError> {
        return Ok(());
    }
}
