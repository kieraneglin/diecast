use std::fs;
use std::path::Path;

pub fn empty<P: AsRef<Path>>(dir: P) -> bool {
    fs::read_dir(dir).unwrap().count() == 0
}
