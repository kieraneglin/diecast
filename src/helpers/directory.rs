use std::fs;
use std::path::{Path, PathBuf};

pub fn empty<P: AsRef<Path>>(dir: P) -> bool {
    fs::read_dir(dir).unwrap().count() == 0
}

pub fn list_files<P: AsRef<Path>>(dir: P) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .expect("Error reading files")
        .map(|e| e.unwrap().path())
        .collect() // TODO: Revisit.  Why is it so hard to get a list of files?
}

pub fn list_sub_dirs(dir: PathBuf) -> Vec<PathBuf> {
    list_files(dir)
        .into_iter()
        .filter(|template| template.metadata().unwrap().is_dir())
        .collect()
}

pub fn file_name(entry: &PathBuf) -> String {
    entry
        .file_name()
        .expect("Error parsing filename")
        .to_str()
        .expect("Error parsing filename")
        .to_owned()
}