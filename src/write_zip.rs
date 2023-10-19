use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;
use zip::write::FileOptions;
use zip::ZipWriter;

pub fn copy_to_zip<P: AsRef<Path>>(path: P, paths: &[P]) {
    let zip_file = File::create(&path).expect("Failed to create zip file.");

    let mut zip = ZipWriter::new(zip_file);

    let options = FileOptions::default();

    for file_path in paths {
        let file = File::open(file_path).expect("Failed to copy file contents");
        let file_name = file_path
            .as_ref()
            .file_name()
            .expect("Failed to get file name")
            .to_str()
            .expect("Failed to convert file name into a string.");

        zip.start_file(file_name, options)
            .expect("Failed to zip file.");

        let mut buffer = Vec::new();
        io::copy(&mut file.take(u64::MAX), &mut buffer).expect("Failed to copy data.");

        zip.write_all(&buffer).expect("Failed to write data.");
    }

    zip.finish().expect("Failed to finalize zip file");
}
