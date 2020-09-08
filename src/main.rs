use std::io;
use std::ffi::OsString;

extern crate clap;
use clap::{App, Arg, AppSettings};

fn main() {
    let authors = env!("CARGO_PKG_AUTHORS").replace(':', "\n");
    let version = format!("{} ({} {})", env!("CARGO_PKG_VERSION"), build_params::version::get(), build_params::date::get());

    let mut cli = App::new("Cargo Rune")
        .bin_name("cargo rune")
        .author(&*authors)
        .version(env!("CARGO_PKG_VERSION"))
        .long_version(&*version)
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::VersionlessSubcommands)
        .arg(Arg::with_name("PATH")
            .about("the path to a rune script to run")
            .default_value("src/main.rn")
            .hide_default_value(true)
            .required(true)
            .index(1))
        .subcommand(App::new("run")
            .about("runs a rune script")
            .arg(Arg::with_name("PATH")
                .about("the path to a rune script to run")
                .default_value("src/main.rn")
                .hide_default_value(true)
                .required(true)
                .index(1)))
        .subcommand(App::new("version")
            .about("gets the current cargo rune version"));

    let command_match = if cfg!(debug_assertions) {
        get_matches_from_mut(&mut cli, std::env::args_os())
    } else {
        get_matches_from_mut(&mut cli, std::env::args_os().skip(1))
    };

    match command_match.subcommand() {
        ("", None) => subcommands::run::main(command_match.value_of("PATH").unwrap()),
        ("run", Some(sub)) => subcommands::run::main(sub.value_of("PATH").unwrap()),
        ("version", Some(_)) => {
            cli.write_long_version(&mut io::stdout()).expect("failed to write to stdout");
            println!();
        },
        (_, _) => {}
    }
}

mod subcommands;
mod build_params;

fn get_matches_from_mut<I, T>(app: &mut App, itr: I) -> clap::ArgMatches
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    app.try_get_matches_from_mut(itr)
        .unwrap_or_else(|e| {
            e.exit()
        })
}
