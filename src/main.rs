use std::io;

extern crate clap;
use clap::App;

fn main() {
    let authors = env!("CARGO_PKG_AUTHORS").replace(':', "\n");

    let mut cli = App::new("Cargo Rune")
        .bin_name("cargo rune")
        .author(&*authors)
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(App::new("run")
            .name("run")
            .about("runs a rune script"))
        .subcommand(App::new("version")
            .name("version")
            .about("gets the current cargo rune version"));

    let command_match = cli.get_matches_mut();

    match command_match.subcommand_name() {
        Some(contents) => match contents {
            "run" => subcommands::run(),
            "version" => {
                cli.write_version(&mut io::stdout()).expect("failed to write to stdout");
                println!();
            },
            _ => {}
        },
        None => subcommands::run()
    }
}

mod subcommands;
