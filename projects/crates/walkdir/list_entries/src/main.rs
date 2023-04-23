use walkdir::WalkDir;

fn main() {
    for entry in WalkDir::new("/usr")
    .into_iter()
    .filter_map(|e| e.ok()) {
        if entry.file_type().is_dir() {
            println!("Directory: {}", entry.path().display());
        } else {
            println!("File: {}", entry.path().display());
        }
    }
}
