use std::env::current_dir;
use std::fs;
use std::path::PathBuf;

mod write_zip;

fn main() {
    let root_directory: PathBuf = current_dir().expect("Could not find directory.");

    let mut paths: Vec<PathBuf> = Vec::new();

    for item in fs::read_dir(&root_directory).expect("Failed to read directory.") {
        let path = item.expect("Failed to read file.").path();
        let file_extention = match path.extension() {
            None => continue,
            Some(ext) => ext,
        };

        if file_extention == "cpp" || file_extention == "h" {
            paths.push(path);
        }
    }

    write_zip::copy_to_zip(root_directory.join("submission").with_extension("zip"), &paths);
}
