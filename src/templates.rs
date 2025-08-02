use std::{
    borrow::Cow,
    fmt::Display,
    fs::{self},
    ops::Deref,
    path::PathBuf,
    str::FromStr,
};

use bytesize::ByteSize;
use itertools::Itertools;
use ptree::{TreeBuilder, TreeItem, print_tree};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{commands::command::error, config::Config, globals::FOLDR_MANIFEST_FILE};
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

#[derive(Clone, Debug)]
pub struct TemplateHierarchy {
    pub path: PathBuf,
    pub children: Vec<TemplateHierarchy>,
}
impl TemplateHierarchy {
    pub fn new(path: PathBuf, children: Vec<TemplateHierarchy>) -> Self {
        return Self { path, children };
    }
    pub fn add_child(&mut self, child: TemplateHierarchy) {
        self.children.push(child);
    }
    pub fn from_paths(template_name: String, paths: &[PathBuf]) -> TemplateHierarchy {
        let root = PathBuf::new(); // synthetic root
        let children = Self::build_subtree(paths, &root);
        let mut root = TemplateHierarchy::new(root, children);
        root.path = template_name.into();
        return root;
    }
    fn build_subtree(paths: &[PathBuf], parent: &PathBuf) -> Vec<TemplateHierarchy> {
        let mut result = Vec::new();
        let mut i = 0;

        while i < paths.len() {
            let current_path = &paths[i];
            // Check if this path is a direct child of `parent`
            let is_direct_child = match current_path.parent() {
                Some(p) => p == parent,
                None => parent.as_os_str().is_empty(), // top-level path
            };

            if is_direct_child {
                let current = current_path.clone();
                i += 1;

                // Collect child entries of `current`
                let mut j = i;
                while j < paths.len() && paths[j].starts_with(&current) {
                    j += 1;
                }

                let children = Self::build_subtree(&paths[i..j], &current);
                result.push(TemplateHierarchy::new(current, children));
                i = j;
            } else {
                i += 1;
            }
        }

        return result;
    }
}
impl TreeItem for TemplateHierarchy {
    type Child = Self;

    fn write_self<W: std::io::Write>(
        &self,
        f: &mut W,
        style: &ptree::Style,
    ) -> std::io::Result<()> {
        return write!(
            f,
            "{}{}",
            style.paint(self.path.file_name().unwrap().to_string_lossy()),
            if self.children.len() > 0 { "/" } else { "" }
        );
    }

    fn children(&self) -> std::borrow::Cow<[Self::Child]> {
        return Cow::from(&self.children);
    }
}
impl Display for TemplateHierarchy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        print_tree(self);
        return Ok(());
    }
}

impl Template {
    pub fn spawn(&self, spawn_path: &PathBuf) {
        ZipUtil::unzip(&self, spawn_path, vec![globals::FOLDR_MANIFEST_FILE.into()]);
    }
    pub fn get_content_hierarchy(&self) -> TemplateHierarchy {
        let mut contents = ZipUtil::get_files(
            PathBuf::from_str(&self.filename).unwrap(),
            vec![FOLDR_MANIFEST_FILE.into()],
        );
        contents.sort_by_key(|p| p.to_string_lossy().into_owned());
        let root = TemplateHierarchy::from_paths(self.info.name.clone(), &contents);
        return root;
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
        templates.reverse();
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
        let mut templates = ZipUtil::get_templates(&config.template_dir);

        templates.sort_by_key(|t| t.info.iteration);
        templates.sort_by_key(|t| t.info.name.clone());
        templates.reverse();

        return templates;
    }

    pub fn delete_by_name(config: &Config, name: &str) -> bool {
        let templates = Self::get_existing(config);

        let mut found = false;
        for template in templates {
            if template.info.name == name {
                println!("Deleting template file: {}", template.filename);
                fs::remove_file(template.filename).unwrap();

                found = true;
            }
        }

        return found;
    }
    pub fn delete_by_name_and_iteration(config: &Config, name: &str, iteration: u64) -> bool {
        let templates = Self::get_existing(config);

        let mut found = false;
        for template in templates {
            if template.info.name == name && template.info.iteration == iteration {
                println!(
                    "Deleting template file: {} version {}",
                    template.info.name, template.info.iteration
                );
                fs::remove_file(template.filename).unwrap();

                found = true;
            }
        }

        return found;
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
