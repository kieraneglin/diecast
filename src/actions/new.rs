use std::fs;
use glob::glob;
use colored::*;
use std::process;
use std::path::Path;
use clap::ArgMatches;
use fs_extra::remove_items;
use helpers::template::Template;
use std::io::{stdin, stdout, Write};

pub fn main(matches: &ArgMatches) {
    let arguments = matches.subcommand_matches("load").unwrap();
    let language = arguments.value_of("language").unwrap().to_string();
    let name = arguments.value_of("name").unwrap().to_string();
    let template = Template { language, name };

    // Ensure the dir where templates are stored exists
    Template::create_dir_if_doesnt_exist(&Template::base_dir());
    Template::create_sub_dir(&[&template.language]);

    verify_template_uniqueness(&template);
    copy_directory_to_template(&template);
    print_success_message(&template);
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

    for entry in glob("**/*").unwrap().filter_map(Result::ok) {
        let filepath = &entry.display().to_string();
        let metadata = Path::new(&filepath).metadata().unwrap();

        let mut destination = template.filepath();
        destination.push(filepath);

        if metadata.is_dir() {
            Template::create_dir_if_doesnt_exist(&destination);
        } else {
            fs::copy(&filepath, destination).unwrap();
        }
    }
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
