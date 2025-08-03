use clap::Subcommand;

use crate::config::Config;

use super::{
    delete::DeleteCommand, list::ListCommand, new::NewCommand, purge::PurgeCommand,
    save::SaveCommand, show::ShowCommand, update::UpdateCommand,
};

#[derive(Subcommand, Debug)]
pub enum Command {
    #[command(about = "Save a new template")]
    Save(SaveCommand),
    #[command(about = "Spawn an existing template")]
    New(NewCommand),
    #[command(about = "Install a template from an http endpoint")]
    Fetch(FetchCommand),
    #[command(about = "List all templates")]
    List(ListCommand),
    #[command(about = "List the contents of a template")]
    Show(ShowCommand),
    #[command(
        about = "Update a template with a new version. Does not overwrite the previous version"
    )]
    Update(UpdateCommand),
    #[command(about = "Purge older versions of templates")]
    Purge(PurgeCommand),
    #[command(about = "Delete all or a specific version of a template")]
    Delete(DeleteCommand),
    #[cfg(feature = "tui")]
    #[command(about = "")]
    Tui,
}
pub type Result<T> = Result<T, CommandError>;
pub fn run(command: Command, config: Config) -> Result<(), CommandError> {
    return match command {
        Command::Save(save_command) => save_command.run(config),
        Command::New(new_command) => new_command.run(config),
        Command::Fetch(fetch_command) => fetch_command.run(config),
        Command::List(list_command) => list_command.run(config),
        Command::Show(show_command) => show_command.run(config),
        Command::Update(update_command) => update_command.run(config),
        Command::Purge(purge_command) => purge_command.run(config),

        Command::Delete(delete_command) => delete_command.run(config),
        #[cfg(feature = "tui")]
        Command::Tui => todo!(),
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
