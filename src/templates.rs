use std::{
    borrow::Cow,
    fmt::Display,
    fs::{self, File},
    io::{Read, Seek, Write},
    ops::Deref,
    path::PathBuf,
};

use bytesize::ByteSize;
use ptree::{TreeItem, print_tree};
use serde::{Deserialize, Serialize};
use serde_json::json;
use zip::{ZipArchive, ZipWriter, write::SimpleFileOptions};

use crate::{
    commands::command::{Iteration, Result, error},
    config::Config,
    globals::FOLDR_MANIFEST_FILE,
};
use crate::{globals, zip::ZipUtil};
use sha2::{Digest, Sha256};

/// Represents a template, including its file data and manifest
#[derive(Clone)]
pub struct Template {
    pub info: TemplateInfo,
    pub filename: PathBuf,
    pub filesize: ByteSize,
}

/// Represents a template manifest, stored inside of the template file
#[derive(Clone, Serialize, Deserialize)]
pub struct TemplateInfo {
    pub name: String,
    pub iteration: Iteration,
}

/// Represents the inner contents of a template file as a file hierarchy
#[derive(Clone, Debug)]
pub struct TemplateHierarchy {
    pub path: PathBuf,
    pub children: Vec<TemplateHierarchy>,
}
impl TemplateHierarchy {
    pub fn new(path: PathBuf, children: Vec<TemplateHierarchy>) -> Self {
        return Self { path, children };
    }
    /// Build a hierarchy from a sorted PathBuf slice
    pub fn from_paths(template_name: String, paths: &[PathBuf]) -> TemplateHierarchy {
        let root = PathBuf::new(); // synthetic root
        let children = Self::build_subtree(paths, &root);
        let mut root = TemplateHierarchy::new(root, children);
        root.path = template_name.into();
        return root;
    }
    /// Builds the hierarchy for any child paths
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

/// Implementation for pretty printing trees
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
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _result = print_tree(self);

        return Ok(());
    }
}

impl Template {
    /// Spawn the template at [`spawn_path`]
    pub fn spawn(&self, spawn_path: &PathBuf) {
        ZipUtil::unzip(&self, spawn_path, vec![globals::FOLDR_MANIFEST_FILE.into()]);
    }
    /// Get the hierarchy of a template
    pub fn get_content_hierarchy(&self) -> TemplateHierarchy {
        let mut contents =
            ZipUtil::get_files(self.filename.clone(), vec![FOLDR_MANIFEST_FILE.into()]);
        contents.sort_by_key(|p| p.to_string_lossy().into_owned());
        let root = TemplateHierarchy::from_paths(self.info.name.clone(), &contents);
        return root;
    }

    /// Create a new template from a directory
    pub fn save(
        config: &Config,
        directory: &PathBuf,
        name: &str,
        iteration: Iteration,
    ) -> Result<Template> {
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
        let filesize = ZipUtil::zip_dir(directory, &output_path, extra_files)?;

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
    pub fn get_existing_by_name(config: &Config, name: &str) -> Result<Option<Template>> {
        let mut templates = ZipUtil::get_templates(&config.template_dir)?;
        templates.sort_by_key(|t| t.info.iteration);
        templates.reverse();
        for template in templates {
            if template.info.name == name {
                return Ok(Some(template));
            }
        }

        return Ok(None);
    }
    /// Get an existing template stored in the template directory by name an version number
    pub fn get_existing_by_name_and_iteration(
        config: &Config,
        name: &str,
        iteration: Iteration,
    ) -> Result<Option<Template>> {
        let templates = ZipUtil::get_templates(&config.template_dir)?;
        for template in templates {
            if template.info.name == name && template.info.iteration == iteration {
                return Ok(Some(template));
            }
        }

        return Ok(None);
    }
    /// Get all existing templates in the template directory
    pub fn get_existing(config: &Config) -> Result<Vec<Template>> {
        let mut templates = ZipUtil::get_templates(&config.template_dir)?;

        templates.sort_by_key(|t| t.info.iteration);
        templates.sort_by_key(|t| t.info.name.clone());
        templates.reverse();

        return Ok(templates);
    }
    /// Delete all iterations of a template by name
    pub fn delete_by_name(config: &Config, name: &str) -> Result<bool> {
        let templates = Self::get_existing(config)?;

        let mut found = false;
        for template in templates {
            if template.info.name == name {
                println!(
                    "Deleting template file: {}",
                    template.filename.to_string_lossy()
                );
                fs::remove_file(template.filename).unwrap();

                found = true;
            }
        }

        return Ok(found);
    }
    /// Delete a single iteration of a template by name and version number
    pub fn delete_by_name_and_iteration(
        config: &Config,
        name: &str,
        iteration: Iteration,
    ) -> Result<bool> {
        let templates = Self::get_existing(config)?;

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

        return Ok(found);
    }
    /// Store an existing template file from a stream into a new template file. This generates a new manifest.
    pub fn store<R: Read + Seek>(
        config: &Config,
        name: String,
        iteration: Iteration,
        mut stream: R,
        remove_from_output: Vec<PathBuf>,
    ) -> Result<Template> {
        let mut input_zip =
            ZipArchive::new(&mut stream).map_err(|_| error("Template file is corrupt"))?;
        let info = TemplateInfo::new(name, iteration);
        let output_file_path = info.generate_output_path(config);
        let mut output_file = File::create(&output_file_path)
            .map_err(|_| error("IO error creating template output file"))?;
        let mut output_zip = ZipWriter::new(&mut output_file);

        for i in 0..input_zip.len() {
            let mut file = input_zip.by_index(i).unwrap();
            let file_name;
            if file.name() == "/" {
                continue;
            }
            if let Some(name) = file.enclosed_name() {
                file_name = name;
            } else {
                return Err(error(
                    "Template file contains files trying to escape its path. Template might be harmful",
                ));
            }
            if remove_from_output.contains(&file_name) {
                continue;
            }
            output_zip
                .start_file_from_path(file_name.as_path(), SimpleFileOptions::default())
                .map_err(|_| error("IO Error creating file inside of output template file"))?;
            let mut buffer = Vec::<u8>::new();
            file.read_to_end(&mut buffer).map_err(|_| {
                error(&format!(
                    "IO Error reading file {}",
                    file_name.to_string_lossy()
                ))
            })?;
            output_zip.write_all(&buffer).map_err(|_| {
                error(&format!(
                    "IO Error writing file {} from template",
                    file_name.to_string_lossy()
                ))
            })?;
        }

        output_zip
            .start_file(FOLDR_MANIFEST_FILE, SimpleFileOptions::default())
            .map_err(|_| error("IO Error creating manifest file in output template"))?;

        output_zip
            .write_all(serde_json::to_string_pretty(&info).unwrap().as_bytes())
            .map_err(|_| error("IO Error writing manifest file in output template"))?;
        output_zip
            .finish()
            .map_err(|_| error("Failure to compress template file on disk"))?;

        println!("Copied template data to disk");
        let size = output_file
            .metadata()
            .map_err(|_| error("Failure to get template metadata. Template still written."))?
            .len();
        return Ok(Template {
            info,
            filename: output_file_path.clone(),
            filesize: ByteSize::b(size),
        });
    }
}
impl TemplateInfo {
    pub fn new(name: String, iteration: Iteration) -> Self {
        return Self { name, iteration };
    }
    // TODO error handling
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
