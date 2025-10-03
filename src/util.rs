use glob::glob;
use std::path::{Path, PathBuf};

pub fn list_categories(sounds_dir: &Path) -> impl Iterator<Item = String> {
    list_category_directories(sounds_dir).filter_map(|f| get_category_name(&f))
}

pub fn list_category_directories(sounds_dir: &Path) -> impl Iterator<Item = PathBuf> {
    list_children(sounds_dir).filter(|path| path.is_dir()) // only directories
}

pub fn list_children(path: &Path) -> impl Iterator<Item = PathBuf> {
    let pattern = path.join("*");
    let pattern_str = pattern.to_str().expect("Non-UTF8 path not supported");

    glob(pattern_str)
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok) // skip invalid entries
}

pub fn get_category_name(path: &Path) -> Option<String> {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|s| s.to_string())
}
