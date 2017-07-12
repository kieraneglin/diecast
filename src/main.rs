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
    let action = matches.value_of("action").unwrap().to_string();

    if action == "new" {
        actions::new::main(&matches);
    } else if action == "load" {
        actions::load::main(&matches);
    }
}
