const KMZ_BASE_OUTPUT: &str = "src/output";
const KMZ_ZIP_OUTPUT: &str = "src/output_kmz";

use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;
use walkdir::WalkDir;
use zip::{write::FileOptions, CompressionMethod};

fn main() {
    create_kmz_base("fasternet", "Teste de KMZ");
    create_kmz_file("fasternet");
}

fn create_kmz_base(name: &str, description: &str) {
    // Check if kmz folder with name alerady exists
    match Path::new(KMZ_BASE_OUTPUT).join(name).exists() {
        true => {
            // println!("kmz {} dir already exists", name);
        }
        false => {
            // Path on which the zip file will be created
            let base_path = Path::new(KMZ_BASE_OUTPUT).join(name);

            // Create kmz dir
            std::fs::create_dir(&base_path).unwrap();

            // Create files dir in kmz
            std::fs::create_dir(base_path.join("files")).unwrap();

            // Create doc.kml
            let mut doc = File::create(base_path.join("doc.kml")).unwrap();

            // Read base_kml.txt
            let mut base_doc_file = File::open("src/data/base_kml.txt").unwrap();

            let mut base_doc_file_contents = String::new();

            // Write base_kml.txt base_doc_file_contents
            base_doc_file
                .read_to_string(&mut base_doc_file_contents)
                .unwrap();

            // Replace placeholders
            base_doc_file_contents = base_doc_file_contents
                .replace("{name}", name)
                .replace("{description}", description);

            // Write base_doc_file_contents to doc.kml
            doc.write_all(base_doc_file_contents.as_bytes()).unwrap();
        }
    }
}

fn create_kmz_file(filename: &str) {
    // Check if kmz folder with name exists
    match Path::new(KMZ_BASE_OUTPUT).join(filename).exists() {
        true => {
            // Path on wich the zip file will be created
            let path = Path::new(KMZ_ZIP_OUTPUT).join(String::from(filename) + ".kmz");

            // Create zip file
            let file: File = File::create(path).unwrap();

            // Walk dir on kmz base output
            let kmz_dir = WalkDir::new(String::from(KMZ_BASE_OUTPUT) + "/" + filename)
                .min_depth(1)
                .into_iter()
                .filter_map(|e| e.ok());

            // Create zip
            let mut zip = zip::ZipWriter::new(&file);

            // Set compression method and unix permissions
            let options = FileOptions::default()
                .compression_method(CompressionMethod::Stored)
                .unix_permissions(0o755);

            // For each entry in kmz base output
            for entry in kmz_dir {
                // Check if entry is dir
                if entry.file_type().is_dir() {
                    // Create dir path
                    let dir_path = entry
                        .path()
                        .display()
                        .to_string()
                        .replace(&(String::from(KMZ_BASE_OUTPUT) + "/" + filename + "/"), "");

                    // Add dir to zip
                    zip.add_directory(dir_path, Default::default()).unwrap();
                }
                // Check if entry is file
                if entry.file_type().is_file() {
                    // Create file path
                    let file_path = entry
                        .path()
                        .display()
                        .to_string()
                        .replace(&(String::from(KMZ_BASE_OUTPUT) + "/" + filename + "/"), "");

                    // Read file content
                    let file_content = fs::read(entry.path()).unwrap();
                    // Add file to zip
                    zip.start_file(file_path, options).unwrap();
                    // Write file content
                    zip.write_all(&file_content).unwrap();
                }
            }
            zip.finish().unwrap();
        }
        false => {
            println!("kmz {} dir  not exists", filename);
        }
    }
}
