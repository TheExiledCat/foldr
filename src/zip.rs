use std::{
    fs::File,
    io::{BufReader, Read, Write},
    path::PathBuf,
};

use walkdir::WalkDir;
use zip::{ZipArchive, ZipWriter, write::SimpleFileOptions};

use crate::templates::Template;

pub struct ZipUtil;

impl ZipUtil {
    pub fn zip_dir(
        input_dir: &PathBuf,
        output_file: &PathBuf,
        extra_files: Vec<(PathBuf, String)>,
    ) -> u64 {
        let file = File::create(output_file).unwrap();
        let mut writer = ZipWriter::new(file);
        let options = SimpleFileOptions::default();

        // Add extra files
        for entry in extra_files {
            let path = entry.0;
            let content = entry.1;

            writer.start_file(path.to_string_lossy(), options);
            writer.write_all(content.as_bytes());
        }

        // Add all files and folders recursively
        for entry in WalkDir::new(input_dir) {
            let entry = entry.unwrap();
            let path = entry.path();
            println!("compressing {}", path.to_string_lossy());
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
                writer.add_directory(folder_name, options);
            }
        }

        let result = writer.finish().unwrap();
        return result.metadata().unwrap().len();
    }
    pub fn get_templates(template_dir: &PathBuf) -> Vec<Template> {
        if !template_dir.is_dir() {
            return vec![];
        }
        let mut templates: Vec<Template> = vec![];
        for entry in WalkDir::new(template_dir).into_iter().skip(1) {
            let entry = entry.unwrap();
            let path = entry.path();
            let file = File::open(path).unwrap();
            let size = file.metadata().unwrap().len();
            let mut zip = ZipArchive::new(BufReader::new(file)).unwrap();
            let mut manifest_file = zip.by_name(".foldrmanifest.json").unwrap();
            let mut manifest_content = String::new();
            manifest_file.read_to_string(&mut manifest_content).unwrap();
            templates.push(Template {
                info: serde_json::from_str(&manifest_content).unwrap(),
                filename: String::from(path.to_string_lossy()),
                filesize: bytesize::ByteSize::b(size),
            });
        }

        return templates;
    }
}
