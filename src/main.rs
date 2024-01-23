use std::fs::{self, File};
use std::io::{prelude::*, BufReader};
use std::path::Path;
use walkdir::WalkDir;
use zip::{write::FileOptions, CompressionMethod};

fn main() {
    // create_kmz_file("faster.kmz").unwrap();
    // read_kmz_file("faster.kmz");
    create_kmz_base("src/output/", "fasternet", "Teste de KMZ");
    create_kmz_file("src/output_kmz/", "fasternet");
}

fn create_kmz_base(path: &str, name: &str, description: &str) {
    // Check if kmz folder with name alerady exists
    match Path::new(path).join(name).exists() {
        true => {
            // println!("kmz {} dir already exists", name);
        }
        false => {
            let base_path = Path::new(path).join(name);

            // Create base dir
            std::fs::create_dir(&base_path).unwrap();

            // Create files dir
            std::fs::create_dir(base_path.join("files")).unwrap();

            // Create doc.kml
            let mut doc = File::create(base_path.join("doc.kml")).unwrap();

            // Read base_kml.txt
            let mut base_doc_file = File::open("src/data/base_kml.txt").unwrap();

            let mut base_doc_file_contents = String::new();

            base_doc_file
                .read_to_string(&mut base_doc_file_contents)
                .unwrap();

            // Replace placeholders
            base_doc_file_contents = base_doc_file_contents
                .replace("{name}", name)
                .replace("{description}", description);

            // Write base_kml.txt to doc.kml
            doc.write_all(base_doc_file_contents.as_bytes()).unwrap();
        }
    }
}

fn create_kmz_file(path: &str, filename: &str) {
    match Path::new("src/output").join(filename).exists() {
        true => {
            let path = Path::new("src/output_kmz").join(String::from(filename) + ".kmz");
            let file: File = File::create(path).unwrap();
            let kmz_dir = WalkDir::new(String::from("src/output") + "/" + filename)
                .min_depth(1)
                .into_iter()
                .filter_map(|e| e.ok());

            let mut zip = zip::ZipWriter::new(&file);

            let options = FileOptions::default()
                .compression_method(CompressionMethod::Stored)
                .unix_permissions(0o755);

            for entry in kmz_dir {
                if entry.file_type().is_file() {
                    let file_content = fs::read(entry.path()).unwrap();
                    zip.start_file(entry.file_name().to_str().unwrap(), options)
                        .unwrap();
                    zip.write_all(&file_content).unwrap();
                }
            }

            // zip.add_directory("files/", Default::default()).unwrap();

            // let options = FileOptions::default()
            //     .compression_method(CompressionMethod::Stored)
            //     .unix_permissions(0o755);

            // zip.start_file("files/success.txt", options).unwrap();
            // zip.write_all(b"Success zip created\n").unwrap();

            // zip.finish().unwrap();
        }
        false => {
            println!("kmz {} dir  not exists", filename);
        }
    }
}

fn read_kmz_file(filename: &str) {
    let fname = Path::new(filename);
    let file = File::open(fname).unwrap();
    let reader = BufReader::new(file);

    let mut archive = zip::ZipArchive::new(reader).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let tt = File::open(file.name()).unwrap();
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer).unwrap();

        println!("{}", String::from_utf8_lossy(&buffer));
    }
}
