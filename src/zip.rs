use std::{
    fs::{self, File},
    io::{self, BufReader, Read, Write},
    path::PathBuf,
};

use walkdir::WalkDir;
use zip::{ZipArchive, ZipWriter, write::SimpleFileOptions};

use crate::{
    commands::command::{CommandError, error},
    globals,
    templates::Template,
};

pub struct ZipUtil;

impl ZipUtil {
    pub fn zip_dir(
        input_dir: &PathBuf,
        output_file: &PathBuf,
        extra_files: Vec<(PathBuf, String)>,
    ) -> Result<u64, CommandError> {
        let file = File::create(output_file).unwrap();
        let mut writer = ZipWriter::new(file);
        let options = SimpleFileOptions::default();

        // Add extra files
        for entry in extra_files {
            let path = entry.0;
            let content = entry.1;

            writer
                .start_file(path.to_string_lossy(), options)
                .map_err(|_| error("Error writing template manifest file"))?;
            writer
                .write_all(content.as_bytes())
                .map_err(|_| error("Error writing template manifest file"))?;
        }

        // Add all files and folders recursively
        for entry in WalkDir::new(input_dir) {
            let entry =
                entry.map_err(|_| error("Something went wrong traversing the template file"))?;
            let path = entry.path();
            if path.is_file() {
                let relative_path = path.strip_prefix(input_dir).unwrap();
                writer
                    .start_file(relative_path.to_string_lossy(), options)
                    .unwrap();

                let mut f = File::open(path).unwrap();
                std::io::copy(&mut f, &mut writer).unwrap();
            } else if path.is_dir() {
                // Zip doesn't require explicit folder entries, but it's OK to include them
                let relative_path = path.strip_prefix(input_dir).unwrap();
                let folder_name = format!("{}/", relative_path.to_string_lossy());
                writer
                    .add_directory(folder_name, options)
                    .map_err(|_| error("Error creating directory in template file"))?;
            }
        }

        let result = writer.finish().unwrap();
        return Ok(result
            .metadata()
            .map_err(|_e| error("Error querying output file size"))?
            .len());
    }
    pub fn get_templates(template_dir: &PathBuf) -> Result<Vec<Template>, CommandError> {
        if !template_dir.is_dir() {
            return Err(error("Template directory points to non directory path"));
        }
        let mut templates: Vec<Template> = vec![];
        for entry in WalkDir::new(template_dir).into_iter().skip(1) {
            let entry = entry.map_err(|_| error("Error traversing template file"))?;
            let path = entry.path();
            let file =
                File::open(path).map_err(|_| error("IO error while opening template file"))?;
            let size = file
                .metadata()
                .map_err(|_| error("Error while querying template file"))?
                .len();
            let mut zip = ZipArchive::new(BufReader::new(file))
                .map_err(|_| error("Error unzipping template file"))?;
            let mut manifest_file = zip
                .by_name(&globals::FOLDR_MANIFEST_FILE)
                .map_err(|_| error("Template file contains no Manifest"))?;
            let mut manifest_content = String::new();
            manifest_file
                .read_to_string(&mut manifest_content)
                .map_err(|_| error("Error reading manifest file from template"))?;
            templates.push(Template {
                info: serde_json::from_str(&manifest_content)
                    .map_err(|_| error("Template manifest file corrupt"))?,
                filename: path.to_owned(),
                filesize: bytesize::ByteSize::b(size),
            });
        }

        return Ok(templates);
    }
    pub fn unzip(template: &Template, path: &PathBuf, hide_from_output: Vec<PathBuf>) {
        let zip_to_open = &template.filename;
        let file = File::open(zip_to_open).unwrap();
        let mut zip = ZipArchive::new(file).unwrap();

        for i in 0..zip.len() {
            let mut file = zip.by_index(i).unwrap();
            if hide_from_output
                .iter()
                .find(|p| p.to_string_lossy() == file.name())
                .iter()
                .len()
                > 0
            {
                continue;
            }
            let out_path = path.join(file.name());
            if file.name().ends_with("/") {
                fs::create_dir_all(&out_path).unwrap();
            } else {
                if let Some(parent) = out_path.parent() {
                    fs::create_dir_all(parent).unwrap();
                }
                let mut out_file = File::create(&out_path).unwrap();
                io::copy(&mut file, &mut out_file).unwrap();
            }
        }
    }

    pub fn get_files(filename: PathBuf, hide_from_output: Vec<String>) -> Vec<PathBuf> {
        let file = File::open(filename).unwrap();

        let mut zip = ZipArchive::new(file).unwrap();

        let mut file_names: Vec<PathBuf> = vec![];

        for i in 0..zip.len() {
            let file = zip.by_index(i).unwrap();
            if file.name() == "/" {
                continue;
            }
            if hide_from_output.contains(
                &file
                    .enclosed_name()
                    .expect(
                        "The archive should not contain files with paths outside of the archive",
                    )
                    .to_string_lossy()
                    .into_owned(),
            ) {
                continue;
            }
            file_names.push(file.enclosed_name().expect("Dangerous zip file").into());
        }
        return file_names;
    }
}
