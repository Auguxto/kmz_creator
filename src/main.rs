use std::io::{prelude::*, BufReader};
use zip::{write::FileOptions, CompressionMethod};

fn main() {
    // create_kmz_file("faster.kmz").unwrap();
    // read_kmz_file("faster.kmz");
    create_kmz_base("src/output/", "fasternet");
}

fn create_kmz_base(path: &str, name: &str) {
    if !(std::path::Path::new(path).join(name)).exists() {
        let base_path = std::path::Path::new(path).join(name);
        std::fs::create_dir(&base_path).unwrap();

        // Create files folder
        std::fs::create_dir(base_path.join("files")).unwrap();

        // Create doc.kml
        std::fs::File::create(base_path.join("doc.kml")).unwrap();
    }
}

fn create_kmz_file(filename: &str) -> zip::result::ZipResult<()> {
    let path = std::path::Path::new(filename);
    let file = std::fs::File::create(path).unwrap();

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
    let fname = std::path::Path::new(filename);
    let file = std::fs::File::open(fname).unwrap();
    let reader = BufReader::new(file);

    let mut archive = zip::ZipArchive::new(reader).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let tt = std::fs::File::open(file.name()).unwrap();
        let mut buffer = Vec::new();

        file.read_to_end(&mut buffer).unwrap();

        println!("{}", String::from_utf8_lossy(&buffer));
    }
}
