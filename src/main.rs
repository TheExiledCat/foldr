use std::process::ExitCode;

use clap::{CommandFactory, Parser};
use commands::command::run;

mod cli;
mod commands;
mod config;
mod globals;
mod network;
mod templates;
mod zip;

#[cfg(feature = "tui")]
mod tui;

fn main() -> ExitCode {
    let cli = cli::Cli::parse();
    let command = cli.command;
    if let None = command {
        let _ = cli::Cli::command().print_help();
        return ExitCode::FAILURE;
    }

    let command = command.unwrap();
    if let Err(err) = run(command, config::Config::default()) {
        println!("Something went wrong during the operation: {}", err.message);
        return ExitCode::FAILURE;
    }

    return ExitCode::SUCCESS;
}
