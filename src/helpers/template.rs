use std::fs;
use std::env;
use helpers::directory;
use std::path::PathBuf;

pub struct Template {
    pub language: String,
    pub name: String,
}

impl Template {
    pub fn exists(&self) -> bool {
        self.file_path().exists()
    }

    pub fn file_path(&self) -> PathBuf {
        let mut path = Self::base_dir();

        path.push(&self.language);
        path.push(&self.name);

        path
    }

    pub fn list_files(&self) -> Vec<PathBuf> {
        directory::list_files(self.file_path())
    }

    pub fn concat_sub_dir(paths: &[&String]) -> PathBuf {
        let mut base = Self::base_dir();

        for path in paths {
            base.push(path);
        }

        base
    }

    pub fn create_sub_dir(paths: &[&String]) {
        fs::create_dir_all(Self::concat_sub_dir(paths)).expect("Could not create subdirectory");
    }

    pub fn create_dir_if_doesnt_exist(pathbuf: &PathBuf) {
        if !pathbuf.as_path().exists() {
            fs::create_dir(pathbuf).expect("Could not create directory");
        }
    }

    pub fn base_dir() -> PathBuf {
        let mut base = env::home_dir().expect("Could not find home directory");
        base.push(".diecast/");

        base
    }
}
