use std::path::PathBuf;

use clap::Subcommand;
use reqwest::Url;

use crate::config::Config;

use super::{delete::DeleteCommand, list::ListCommand, new::NewCommand, save::SaveCommand};

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(about = "Save a new template")]
    Save(SaveCommand),
    #[command(about = "Spawn an existing template")]
    New(NewCommand),
    #[command(about = "Install a template from an http endpoint")]
    Fetch {},
    #[command(about = "List all templates")]
    List(ListCommand),
    #[command(about = "List the contents of a template")]
    Show {},
    #[command(
        about = "Update a template with a new version. Does not overwrite the previous version"
    )]
    Update {},
    #[command(about = "Purge older versions of templates")]
    Purge {},
    #[command(about = "Delete all versions of a template")]
    Delete(DeleteCommand),
    #[cfg(feature = "tui")]
    #[command(about = "")]
    Tui,
}

pub fn run(command: Command, config: Config) -> Result<(), CommandError> {
    return match command {
        Command::Save(save_command) => save_command.run(config),
        Command::New(new_command) => new_command.run(config),
        Command::Fetch {} => todo!(),
        Command::List(list_command) => list_command.run(config),
        Command::Show {} => todo!(),
        Command::Update {} => todo!(),
        Command::Purge {} => todo!(),
        #[cfg(feature = "tui")]
        Command::Tui => todo!(),
        Command::Delete(delete_command) => delete_command.run(config),
    };
}
pub struct CommandError {
    pub message: String,
}
pub fn error(message: &str) -> CommandError {
    return CommandError {
        message: message.into(),
    };
}
pub trait RunCommand {
    fn run(&self, config: Config) -> Result<(), CommandError>;
}
