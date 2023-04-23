use walkdir::WalkDir;

fn main() {
    let path = "/usr";
    for entry in WalkDir::new(path)
    .max_depth(3)
    .into_iter()
    .filter_map(|e| e.ok()) {
        if entry.file_type().is_dir() {
            println!("Directory: {}", entry.path().display());
        } else {
            println!("File: {}", entry.path().display());
        }
    }
}
