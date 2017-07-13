use std::fs;
use colored::*;
use std::process;
use fs_extra::dir;
use clap::ArgMatches;
use fs_extra::copy_items;
use helpers::template::Template;
use std::io::{stdin, stdout, Write};
use helpers::directory;

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
    if directory::empty(".") || should_replace_contents() {
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
    for entry in fs::read_dir(".").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            fs::remove_dir_all(path).unwrap();
        } else {
            fs::remove_file(path).unwrap();
        }
    }
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
