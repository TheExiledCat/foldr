use clap::Args;

use crate::templates::Template;

use super::command::RunCommand;

#[derive(Args, Debug)]
pub struct ListCommand {
    pattern: Option<String>,
}

impl RunCommand for ListCommand {
    fn run(&self, config: crate::config::Config) -> Result<(), super::command::CommandError> {
        let mut templates = Template::get_existing(&config);
        println!("{:<20}{:<3}", "name", "version");
        println!("{}", "-".repeat(30));
        templates.sort_by_key(|i| i.info.name.clone());
        if templates.len() == 0 {
            println!("No templates stored yet. Create a new one using foldr save");
            return Ok(());
        }

        for info in if let Some(pattern) = &self.pattern {
            templates
                .iter()
                .filter(|i| i.info.name.contains(pattern))
                .map(|i| i.clone())
                .collect()
        } else {
            templates
        } {
            println!("{:<20}{:<3}", info.info.name, info.info.iteration);
        }

        return Ok(());
    }
}
