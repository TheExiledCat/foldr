use std::{fs, path::PathBuf};

use clap::Args;

use crate::{
    config::Config,
    templates::{self, Template},
};

use super::command::{RunCommand, error};

#[derive(Args, Debug)]
pub struct SaveCommand {
    pub directory: PathBuf,
    pub name: String,
}

impl RunCommand for SaveCommand {
    fn run(&self, config: Config) -> Result<(), super::command::CommandError> {
        let result = Template::save(&config, &self.directory, &self.name)?;
        println!(
            "Created template: {}\nSize: {}",
            result.filename, result.filesize
        );
        return Ok(());
    }
}
