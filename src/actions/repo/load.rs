use colored::*;
use clap::ArgMatches;
use git2::Repository;
use helpers::directory;

pub fn main(matches: &ArgMatches) {
    // Unwrap is fine, since clap verifies these exist
    let arguments = matches.subcommand_matches("load-git").unwrap();
    let url = arguments.value_of("url").unwrap();

    load_repo(url);
}

fn load_repo(url: &str) {
    if directory::empty(".") || should_replace_contents() {
        directory::remove_dir_contents(".");

        match Repository::clone(url, ".") {
            Ok(repo) => repo,
            Err(e) => panic!("Failed to clone: {}", e),
        };
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
