use std::path::PathBuf;

use clap::Args;

use crate::{cli::CliUtils, config::Config, network::NetworkUtil, templates::Template};

use super::command::{Iteration, RunCommand, error};

#[derive(Args, Debug)]
pub struct NewCommand {
    #[arg(
        help = "The template to spawn. Can also be a remote template fetched over http. If no template is passed, a fuzzy finder will open instead"
    )]
    pub template_name: Option<String>,
    #[arg(
        short,
        long,
        help = "A number specifying the version of the template to generate. Defaults to the most recent iteration"
    )]
    pub iteration: Option<Iteration>,
    #[arg(help = "The output path to spawn the template into. Defaults to the current directory")]
    pub path: Option<PathBuf>,
}

impl RunCommand for NewCommand {
    fn run(&self, config: Config) -> Result<(), super::command::CommandError> {
        let name;
        if let None = self.template_name {
            let all_existing = Template::get_existing(&config)?;
            name = CliUtils::template_fuzzy_find(all_existing)?;
        } else {
            name = self.template_name.clone().unwrap();
        }

        let spawn_path = self.path.clone().unwrap_or("./".into());
        if let Some(name) = self.template_name.clone() {
            if name.starts_with("http://") || name.starts_with("https://") {
                // Fetch from remote
                NetworkUtil::fetch_and_spawn_template(&config, name.clone(), spawn_path.clone())?;
                println!(
                    "Spawned template {} into {}",
                    name,
                    spawn_path.to_string_lossy()
                );
                return Ok(());
            }
        }
        let existing = if let Some(iteration) = self.iteration {
            Template::get_existing_by_name_and_iteration(&config, &name, iteration)?
        } else {
            Template::get_existing_by_name(&config, &name)?
        };

        if let None = existing {
            return Err(error("Template or template version not found"));
        }
        let existing = existing.unwrap();

        existing.spawn(&spawn_path);
        println!(
            "Template {} created at {}",
            &name,
            spawn_path.to_string_lossy()
        );
        return Ok(());
    }
}
