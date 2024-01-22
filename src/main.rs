use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;
use zip::{write::FileOptions, CompressionMethod};

fn main() {
    // create_kmz_file("faster.kmz").unwrap();
    // read_kmz_file("faster.kmz");
    create_kmz_base("src/output/", "fasternet", "Teste de KMZ");
}

fn create_kmz_base(path: &str, name: &str, description: &str) {
    // Check if kmz folder with name alerady exists
    match Path::new(path).join(name).exists() {
        true => {
            println!("kmz {} dir already exists", name);
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

fn create_kmz_file(filename: &str) -> zip::result::ZipResult<()> {
    let path = Path::new(filename);
    let file = File::create(path).unwrap();

    let mut zip = zip::ZipWriter::new(file);

    zip.add_directory("files/", Default::default())?;

    let options = FileOptions::default()
        .compression_method(CompressionMethod::Stored)
        .unix_permissions(0o755);

    zip.start_file("files/success.txt", options)?;
    zip.write_all(b"Success zip created\n")?;

    zip.finish()?;

    Ok(())
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
