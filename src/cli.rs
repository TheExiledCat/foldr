use crate::commands::command::Command;
use clap::Parser;
use std::path::PathBuf;
#[derive(Parser, Debug)]
#[command(name = "foldr")]
#[command(version = "1.0")]
#[command(about = "foldr, the blazing fast templating tool")]
pub struct Cli {
    #[arg(short, long, name = "config")]
    pub config_path: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Command>,
}
