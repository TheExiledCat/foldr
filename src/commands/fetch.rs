use clap::Args;

use crate::{commands::command::error, config::Config, network::NetworkUtil, templates::Template};

use super::command::{Result, RunCommand};

#[derive(Args, Debug)]
pub struct FetchCommand {
    #[arg(help = "The endpoint to download the template from. Must be http(s)")]
    pub endpoint: String,
    #[arg(help = "The name used to store the downloaded template")]
    pub template_name: String,
    #[arg(
        short,
        long,
        help = "If set, update an already existing template with the fetched template. Does nothing if there is no existing template with the same name"
    )]
    pub update: bool,
}

impl RunCommand for FetchCommand {
    fn run(&self, config: Config) -> Result<()> {
        let existing = Template::get_existing_by_name(&config, &self.template_name)?;
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
                self.template_name.clone(),
                existing.info.iteration + 1,
            )?;
        } else {
            template = NetworkUtil::fetch_template(
                &config,
                self.endpoint.clone(),
                self.template_name.clone(),
                1,
            )?;
        }

        println!(
            "Stored template {}\nSize: {}",
            template.info.name, template.filesize
        );
        return Ok(());
    }
}
