use std::{
    fs::{self, File},
    ops::Deref,
    path::{Path, PathBuf},
};

use bytesize::ByteSize;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::zip::ZipUtil;
use crate::{commands::command::error, config::Config};
use sha2::{Digest, Sha256};

pub struct Template {
    info: TemplateInfo,
    pub filename: String,
    pub filesize: ByteSize,
}
#[derive(Serialize, Deserialize)]
pub struct TemplateInfo {
    pub name: String,
    pub iteration: usize,
}

impl Template {
    pub fn save(
        config: &Config,
        directory: &PathBuf,
        name: &str,
    ) -> Result<Template, crate::commands::command::CommandError> {
        if let Ok(exists) = fs::exists(directory) {
            if !exists {
                return Err(error("Non existent directory passed"));
            }
        } else {
            return Err(error("File IO Error while saving template"));
        }

        //load dir into memory TODO make version actually increment
        let info = TemplateInfo::new(name.into(), 1);
        let output_path = info.generate_output_path(config);
        let extra_files = vec![(
            PathBuf::from(".foldrmanifest.json"),
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
    pub fn get_existing_by_name(config: &Config, name: &str) -> Option<TemplateInfo> {
        let templates = ZipUtil::get_template_infos(&config.template_dir);

        for template in templates {
            if template.name == name {
                return Some(template);
            }
        }

        return None;
    }
    pub fn get_existing(config: &Config) -> Vec<TemplateInfo> {
        let templates = ZipUtil::get_template_infos(&config.template_dir);

        return templates;
    }
}
impl TemplateInfo {
    pub fn new(name: String, iteration: usize) -> Self {
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
