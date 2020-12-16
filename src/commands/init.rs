use clap::{
    App, ArgMatches
};

pub fn construct_app() -> App<'static> {
    App::new("init")
        .about("Initializes a new rune project in the current directory")
}

pub fn execute(matches: Option<&ArgMatches>) {
    println!("executing the init command");
}