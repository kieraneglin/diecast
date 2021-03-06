use std::fs;
use colored::*;
use clap::ArgMatches;
use helpers::directory;
use helpers::template::Template;

pub fn main(matches: &ArgMatches) {
    // Unwrap is fine, since clap verifies these exist
    let arguments = matches.subcommand_matches("remove").unwrap();
    let language = arguments.value_of("language").unwrap().to_string();
    let name = arguments.value_of("name").unwrap().to_string();
    let template = Template { language, name };

    remove_template(&template);
}

fn remove_template(template: &Template) {
    if template.exists() {
        if should_remove_template() {
            fs::remove_dir_all(template.file_path()).expect("Could not remove template");

            // Remove language directory if there's no more entries in it.
            if directory::empty(Template::concat_sub_dir(&[&template.language])) {
                fs::remove_dir(Template::concat_sub_dir(&[&template.language]))
                    .expect("Could not remove parent template folder");
            }

            print_success_message(template);
        }
    } else {
        println!(
            "Template {} for {} not found.",
            template.name.yellow().italic(),
            template.language.yellow().italic()
        );
    }
}

fn print_success_message(template: &Template) {
    println!(
        "Successfully removed {} for {}.",
        &template.name.italic().green(),
        &template.language.italic().green()
    );
}

fn should_remove_template() -> bool {
    print!(
        "{action} This cannot be undone. (y/n): ",
        action = "Remove template?".red().underline(),
    );

    directory::confirm_overwrite()
}
