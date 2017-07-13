#[macro_use]
extern crate clap;

extern crate fs_extra;
extern crate colored;
extern crate glob;

mod actions;
mod helpers;

use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Unwrap is safe because clap ensures there's a value
    match matches.subcommand_name().unwrap() {
        "new" => actions::new::main(&matches),
        "load" => actions::load::main(&matches),
        "list" => actions::list::main(&matches),
        _ => unreachable!(),
    }
}
