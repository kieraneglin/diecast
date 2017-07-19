use colored::*;
use clap::ArgMatches;
use git2::Repository;
use helpers::directory;

pub fn main(matches: &ArgMatches) {
    // Unwrap is fine, since clap verifies these exist
    let arguments = matches.subcommand_matches("repo").unwrap();
    let action = arguments.value_of("action").unwrap();
    let url = arguments.value_of("url").unwrap();

    match action {
        "load" => load_repo(url),
        _ => unreachable!(),
    }
}

fn load_repo(url: &str) {
    if directory::empty(".") || should_replace_contents() {
        match Repository::clone(url, ".") {
            Ok(repo) => repo,
            Err(e) => panic!("failed to clone: {}", e),
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
