use crate::commands::command::Command;
use clap::Parser;
use std::{
    io::{Write, stdin, stdout},
    path::PathBuf,
};
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

pub struct CliUtils;

impl CliUtils {
    pub fn input(message: &str) -> String {
        let stdin = stdin();
        stdout().flush();
        print!("{}\n> ", message);
        let mut text = String::new();
        stdout().flush();
        stdin.read_line(&mut text).unwrap();
        if let Some('\n') = text.chars().next_back() {
            text.pop();
        }
        if let Some('\r') = text.chars().next_back() {
            text.pop();
        }

        return text;
    }
}
