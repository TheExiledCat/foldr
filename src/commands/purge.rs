use clap::Args;
use itertools::Itertools;

use crate::{config::Config, templates::Template};

use super::command::{RunCommand, error};

#[derive(Args, Debug)]
pub struct PurgeCommand {
    #[arg(help = "Optional template name to purge. Defaults to all templates")]
    pub template_name: Option<String>,
}

impl RunCommand for PurgeCommand {
    fn run(&self, config: Config) -> Result<(), super::command::CommandError> {
        let existing = Template::get_existing(&config)?;
        let mut entries_deleted = 0;
        for (key, group) in &existing.iter().chunk_by(|t| t.info.name.clone()) {
            if let Some(name) = &self.template_name {
                if key != name.clone() {
                    continue;
                }
            }
            let mut all = if let Some(name) = &self.template_name {
                group
                    .filter(|g| g.info.name == name.clone())
                    .collect::<Vec<&Template>>()
            } else {
                group.collect::<Vec<&Template>>()
            };
            all.reverse();
            if all.len() == 1 {
                continue;
            }
            for i in 0..all.len() - 1 {
                Template::delete_by_name_and_iteration(
                    &config,
                    &key,
                    all.get(i).unwrap().info.iteration,
                )?;
                entries_deleted += 1;
            }
        }
        if entries_deleted == 0 {
            return Err(error("Template not found or no templates to purge"));
        }
        return Ok(());
    }
}
