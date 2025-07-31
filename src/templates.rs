use std::{
    fs::{self},
    ops::Deref,
    path::PathBuf,
};

use bytesize::ByteSize;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{commands::command::error, config::Config};
use crate::{globals, zip::ZipUtil};
use sha2::{Digest, Sha256};

#[derive(Clone)]
pub struct Template {
    pub info: TemplateInfo,
    pub filename: String,
    pub filesize: ByteSize,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct TemplateInfo {
    pub name: String,
    pub iteration: u64,
}

impl Template {
    pub fn spawn(&self, spawn_path: &PathBuf) {
        let with_root = !spawn_path.exists();

        ZipUtil::unzip(&self, spawn_path, vec![globals::FOLDR_MANIFEST_FILE.into()]);
    }
    pub fn save(
        config: &Config,
        directory: &PathBuf,
        name: &str,
        iteration: u64,
    ) -> Result<Template, crate::commands::command::CommandError> {
        if let Ok(exists) = fs::exists(directory) {
            if !exists {
                return Err(error("Non existent directory passed"));
            }
        } else {
            return Err(error("File IO Error while saving template"));
        }

        //load dir into memory TODO make version actually increment
        let info = TemplateInfo::new(name.into(), iteration);
        let output_path = info.generate_output_path(config);
        let extra_files = vec![(
            PathBuf::from(globals::FOLDR_MANIFEST_FILE),
            json!(info).to_string(),
        )];
        let filesize = ZipUtil::zip_dir(directory, &output_path, extra_files);

        return Ok(Template {
            info,
            filename: output_path
                .file_name()
                .unwrap()
                .to_string_lossy()
                .deref()
                .into(),
            filesize: ByteSize::b(filesize),
        });
    }
    pub fn get_existing_by_name(config: &Config, name: &str) -> Option<Template> {
        let mut templates = ZipUtil::get_templates(&config.template_dir);
        templates.sort_by_key(|t| t.info.iteration);
        for template in templates {
            if template.info.name == name {
                return Some(template);
            }
        }

        return None;
    }
    pub fn get_existing_by_name_and_iteration(
        config: &Config,
        name: &str,
        iteration: u64,
    ) -> Option<Template> {
        let templates = ZipUtil::get_templates(&config.template_dir);
        for template in templates {
            if template.info.name == name && template.info.iteration == iteration {
                return Some(template);
            }
        }

        return None;
    }
    pub fn get_existing(config: &Config) -> Vec<Template> {
        let templates = ZipUtil::get_templates(&config.template_dir);

        return templates;
    }

    pub fn delete_by_name(config: &Config, name: &str) -> bool {
        let templates = Self::get_existing(config);

        for template in templates {
            if template.info.name == name {
                println!("Deleting template file: {}", template.filename);
                fs::remove_file(template.filename).unwrap();
            }
        }

        return true;
    }
}
impl TemplateInfo {
    pub fn new(name: String, iteration: u64) -> Self {
        return Self { name, iteration };
    }
    pub fn generate_output_path(&self, config: &Config) -> PathBuf {
        let output_dir = &config.template_dir;
        let output_file = format!(
            "{}/{}-{}.foldr",
            output_dir.to_string_lossy(),
            format!("{:x}", Sha256::digest(&self.name.as_bytes())),
            self.iteration
        )
        .into();
        fs::create_dir_all(output_dir).unwrap();
        return output_file;
    }
}
