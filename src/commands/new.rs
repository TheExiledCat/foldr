use std::{fs, path::PathBuf};

use clap::Args;

use crate::{
    config::Config,
    templates::{self, Template},
};

use super::command::{RunCommand, error};

#[derive(Args, Debug)]
pub struct NewCommand {
    pub name: String,
    pub iteration: Option<u64>,
}

impl RunCommand for NewCommand {
    fn run(&self, config: Config) -> Result<(), super::command::CommandError> {
        let existing = Template::get_existing_by_name(&config, &self.name);
        if let Some(template) = existing {
            return Err(error(&format!(
                "Template with the same name already exists: {}",
                &self.name
            )));
        }
        let result = Template::save(&config, &self.directory, &self.name)?;
        println!(
            "Created template: {}\nSize: {}",
            result.filename, result.filesize
        );
        return Ok(());
    }
}
