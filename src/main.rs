use std::env::current_dir;
use std::fs;
use std::path::PathBuf;

mod write_zip;

fn main() {
    let root_directory: PathBuf = current_dir().expect("Could not find directory.");

    let paths: Vec<PathBuf> = fs::read_dir(&root_directory).expect("Failed to read directory.")
        .filter_map(|x| x.ok())
        .map(|x| x.path())
        .filter(|x| {
            match x.extension() {
                None => false,
                Some(ext) => ext == "cpp" || ext == "h",
            }
        })
        .collect();

    write_zip::copy_to_zip(root_directory.join("submission").with_extension("zip"), &paths);
}
