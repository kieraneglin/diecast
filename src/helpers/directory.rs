use std::fs;
use std::process;
use std::path::{Path, PathBuf};
use std::io::{stdin, stdout, Write};

pub fn empty<P: AsRef<Path>>(dir: P) -> bool {
    fs::read_dir(dir).expect("Could not read directory").count() == 0
}

pub fn list_files<P: AsRef<Path>>(dir: P) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .expect("Error reading files")
        .map(|e| e.expect("Unable to parse path data").path())
        .collect() // TODO: Revisit.  Why is it so hard to get a list of files?
}

pub fn list_sub_dirs(dir: PathBuf) -> Vec<PathBuf> {
    list_files(dir)
        .into_iter()
        .filter(|template| {
            template
                .metadata()
                .expect("Unable to parse metadata")
                .is_dir()
        })
        .collect()
}

pub fn file_name(entry: &Path) -> Option<&str> {
    entry.file_name().and_then(|s| s.to_str())
}

pub fn confirm_overwrite() -> bool {
    stdout().flush().expect("Unable to flush STDOUT");
    let mut answer = String::new();
    stdin().read_line(&mut answer).expect(
        "Unable to parse input",
    );
    let answer = answer.trim_right();

    if answer == "y" {
        true
    } else if answer == "n" {
        false
    } else {
        println!("Unable to parse answer. Shutting down.");
        process::exit(1);
    }
}
