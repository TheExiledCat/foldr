use crate::templates::Template;
use clap::Args;
use itertools::Itertools;

use super::command::RunCommand;

#[derive(Args, Debug)]
pub struct ListCommand {
    pattern: Option<String>,
    #[arg(
        short,
        long,
        help = "Instead of grouping by template, flatten each version of the same template into seperate entries"
    )]
    flatten: bool,
}

impl RunCommand for ListCommand {
    fn run(&self, config: crate::config::Config) -> Result<(), super::command::CommandError> {
        let mut templates = Template::get_existing(&config);
        if templates.len() == 0 {
            println!("No templates stored yet. Create a new one using foldr save");
            return Ok(());
        }
        println!("{:<20}{:<3}", "name", "version");
        println!("{}", "-".repeat(30));
        if let Some(pattern) = &self.pattern {
            templates = templates
                .iter()
                .filter(|i| i.info.name.contains(pattern))
                .map(|t| t.clone())
                .collect();
        }
        templates.sort_by_key(|i| i.info.iteration);
        templates.sort_by_key(|i| i.info.name.clone());
        if self.flatten {
            for template in templates {
                println!("{:<20}{:<3}", template.info.name, template.info.iteration);
            }
        } else {
            for (key, group) in &templates.iter().chunk_by(|t| &t.info.name) {
                println!(
                    "{:<20}{:<3}",
                    key.to_string(),
                    group
                        .collect::<Vec<&Template>>()
                        .iter()
                        .map(|t| t.info.iteration)
                        .last()
                        .unwrap()
                );
            }
        }

        return Ok(());
    }
}
