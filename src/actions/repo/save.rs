use colored::*;
use std::process;
use std::path::Path;
use clap::ArgMatches;
use git2::Repository;
use helpers::directory;
use helpers::template::Template;

pub fn main(matches: &ArgMatches) {
    // Unwrap is fine, since clap verifies these exist
    let arguments = matches.subcommand_matches("save-git").unwrap();
    let language = arguments.value_of("language").unwrap().to_string();
    let name = arguments.value_of("name").unwrap().to_string();
    let url = arguments.value_of("url").unwrap();
    let template = Template { language, name };

    save_repo(&template, url);
}

fn save_repo(template: &Template, url: &str) {
    verify_template_uniqueness(template);

    if Path::exists(&template.file_path()) {
        directory::remove_dir_contents(template.file_path());
    } else {
        Template::create_sub_dir(&[&template.language, &template.name]);
    }

    match Repository::clone(url, template.file_path()) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to clone: {}", e),
    };

}

fn verify_template_uniqueness(template: &Template) {
    let dir = Template::concat_sub_dir(&[&template.language, &template.name]);

    if Path::new(&dir).exists() {
        if should_replace_template() {
            directory::remove_dir_contents(dir);
        } else {
            process::exit(1);
        }
    }
}

fn should_replace_template() -> bool {
    print!(
        "{error}. Replace it, {consequence}? (y/n): ",
        error = "Template already exists".yellow(),
        consequence = "deleting existing template".red().underline(),
    );

    directory::confirm_overwrite()
}
