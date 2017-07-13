use colored::*;
use std::process;
use fs_extra::dir;
use std::path::Path;
use clap::ArgMatches;
use helpers::directory;
use fs_extra::copy_items;
use fs_extra::remove_items;
use helpers::template::Template;
use std::io::{stdin, stdout, Write};

pub fn main(matches: &ArgMatches) {
    let arguments = matches.subcommand_matches("new").unwrap();
    let language = arguments.value_of("language").unwrap().to_string();
    let name = arguments.value_of("name").unwrap().to_string();
    let template = Template { language, name };

    create_new_template(&template);
}

fn create_new_template(template: &Template) {
    Template::create_sub_dir(&[&template.language]);

    verify_template_uniqueness(template);
    copy_directory_to_template(template);
    print_success_message(template);
}

fn verify_template_uniqueness(template: &Template) {
    let dir = Template::concat_sub_dir(&[&template.language, &template.name]);

    if Path::new(&dir).exists() {
        if should_replace_template() {
            remove_items(&vec![dir]).unwrap();
        } else {
            process::exit(1);
        }
    }
}

fn copy_directory_to_template(template: &Template) {
    Template::create_sub_dir(&[&template.language, &template.name]);

    let copy_options = dir::CopyOptions::new();
    let file_list = directory::list_files(".");

    copy_items(&file_list, template.file_path(), &copy_options).unwrap();
}

fn should_replace_template() -> bool {
    print!(
        "{error}. Replace it, {consequence}? (y/n): ",
        error = "Template already exists".yellow(),
        consequence = "deleting existing template".red().underline(),
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
        "Template {} created for {}.",
        &template.name.italic().green(),
        &template.language.italic().green()
    );
}
