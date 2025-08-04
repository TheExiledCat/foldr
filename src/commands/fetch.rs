use clap::Args;

use crate::{commands::command::error, config::Config, network::NetworkUtil, templates::Template};

use super::command::{Result, RunCommand};

#[derive(Args, Debug)]
pub struct FetchCommand {
    pub endpoint: String,
    pub name: String,
    #[arg(short, long)]
    pub update: bool,
}

impl RunCommand for FetchCommand {
    fn run(&self, config: Config) -> Result<()> {
        let existing = Template::get_existing_by_name(&config, &self.name)?;
        let template;
        if let Some(existing) = existing {
            if !self.update {
                return Err(error(
                    "Template name already in use, use --update to update",
                ));
            }
            template = NetworkUtil::fetch_template(
                &config,
                self.endpoint.clone(),
                self.name.clone(),
                existing.info.iteration + 1,
            )?;
        } else {
            template =
                NetworkUtil::fetch_template(&config, self.endpoint.clone(), self.name.clone(), 1)?;
        }

        println!(
            "Stored template {}\nSize: {}",
            template.info.name, template.filesize
        );
        return Ok(());
    }
}
