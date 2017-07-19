use clap::ArgMatches;
use git2::Repository;

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
    match Repository::clone(url, ".") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };
}
