use colored::*;
use clap::ArgMatches;
use std::path::PathBuf;
use helpers::directory;
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
        let template_list = directory::list_sub_dirs(language_dir);

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
    let language_list = directory::list_sub_dirs(language_dir);

    for language in language_list {
        let lang_name = directory::file_name(&language);
        show_language_templates(&lang_name);
    }
}

fn print_templates_for_language(language: &str, template_list: Vec<PathBuf>) {
    println!("{}", language.cyan().bold().underline());

    for template in template_list {
        let filename = directory::file_name(&template);
        println!("  - {}", filename.blue().italic());
    }
}
