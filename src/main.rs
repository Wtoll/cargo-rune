mod commands;

use clap::{
    App,
    Arg,
    AppSettings
};

use clap::{
    crate_authors,
    crate_description,
    crate_version
};

use indoc::indoc;

fn main() {

    // Filter the args so that if the second item is "rune" (the program was run
    // as a cargo extension) that item is excluded and it doesn't mess everything up
    // 
    // NOTE: This does not affect running the binary on it's own, nor through `cargo run`
    // that being said if a "rune" subcommand is created (which sounds like a bad idea
    // anyways) then this will break that subcommand.
    let args = std::env::args_os()
        .enumerate()
        .filter_map(|(index, item)| {
            return if index == 1 && item == "rune" { None } else { Some(item) };
        });
    
    // Construct the application
    let mut app = construct_app();

    // Get matches for the application
    match app.try_get_matches_from_mut(args) {
        // If there are matches dispatch them to the proper place
        Ok(matches) => commands::dispatch(app, matches),
        Err(err) => err.exit()
    }
}

fn construct_app() -> App<'static> {

    // Construct a static long version string from environment variables and files
    // generated from build.rs
    let long_version = concat!(
        env!("CARGO_PKG_VERSION"), 
        " (",
        include_str!(concat!(env!("OUT_DIR"), "/commit.txt")),
        " ",
        include_str!(concat!(env!("OUT_DIR"), "/date.txt")),
        ")");

    App::new("Cargo Rune")
        .bin_name("cargo rune")
        .version(crate_version!())
        .author(crate_authors!())
        .long_version(long_version)
        .about(crate_description!())
        // TODO: Maybe look at this again, I'm guessing this is still in flux for 3.0
        .help_template(indoc!("
            {about}

            USAGE:
                {usage}

            {all-args}"))
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(commands::run::construct_app())
        .subcommand(commands::init::construct_app())
        .subcommand(App::new("version")
            .about("Gets the current cargo rune version"))
}