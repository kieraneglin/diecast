use std::fs;
use colored::*;
use std::path::PathBuf;
use clap::ArgMatches;
use helpers::template::Template;

pub fn main(matches: &ArgMatches) {
    let arguments = matches.subcommand_matches("list").unwrap();
    let language = arguments.value_of("language");

    match language {
        Some(lang) => show_language_templates(lang),
        None => show_all_language_templates(),
    }
}

fn show_language_templates(language: &str) {
    let mut language_dir = Template::base_dir();
    language_dir.push(language);

    if language_dir.exists() {
        let template_list = get_dirs_in_path(language_dir);

        print_templates_for_language(language, template_list);
    } else {
        println!(
            "Language {} not found in template list.",
            language.yellow().italic()
        );
    }
}

fn show_all_language_templates() {
    let language_dir = Template::base_dir();
    let language_list = get_dirs_in_path(language_dir);

    for language in language_list {
        let lang_name = language.file_name().unwrap().to_str().unwrap(); // TODO: Make this better
        show_language_templates(lang_name);
    }
}

fn print_templates_for_language(language: &str, template_list: Vec<PathBuf>) {
    println!("{}", language.cyan().bold().underline());

    for template in template_list {
        let filename = template.file_name().unwrap().to_str().unwrap(); // TODO: Make this better
        println!("  - {}", filename.blue().italic());
    }
}

fn get_dirs_in_path(dir: PathBuf) -> Vec<PathBuf> {
    fs::read_dir(dir)
        .unwrap()
        .map(|template| template.unwrap().path())
        .filter(|template| template.metadata().unwrap().is_dir())
        .collect()
}
