use clap::{
    App, Arg, ArgMatches
};

pub mod dynlink;

pub fn construct_app() -> App<'static> {
    App::new("run")
        .about("Runs a rune project")
}

pub fn execute(matches: Option<&ArgMatches>) {
    println!("executing the run command")
}