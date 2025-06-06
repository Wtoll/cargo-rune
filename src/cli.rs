use std::{env, ffi::OsString};

use clap::{Parser, Subcommand};
use clio::Input;

pub fn parse() -> Cli {
    let mut args: Vec<OsString> = env::args_os().collect();
    let args = if args.get(1).map(|arg| arg == "rune").unwrap_or_default() {
        // ["/bin/cargo-rune", "rune", ...] -> ["rune", "/bin/cargo-rune", ...] -> ["/bin/cargo-rune", ...]
        args.swap(0, 1);
        let mut args = args.into_iter();
        drop(args.next().unwrap());

        args
    } else {
        args.into_iter()
    };

    Cli::parse_from(args)
}

#[derive(Parser, Debug)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    #[command(flatten)]
    pub run_args: Option<RunArgs>
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Runs a rune script
    Run(RunArgs),
}

#[derive(Debug, clap::Args)]
pub struct RunArgs {
    /// The rune script to be run
    pub input: Input
}