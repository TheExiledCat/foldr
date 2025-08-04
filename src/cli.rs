use crate::{
    commands::command::{Command, Result, error},
    templates::Template,
};
use clap::Parser;
use inquire::{Autocomplete, Text, autocompletion};
use itertools::Itertools;
use std::{
    io::{Write, stdin, stdout},
    path::PathBuf,
};
///Cli arguments struct for clap
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

#[derive(Clone)]
struct TextCompleter {
    pub options: Vec<String>,
}
impl TextCompleter {
    pub fn new(options: Vec<String>) -> Self {
        return Self { options };
    }
}
impl Autocomplete for TextCompleter {
    fn get_suggestions(
        &mut self,
        input: &str,
    ) -> std::result::Result<Vec<String>, inquire::CustomUserError> {
        return Ok(self
            .options
            .iter()
            .filter(|t| t.contains(input))
            .map(|s| s.to_owned())
            .collect());
    }

    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> std::result::Result<autocompletion::Replacement, inquire::CustomUserError> {
        if let Some(suggestion) = &highlighted_suggestion {
            return Ok(highlighted_suggestion);
        }
        let suggestions = self.get_suggestions(input).unwrap();
        let top_suggestion = suggestions.get(0);
        if let Some(suggestion) = top_suggestion {
            return Ok(Some(suggestion.clone()));
        }
        return Ok(None);
    }
}
impl CliUtils {
    /// a rust version of the python `input` command. used to easily display and read input from stdin
    pub fn input(message: &str) -> String {
        let stdin = stdin();
        stdout().flush().expect("IO Error");
        print!("{}\n> ", message);
        let mut text = String::new();
        stdout().flush().expect("IO Error");
        stdin.read_line(&mut text).unwrap();
        if let Some('\n') = text.chars().next_back() {
            text.pop();
        }
        if let Some('\r') = text.chars().next_back() {
            text.pop();
        }

        return text;
    }

    pub fn template_fuzzy_find(templates: Vec<Template>) -> Result<String> {
        let completer = TextCompleter::new(templates.iter().map(|t| t.info.name.clone()).collect());
        let template_name = Text::new("Please fill in the template name: ")
            .with_autocomplete(completer)
            .prompt();

        return template_name.map_err(|_| error("Fuzzy find error"));
    }
}
