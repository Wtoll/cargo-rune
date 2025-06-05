use clap::{Parser, Subcommand};
use clio::Input;

///
#[derive(Parser, Debug)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Cli {
    ///
    #[command(subcommand)]
    pub command: Option<Commands>,

    ///
    #[command(flatten)]
    pub run_args: Option<RunArgs>
}

///
#[derive(Debug, Subcommand)]
pub enum Commands {
    ///
    Test {
        #[arg(short, long)]
        list: bool,
    },
    ///
    Run(RunArgs),
}

///
#[derive(Debug, clap::Args)]
pub struct RunArgs {
    ///
    pub input: Input
}