use std::env::current_dir;

fn main() {
    let root_directory = current_dir().expect("Could not find directory.");

    cpp_pack::pack_project(&root_directory);
}
