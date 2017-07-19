use std::fs;
use colored::*;
use std::process;
use fs_extra::dir;
use clap::ArgMatches;
use helpers::directory;
use fs_extra::copy_items;
use helpers::template::Template;

pub fn main(matches: &ArgMatches) {
    // Unwrap is fine, since clap verifies these exist
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
        remove_dir_contents();
        copy_template(template);
        print_success_message(template);
    }
}

fn copy_template(template: &Template) {
    let files = template.list_files();
    let copy_options = dir::CopyOptions::new();

    copy_items(&files, ".", &copy_options).expect("Unable to copy template to current directory");
}

fn remove_dir_contents() {
    for entry in fs::read_dir(".").expect("Unable to read current directory") {
        let entry = entry.expect("Unable to parse directory entry");
        let path = entry.path();

        if path.is_dir() {
            fs::remove_dir_all(path).expect("Unable to delete subdirectory");
        } else {
            fs::remove_file(path).expect("Unable to delete file");
        }
    }
}

fn should_replace_contents() -> bool {
    print!(
        "{error}. {action} in this directory and replace it with a template? (y/n): ",
        error = "This directory not empty".yellow(),
        action = "Delete everything".red().underline(),
    );

    directory::confirm_overwrite()
}

fn print_success_message(template: &Template) {
    println!(
        "Successfully loaded {} for {}.",
        &template.name.italic().green(),
        &template.language.italic().green()
    );
}
