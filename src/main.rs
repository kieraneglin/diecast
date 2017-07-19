#[macro_use]
extern crate clap;

extern crate fs_extra;
extern crate colored;
extern crate git2;

mod actions;
mod helpers;

use clap::App;
use helpers::template::Template;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // Ensure the dir where templates are stored exists
    Template::create_dir_if_doesnt_exist(&Template::base_dir());

    // Unwrap is safe because clap ensures there's a value
    match matches.subcommand_name().unwrap() {
        "save" => actions::save::main(&matches),
        "load" => actions::load::main(&matches),
        "list" => actions::list::main(&matches),
        "load-git" => actions::repo::load::main(&matches),
        "remove" => actions::remove::main(&matches),
        _ => unreachable!(),
    }
}
