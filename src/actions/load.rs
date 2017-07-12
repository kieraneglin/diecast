use std::fs;
use colored::*;
use std::process;
use glob::glob;
use std::path::Path;
use clap::ArgMatches;
use fs_extra::dir::ls;
use std::collections::HashSet;
use helpers::template::Template;
use std::io::{stdin, stdout, Write};

pub fn main(matches: &ArgMatches) {
    let language = matches.value_of("language").unwrap().to_string();
    let name = matches.value_of("name").unwrap().to_string();
    let template = Template { language, name };

    load_template(&template);
}

fn load_template(template: &Template) {
    if template.exists() {
        replace_dir_contents(template);
    } else {
        println!(
            "Template {} not found in {}.",
            &template.name.italic().yellow(),
            &template.language.italic().yellow()
        );
        process::exit(1);
    }
}

fn replace_dir_contents(template: &Template) {
    if dir_empty("../sandbox") || should_replace_contents() {
        delete_directory_contents();
    }
}

fn delete_directory_contents() {
    for entry in glob("*").unwrap().filter_map(Result::ok) {
        let filepath = &entry.display().to_string();
        let metadata = Path::new(&filepath).metadata().unwrap();

        if metadata.is_dir() {
            fs::remove_dir_all(filepath).unwrap();
        } else {
            fs::remove_file(filepath).unwrap();
        }
    }
}

fn dir_empty(path: &str) -> bool {
    let result = ls(path, &HashSet::new());
    result.unwrap().items.len() == 0
}

fn should_replace_contents() -> bool {
    print!("Directory not empty. Delete contents and replace with template? (y/n): ");
    stdout().flush().unwrap();
    let mut answer = String::new(); // TODO: Revisit. Reading input can't actually be this hard
    stdin().read_line(&mut answer).unwrap();
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
