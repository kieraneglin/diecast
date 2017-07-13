use std::fs;
use colored::*;
use glob::glob;
use std::process;
use fs_extra::dir;
use std::path::Path;
use clap::ArgMatches;
use fs_extra::dir::ls;
use fs_extra::copy_items;
use std::collections::HashSet;
use helpers::template::Template;
use std::io::{stdin, stdout, Write};

pub fn main(matches: &ArgMatches) {
    let arguments = matches.subcommand_matches("load").unwrap();
    let language = arguments.value_of("language").unwrap().to_string();
    let name = arguments.value_of("name").unwrap().to_string();
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
    if dir_empty(".") || should_replace_contents() {
        delete_dir_contents();
        copy_template(template);
        print_success_message(template);
    }
}

fn copy_template(template: &Template) {
    let files = template.files();
    let copy_options = dir::CopyOptions::new();

    copy_items(&files, ".", &copy_options).unwrap();
}

fn delete_dir_contents() {
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
    result.unwrap().items.is_empty()
}

fn should_replace_contents() -> bool {
    print!(
        "{error}. {action} in this directory and replace it with a template? (y/n): ",
        error = "This directory not empty".yellow(),
        action = "Delete everything".red().underline(),
    );

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

fn print_success_message(template: &Template) {
    println!(
        "Successfully loaded {} for {}.",
        &template.name.italic().green(),
        &template.language.italic().green()
    );
}
