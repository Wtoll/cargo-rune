pub mod run;
pub mod init;

use clap::{
    ArgMatches,
    App
};

pub fn dispatch(app: App, matches: ArgMatches) {
    match matches.subcommand() {
        ("", inner) => run::execute(inner),
        ("run", inner) => run::execute(inner),
        ("init", inner) => init::execute(inner),
        ("version", _) => {
            app.write_long_version(&mut std::io::stdout()).expect("Failed to flush stdout");
            println!();
        }
        _ => {}
    }
}