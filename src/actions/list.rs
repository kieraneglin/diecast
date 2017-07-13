use clap::ArgMatches;

pub fn main(matches: &ArgMatches) {
    let arguments = matches.subcommand_matches("list").unwrap();
    let language = arguments.value_of("language");

    println!("{:#?}", language);
}
