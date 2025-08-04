use crate::{
    commands::command::{Command, Result, error},
    templates::Template,
};
use clap::Parser;
use inquire::{Autocomplete, Text, autocompletion};
use std::path::PathBuf;
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
            .filter(|t| t.contains(&input.trim().replace(" ", "")))
            .map(|s| s.to_owned())
            .collect());
    }

    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> std::result::Result<autocompletion::Replacement, inquire::CustomUserError> {
        if let Some(suggestion) = &highlighted_suggestion {
            return Ok(Some(suggestion.clone()));
        }
        let suggestions = self
            .get_suggestions(&input.trim().replace(" ", ""))
            .unwrap();
        let top_suggestion = suggestions.get(0);
        if let Some(suggestion) = top_suggestion {
            return Ok(Some(suggestion.clone()));
        }
        return Ok(None);
    }
}
impl CliUtils {
    pub fn template_fuzzy_find(templates: Vec<Template>) -> Result<String> {
        let completer = TextCompleter::new(
            templates
                .iter()
                .map(|t| format!("{}-{}", t.info.name.clone(), t.info.iteration))
                .collect(),
        );
        let template_name = Text::new("Please fill in the template name: ")
            .with_autocomplete(completer)
            .prompt();

        return template_name.map_err(|_| error("Fuzzy find error"));
    }
}
