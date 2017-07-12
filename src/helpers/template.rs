use std::fs;
use std::env;
use std::path::{Path, PathBuf};

pub struct Template {
    pub language: String,
    pub name: String,
}

impl Template {
    pub fn exists(&self) -> bool {
        self.filepath().exists()
    }

    pub fn filepath(&self) -> PathBuf {
        let mut path = Self::base_dir();
        path.push(&self.language);
        path.push(&self.name);

        path
    }

    pub fn concat_sub_dir(paths: &[&String]) -> PathBuf {
        let mut base = Self::base_dir();
        for path in paths {
            base.push(path);
        }

        base
    }

    pub fn create_sub_dir(paths: &[&String]) {
        fs::create_dir_all(Self::concat_sub_dir(paths));
    }

    pub fn create_dir_if_doesnt_exist(pathbuf: &PathBuf) {
        if !pathbuf.as_path().exists() {
            fs::create_dir(pathbuf);
        }
    }

    pub fn base_dir() -> PathBuf {
        let mut base = env::home_dir().unwrap();
        base.push(".diecast/");
        base
    }
}
