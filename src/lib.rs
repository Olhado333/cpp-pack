use std::ffi::OsStr;
use std::fs;
use std::path::{PathBuf, Path};

mod write_zip;

fn valid_extention(ext: &OsStr) -> bool {
    ["cpp", "h", "txt"].iter().map(OsStr::new).any(|x| ext == x)
}

pub fn pack_project(dir: &Path) {
    let paths: Vec<PathBuf> = fs::read_dir(dir)
        .expect("Failed to read directory.")
        .filter_map(|x| x.ok())
        .map(|x| x.path())
        .filter(|x| match x.extension() {
            None => false,
            Some(ext) => valid_extention(ext),
        })
        .collect();

    write_zip::copy_to_zip(
        dir.join("submission").with_extension("zip"),
        &paths,
    );
}


